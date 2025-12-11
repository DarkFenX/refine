from tests import Muta, approx, check_no_field
from tests.fw.api import ValOptions


def test_add_remove(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked)
    eve_module1_id = client.mk_eve_item(attrs={eve_attr_id: 0})
    eve_module2_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module2 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_module2.id]
    # Action
    api_module2.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_state_switch(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked)
    eve_module_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(state=consts.ApiModuleState.overload)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_module.id]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_module.id]
    # Action
    api_module.change_module(state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked)
    eve_module_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_rig(type_id=eve_other_id)
    api_module1 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module2 = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=(True, [api_module1.id])))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_module2.id]
    api_val = api_fit.validate(options=ValOptions(activation_blocked=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_module1.id]
    api_val = api_fit.validate(options=ValOptions(activation_blocked=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        activation_blocked=(True, [api_module1.id, api_other.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified(client, consts):
    # This is the basic use-case, usually modules don't have it set, and external factors (e.g.
    # scrambler) set it to 1
    eve_block_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_range_attr_id = client.mk_eve_attr()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_block_attr_id)
    eve_affector_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_simple,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_range_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 1, eve_range_attr_id: 10000},
        eff_ids=[eve_affector_effect_id],
        defeff_id=eve_affector_effect_id)
    eve_affectee_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_module = api_affectee_fit.add_module(type_id=eve_affectee_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_val = api_affectee_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    api_val = api_affectee_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_module.id]
    # Action
    api_affectee_ship.change_ship(coordinates=(10001, 0, 0))
    # Verification
    api_val = api_affectee_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_affectee_ship.change_ship(coordinates=(10000, 0, 0))
    # Verification
    api_val = api_affectee_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_module.id]


def test_mutation(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_base_module_id = client.mk_eve_item(attrs={eve_attr_id: 0})
    eve_mutated_module_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_attr_id: (0, 1)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_base_module_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_module.update().attrs[eve_attr_id].extra == approx(0)
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(mutation=eve_mutator_id)
    # Verification
    assert api_module.update().attrs[eve_attr_id].extra == approx(1)
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_module.id]
    # Action
    api_module.change_module(mutation={eve_attr_id: Muta.roll_to_api(val=0)})
    # Verification
    assert api_module.update().attrs[eve_attr_id].extra == approx(0)
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(mutation={eve_attr_id: Muta.roll_to_api(val=1)})
    # Verification
    assert api_module.update().attrs[eve_attr_id].extra == approx(1)
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == [api_module.id]
    # Action
    api_module.change_module(mutation=None)
    # Verification
    assert api_module.update().attrs[eve_attr_id].extra == approx(0)
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_values(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked, def_val=0)
    eve_module1_id = client.mk_eve_item()
    eve_module2_id = client.mk_eve_item(attrs={eve_attr_id: 0})
    eve_module3_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_module4_id = client.mk_eve_item(attrs={eve_attr_id: 0.01})
    eve_module5_id = client.mk_eve_item(attrs={eve_attr_id: -0.01})
    eve_module6_id = client.mk_eve_item(attrs={eve_attr_id: -5000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active)
    api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.active)
    api_module3 = api_fit.add_module(type_id=eve_module3_id, state=consts.ApiModuleState.active)
    api_module4 = api_fit.add_module(type_id=eve_module4_id, state=consts.ApiModuleState.active)
    api_module5 = api_fit.add_module(type_id=eve_module5_id, state=consts.ApiModuleState.active)
    api_module6 = api_fit.add_module(type_id=eve_module6_id, state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is False
    assert api_val.details.activation_blocked == sorted([
        api_module3.id, api_module4.id, api_module5.id, api_module6.id])


def test_no_attr(client, consts):
    eve_attr_id = consts.EveAttr.activation_blocked
    eve_module1_id = client.mk_eve_item(attrs={eve_attr_id: 0})
    eve_module2_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active)
    api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.active)
    # Verification - validation passes if attr does not exist
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.activation_blocked)
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_kind(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.activation_blocked)
    eve_item_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_item_id, eve_attr_id: 1},
        eff_ids=[eve_autocharge_effect_id])
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_item_id)
    api_fit.set_character(type_id=eve_item_id)
    api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_item_id)
    api_fit.add_implant(type_id=eve_item_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.overload, charge_type_id=eve_item_id)
    api_fit.add_rig(type_id=eve_item_id)
    api_fit.add_service(type_id=eve_item_id, state=consts.ApiServiceState.online)
    api_fit.set_ship(type_id=eve_item_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    api_fit.set_stance(type_id=eve_item_id)
    api_fit.add_subsystem(type_id=eve_item_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_val = api_fit.validate(options=ValOptions(activation_blocked=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
