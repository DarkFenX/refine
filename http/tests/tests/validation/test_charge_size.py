from tests import approx, check_no_field
from tests.fw.api import ValOptions


def test_bundled(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge1_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_charge2_id = client.mk_eve_item(attrs={eve_size_attr_id: 3})
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module1.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module2 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module2.charge.id: (api_module2.id, 3, 2)}
    # Action
    api_module2.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_separate(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge1_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_charge2_id = client.mk_eve_item(attrs={eve_size_attr_id: 3})
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 3, 2)}
    # Action
    api_module.change_module(charge_type_id=None)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge1_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_charge2_id = client.mk_eve_item(attrs={eve_size_attr_id: 3})
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    api_module2 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    api_module3 = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=(True, [api_module2.charge.id])))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module3.charge.id: (api_module3.id, 3, 2)}
    api_val = api_fit.validate(options=ValOptions(charge_size=(True, [api_module3.charge.id])))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module2.charge.id: (api_module2.id, 3, 2)}
    api_val = api_fit.validate(options=ValOptions(charge_size=(True, [api_module2.charge.id, api_module3.charge.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(
        options=ValOptions(charge_size=(True, [api_module2.charge.id, api_other.id, api_module3.charge.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_rounding(client, consts):
    # Size isn't rounded and is compared directly
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge_id = client.mk_eve_item(attrs={eve_size_attr_id: 2.01})
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 2.01, 2)}


def test_switch_type_id_module(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_module1_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_module2_id = client.mk_eve_item(attrs={eve_size_attr_id: 3})
    eve_module3_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module1_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 2, 3)}
    # Action
    api_module.change_module(type_id=eve_module3_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 2, 3)}


def test_switch_type_id_charge(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge1_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_charge2_id = client.mk_eve_item(attrs={eve_size_attr_id: 3})
    eve_charge3_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.charge.change_charge(type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 3, 2)}
    # Action
    api_module.charge.change_charge(type_id=eve_charge3_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.charge.change_charge(type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 3, 2)}


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
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    assert api_module.update().attrs[eve_size_attr_id].extra == approx(2)
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 2, 1)}
    # Action
    api_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    assert api_module.update().attrs[eve_size_attr_id].extra == approx(2)
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_implant.remove()
    # Verification
    assert api_module.update().attrs[eve_size_attr_id].extra == approx(1)
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(charge_type_id=eve_charge1_id)
    # Verification
    assert api_module.update().attrs[eve_size_attr_id].extra == approx(1)
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
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
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    assert api_module.charge.update().attrs[eve_size_attr_id].extra == approx(1)
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 2, 1)}
    # Action
    api_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    assert api_module.charge.update().attrs[eve_size_attr_id].extra == approx(1)
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_implant.remove()
    # Verification
    assert api_module.charge.update().attrs[eve_size_attr_id].extra == approx(1)
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(charge_type_id=eve_charge1_id)
    # Verification
    assert api_module.charge.update().attrs[eve_size_attr_id].extra == approx(2)
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
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
    api_module = api_fit.add_module(type_id=eve_base_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(mutation=eve_mutator_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 1, 2)}
    # Action
    api_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_module.change_module(mutation=eve_mutator_id)
    # Action
    api_module.change_module(mutation=None)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 2, 1)}


def test_no_charge(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_non_positive(client, consts):
    # Size of 0 is actually used in EVE, small RASB charges
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge1_id = client.mk_eve_item(attrs={eve_size_attr_id: 0})
    eve_charge2_id = client.mk_eve_item(attrs={eve_size_attr_id: -3})
    eve_module1_id = client.mk_eve_item(attrs={eve_size_attr_id: 0})
    eve_module2_id = client.mk_eve_item(attrs={eve_size_attr_id: -3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module1_id, charge_type_id=eve_charge2_id)
    api_module2 = api_fit.add_module(type_id=eve_module2_id, charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {
        api_module1.charge.id: (api_module1.id, -3, 0),
        api_module2.charge.id: (api_module2.id, 0, -3)}
    # Action
    api_module1.change_module(charge_type_id=eve_charge1_id)
    api_module2.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_value_module(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge_id = client.mk_eve_item(attrs={eve_size_attr_id: 1})
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_value_charge(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge_id = client.mk_eve_item()
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, None, 2)}


def test_not_loaded_module(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_charge(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge_id = client.alloc_item_id()
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_state(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_charge_id = client.mk_eve_item(attrs={eve_size_attr_id: 1})
    eve_module_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        charge_type_id=eve_charge_id,
        state=consts.ApiModuleState.disabled)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 1, 2)}
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    api_module.charge.change_charge(state=False)
    # Verification
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is False
    assert api_val.details.charge_size == {api_module.charge.id: (api_module.id, 1, 2)}


def test_criterion_item_kind(client, consts):
    eve_size_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_size)
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_autocharge_id = client.mk_eve_item(attrs={eve_size_attr_id: 1})
    eve_booster_id = client.mk_eve_item(attrs={eve_size_attr_id: 2})
    eve_character_id = client.mk_eve_item(attrs={eve_size_attr_id: 3})
    eve_charge_id = client.mk_eve_item(attrs={eve_size_attr_id: 4})
    eve_drone_id = client.mk_eve_item(attrs={eve_size_attr_id: 5})
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_autocharge_id, eve_size_attr_id: 6},
        eff_ids=[eve_autocharge_effect_id])
    eve_fw_effect_id = client.mk_eve_item(attrs={eve_size_attr_id: 7})
    eve_implant_id = client.mk_eve_item(attrs={eve_size_attr_id: 8})
    eve_module_id = client.mk_eve_item()
    eve_rig_id = client.mk_eve_item(attrs={eve_size_attr_id: 9})
    eve_service_id = client.mk_eve_item(attrs={eve_size_attr_id: 10})
    eve_ship_id = client.mk_eve_item(attrs={eve_size_attr_id: 11})
    eve_skill_id = client.mk_eve_item(attrs={eve_size_attr_id: 12})
    eve_stance_id = client.mk_eve_item(attrs={eve_size_attr_id: 13})
    eve_subsystem_id = client.mk_eve_item(attrs={eve_size_attr_id: 14})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_booster_id)
    api_fit.set_character(type_id=eve_character_id)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload, charge_type_id=eve_charge_id)
    api_fit.add_rig(type_id=eve_rig_id)
    api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.online)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_skill(type_id=eve_skill_id, level=5)
    api_fit.set_stance(type_id=eve_stance_id)
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_val = api_fit.validate(options=ValOptions(charge_size=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
