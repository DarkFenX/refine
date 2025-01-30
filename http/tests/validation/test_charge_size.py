from tests import approx, check_no_field


def test_bundled(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge1_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_charge2_id = client.mk_eve_item(attrs={eve_size_attr_id: 3})
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module1.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module2 = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module2.charge.id: (api_module2.id, 3, 2)}
    # Action
    api_module2.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_separate(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge1_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_charge2_id = client.mk_eve_item(attrs={eve_size_attr_id: 3})
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module.change_mod(charge=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module.change_mod(charge=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 3, 2)}
    # Action
    api_module.change_mod(charge=None)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_rounding(client, consts):
    # Size isn't rounded and is compared directly
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge_id = client.mk_eve_item(attrs={eve_size_attr_id: 2.01})
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 2.01, 2)}


def test_modified_module(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge1_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_charge2_id = client.mk_eve_item(attrs={eve_size_attr_id: 1})
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 1})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_size_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 2}, eff_ids=[eve_effect_id])
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship)
    api_module = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    assert api_module.update().attrs[eve_size_attr_id].extra == approx(2)
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 2, 1)}
    # Action
    api_module.change_mod(charge=eve_charge2_id)
    # Verification
    assert api_module.update().attrs[eve_size_attr_id].extra == approx(2)
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_implant.remove()
    # Verification
    assert api_module.update().attrs[eve_size_attr_id].extra == approx(1)
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module.change_mod(charge=eve_charge1_id)
    # Verification
    assert api_module.update().attrs[eve_size_attr_id].extra == approx(1)
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 2, 1)}


def test_modified_charge(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge1_id = client.mk_eve_item(attrs={eve_size_attr_id: 2}, srqs={eve_skill_id: 1})
    eve_charge2_id = client.mk_eve_item(attrs={eve_size_attr_id: 1}, srqs={eve_skill_id: 1})
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 1})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_size_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    assert api_module.charge.update().attrs[eve_size_attr_id].extra == approx(1)
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 2, 1)}
    # Action
    api_module.change_mod(charge=eve_charge2_id)
    # Verification
    assert api_module.charge.update().attrs[eve_size_attr_id].extra == approx(1)
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_implant.remove()
    # Verification
    assert api_module.charge.update().attrs[eve_size_attr_id].extra == approx(1)
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module.change_mod(charge=eve_charge1_id)
    # Verification
    assert api_module.charge.update().attrs[eve_size_attr_id].extra == approx(2)
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 2, 1)}


def test_mutation_module(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge1_id = client.mk_eve_item(attrs={eve_size_attr_id: 1})
    eve_charge2_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_base_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 1})
    eve_mutated_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_base_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module.change_mod(mutation=eve_mutator_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 1, 2)}
    # Action
    api_module.change_mod(charge=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    api_module.change_mod(mutation=eve_mutator_id)
    # Action
    api_module.change_mod(mutation=None)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 2, 1)}


def test_no_charge(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_not_loaded_module(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_not_loaded_charge(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, None, 2)}


def test_no_attr(client, consts):
    eve_size_attr_id = consts.EveAttr.charge_size
    eve_charge_id = client.mk_eve_item(attrs={eve_size_attr_id: 1})
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 1, 0)}


def test_state(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge_id = client.mk_eve_item(attrs={eve_size_attr_id: 1})
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge_id, state=consts.ApiState.ghost)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 1, 2)}
    # Action
    api_module.change_mod(state=consts.ApiState.online)
    api_module.charge.change_charge(state=False)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 1, 2)}


def test_autocharge_fighter(client, consts):
    # Autocharges / fighters are not subject for the validation
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_charge_id = client.mk_eve_item(attrs={eve_size_attr_id: 1})
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_charge_id, eve_size_attr_id: 2},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_val = api_fit.validate(include=[consts.ApiValType.charge_size])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
