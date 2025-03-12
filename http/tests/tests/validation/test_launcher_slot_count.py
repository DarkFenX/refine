from tests import approx, check_no_field
from tests.fw.api import ValOptions


def test_fail_single(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.launcher_slot_count.used == 1
    assert api_val.details.launcher_slot_count.max == 0
    assert api_val.details.launcher_slot_count.users == [api_module.id]


def test_fail_multiple_ship(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    api_module2 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.launcher_slot_count.used == 2
    assert api_val.details.launcher_slot_count.max == 1
    assert api_val.details.launcher_slot_count.users == sorted([api_module1.id, api_module2.id])


def test_fail_multiple_struct(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_struct_id = client.mk_eve_struct(attrs={eve_total_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_module1 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    api_module2 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.launcher_slot_count.used == 2
    assert api_val.details.launcher_slot_count.max == 1
    assert api_val.details.launcher_slot_count.users == sorted([api_module1.id, api_module2.id])


def test_equal(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 1})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    api_module2 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=(True, [api_module1.id])))
    assert api_val.passed is False
    assert api_val.details.launcher_slot_count.used == 2
    assert api_val.details.launcher_slot_count.max == 1
    assert api_val.details.launcher_slot_count.users == [api_module2.id]
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.launcher_slot_count.used == 2
    assert api_val.details.launcher_slot_count.max == 1
    assert api_val.details.launcher_slot_count.users == [api_module1.id]
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=(True, [api_module2.id, api_module1.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        launcher_slot_count=(True, [api_module2.id, api_other.id, api_module1.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified_total(client, consts):
    # Unrealistic scenario, but modification of total count is supported
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_total_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_mod_effect_id])
    eve_launcher_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_launcher_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    assert api_ship.update().attrs[eve_total_attr_id].extra == approx(0)
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.launcher_slot_count.used == 1
    assert api_val.details.launcher_slot_count.max == 0
    assert api_val.details.launcher_slot_count.users == [api_module.id]
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_total_attr_id].extra == approx(1)
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_fractional_total(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_ship1_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0.4})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.launcher_slot_count.used == 1
    assert api_val.details.launcher_slot_count.max == 0
    assert api_val.details.launcher_slot_count.users == [api_module.id]
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_ship(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_total_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.launcher_slot_count.used == 1
    assert api_val.details.launcher_slot_count.max is None
    assert api_val.details.launcher_slot_count.users == [api_module.id]


def test_not_loaded_user(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    # Create an item which has the effect, just to prevent it from being cleaned up
    client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_ship(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_total_attr_id: 5})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_value_total(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship()
    # Make an item to ensure that total attribute is not cleaned up
    client.mk_eve_item(attrs={eve_total_attr_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.launcher_slot_count.used == 1
    assert api_val.details.launcher_slot_count.max == 0
    assert api_val.details.launcher_slot_count.users == [api_module.id]


def test_no_attr_total(client, consts):
    # Invalid situation which shouldn't happen; just check that nothing crashes, behavior is
    # irrelevant
    eve_total_attr_id = consts.EveAttr.launcher_slots_left
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.launcher_slot_count.used == 1
    assert api_val.details.launcher_slot_count.max is None
    assert api_val.details.launcher_slot_count.users == [api_module.id]


def test_criterion_state(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.launcher_slot_count.used == 1
    assert api_val.details.launcher_slot_count.max == 0
    assert api_val.details.launcher_slot_count.users == [api_module.id]
    # Action
    api_module.change_mod(state=consts.ApiModuleState.ghost)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_mod(state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.launcher_slot_count.used == 1
    assert api_val.details.launcher_slot_count.max == 0
    assert api_val.details.launcher_slot_count.users == [api_module.id]


def test_criterion_effect(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.launcher_slot_count.used == 1
    assert api_val.details.launcher_slot_count.max == 0
    assert api_val.details.launcher_slot_count.users == [api_module.id]
    # Action
    api_module.change_mod(effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_mod(
        state=consts.ApiModuleState.offline,
        effect_modes={eve_effect_id: consts.ApiEffMode.full_compliance})
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.launcher_slot_count.used == 1
    assert api_val.details.launcher_slot_count.max == 0
    assert api_val.details.launcher_slot_count.users == [api_module.id]
    # Action
    api_module.change_mod(effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_kind(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.launcher_slots_left)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.launcher_fitted)
    eve_rig_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(launcher_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
