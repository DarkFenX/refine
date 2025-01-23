from tests import approx, check_no_field


def test_same(client, consts):
    # Simple but realistic scenario
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_active)
    eve_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_module2 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is False
    assert api_val.details.max_group_active == {eve_grp_id: [2, {api_module1.id: 1, api_module2.id: 1}]}


def test_mix(client, consts):
    # Checks in details how validation works, but uses unrealistic scenario, since modules in the
    # same EVE group have the same value of the attribute
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_active)
    eve_module1_id = client.mk_eve_item(grp_id=eve_grp_id)
    eve_module2_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 2})
    eve_module3_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module1_id, state=consts.ApiState.active)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id, state=consts.ApiState.active)
    api_fit.add_mod(type_id=eve_module3_id, state=consts.ApiState.active)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is False
    assert api_val.details.max_group_active == {eve_grp_id: [3, {api_module2.id: 2}]}
    # Action
    api_module1.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_modified(client, consts):
    # Check rounding in this case too
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_active)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_limit_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_module1_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 0.6})
    eve_module2_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1.4})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_mod(type_id=eve_module1_id, state=consts.ApiState.active)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id, state=consts.ApiState.active)
    # Verification
    assert api_module1.update().attrs[eve_limit_attr_id].extra == approx(0.6)
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(1.4)
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is False
    assert api_val.details.max_group_active == {eve_grp_id: [2, {api_module1.id: 1, api_module2.id: 1}]}
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    assert api_module1.update().attrs[eve_limit_attr_id].extra == approx(1.6)
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(2.4)
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_rig.remove()
    # Verification
    assert api_module1.update().attrs[eve_limit_attr_id].extra == approx(0.6)
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(1.4)
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is False
    assert api_val.details.max_group_active == {eve_grp_id: [2, {api_module1.id: 1, api_module2.id: 1}]}


def test_mutation_limit_priority(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_active)
    eve_base_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 2})
    eve_mutated_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_base_module_id, state=consts.ApiState.active)
    api_module2 = api_fit.add_mod(type_id=eve_base_module_id, state=consts.ApiState.active)
    # Verification
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(2)
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module2.change_mod(mutation=eve_mutator_id)
    # Verification
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(1)
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is False
    assert api_val.details.max_group_active == {eve_grp_id: [2, {api_module2.id: 1}]}
    # Action
    api_module2.change_mod(mutation=None)
    # Verification
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(2)
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_mutation_limit_inheritance(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_active)
    eve_base_module_id = client.mk_eve_item(grp_id=eve_grp1_id, attrs={eve_limit_attr_id: 1})
    eve_mutated_module_id = client.mk_eve_item(grp_id=eve_grp2_id)
    eve_other_module_id = client.mk_eve_item(grp_id=eve_grp2_id)
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_other_module_id, state=consts.ApiState.active)
    api_module2 = api_fit.add_mod(type_id=eve_base_module_id, state=consts.ApiState.active)
    # Verification
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(1)
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module2.change_mod(mutation=eve_mutator_id)
    # Verification
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(1)
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is False
    assert api_val.details.max_group_active == {eve_grp2_id: [2, {api_module2.id: 1}]}
    # Action
    api_module2.change_mod(mutation=None)
    # Verification
    assert api_module2.update().attrs[eve_limit_attr_id].extra == approx(1)
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_unloaded(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_active)
    eve_module_id = client.alloc_item_id()
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_limit_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_no_attr(client, consts):
    # Check rounding in this case too
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = consts.EveAttr.max_group_active
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_limit_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_module1_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 0.6})
    eve_module2_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1.4})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_mod(type_id=eve_module1_id, state=consts.ApiState.active)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id, state=consts.ApiState.active)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is False
    assert api_val.details.max_group_active == {eve_grp_id: [2, {api_module1.id: 1, api_module2.id: 1}]}
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification - due to no attribute, the lib can't calculate modified value
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is False
    assert api_val.details.max_group_active == {eve_grp_id: [2, {api_module1.id: 1, api_module2.id: 1}]}


def test_criterion_state(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_active)
    eve_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_module2 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is False
    assert api_val.details.max_group_active == {eve_grp_id: [2, {api_module1.id: 1, api_module2.id: 1}]}
    # Action
    api_module2.change_mod(state=consts.ApiState.online)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module2.change_mod(state=consts.ApiState.active)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is False
    assert api_val.details.max_group_active == {eve_grp_id: [2, {api_module1.id: 1, api_module2.id: 1}]}


def test_criterion_item_type(client, consts):
    # Validation applies only to modules and rigs
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_active)
    eve_drone_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.active)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.active)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_active])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
