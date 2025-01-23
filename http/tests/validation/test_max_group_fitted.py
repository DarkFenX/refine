from tests import check_no_field


def test_same_module(client, consts):
    # Simple but realistic scenario
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.offline)
    api_module2 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.offline)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_fitted])
    assert api_val.passed is False
    assert api_val.details.max_group_fitted == {eve_grp_id: [2, {api_module1.id: 1, api_module2.id: 1}]}


def test_same_rig(client, consts):
    # Simple but realistic scenario
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_rig1 = api_fit.add_rig(type_id=eve_rig_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_fitted])
    assert api_val.passed is False
    assert api_val.details.max_group_fitted == {eve_grp_id: [2, {api_rig1.id: 1, api_rig2.id: 1}]}


def test_mix(client, consts):
    # Checks in details how validation works, but uses unrealistic scenario, since modules in the
    # same EVE group have the same value of the attribute
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_module1_id = client.mk_eve_item(grp_id=eve_grp_id)
    eve_module2_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 2})
    eve_module3_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module1_id, state=consts.ApiState.offline)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id, state=consts.ApiState.offline)
    api_fit.add_mod(type_id=eve_module3_id, state=consts.ApiState.offline)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_fitted])
    assert api_val.passed is False
    assert api_val.details.max_group_fitted == {eve_grp_id: [3, {api_module2.id: 2}]}
    # Action
    api_module1.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_fitted])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_modified(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_limit_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.offline)
    api_module2 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.offline)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_fitted])
    assert api_val.passed is False
    assert api_val.details.max_group_fitted == {eve_grp_id: [2, {api_module1.id: 1, api_module2.id: 1}]}
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_fitted])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_rig.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_fitted])
    assert api_val.passed is False
    assert api_val.details.max_group_fitted == {eve_grp_id: [2, {api_module1.id: 1, api_module2.id: 1}]}


def test_mutation(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_base_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 2})
    eve_mutated_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_base_module_id, state=consts.ApiState.offline)
    api_module2 = api_fit.add_mod(type_id=eve_base_module_id, state=consts.ApiState.offline)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_fitted])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module2.change_mod(mutation=eve_mutator_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_fitted])
    assert api_val.passed is False
    assert api_val.details.max_group_fitted == {eve_grp_id: [2, {api_module2.id: 1}]}
    # Action
    api_module2.change_mod(mutation=None)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_fitted])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
