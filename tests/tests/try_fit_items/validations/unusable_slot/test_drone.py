from fw import approx
from fw.api import ValOptions


def test_fail_single(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_drone()
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_drone_slot=True),
        passing=[],
        failing=[eve_drone_id])


def test_one(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_drone()
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_drone_slot=True),
        passing=[eve_drone_id],
        failing=[])


def test_known_failures(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_drone()
    eve_fighter_id = client.mk_eve_fighter()
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    api_drone2 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_drone_slot=[api_drone1.id]),
        passing=[],
        failing=[eve_fighter_id, eve_drone_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_drone_slot=[api_drone2.id]),
        passing=[],
        failing=[eve_fighter_id, eve_drone_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_drone_slot=[api_drone1.id, api_drone2.id]),
        passing=[eve_fighter_id],
        failing=[eve_drone_id])


def test_modified_max(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_drone_id = client.mk_eve_drone()
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_char = api_fit.set_character(type_id=eve_char_id)
    # Verification
    assert api_char.update().attrs[eve_max_attr_id].modified == approx(0)
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_drone_slot=True),
        passing=[],
        failing=[eve_drone_id])
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_char.update().attrs[eve_max_attr_id].modified == approx(1)
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_drone_slot=True),
        passing=[eve_drone_id],
        failing=[])


def test_fractional_max(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_drone()
    eve_char1_id = client.mk_eve_item(attrs={eve_max_attr_id: 0.4})
    eve_char2_id = client.mk_eve_item(attrs={eve_max_attr_id: 0.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_drone_slot=True),
        passing=[],
        failing=[eve_drone_id])
    # Action
    api_fit.set_character(type_id=eve_char2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_drone_slot=True),
        passing=[eve_drone_id],
        failing=[])


def test_no_char(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_drone()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_drone_slot=True),
        passing=[],
        failing=[eve_drone_id])


def test_no_value_max(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_drone()
    eve_char_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_drone_slot=True),
        passing=[],
        failing=[eve_drone_id])


def test_no_attr_max(client, consts):
    eve_max_attr_id = consts.EveAttr.max_active_drones
    eve_drone_id = client.mk_eve_drone()
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    # Verification - when output attr does not exist, it is assumed to be 0
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_drone_slot=True),
        passing=[],
        failing=[eve_drone_id])


def test_not_loaded_char(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_drone()
    eve_char_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_drone_slot=True),
        passing=[],
        failing=[eve_drone_id])


def test_criterion_item_kind(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_booster_id = client.mk_eve_booster()
    eve_charge_id = client.mk_eve_charge()
    eve_drone_id = client.mk_eve_drone()
    eve_fighter_id = client.mk_eve_fighter()
    eve_implant_id = client.mk_eve_implant()
    eve_module_id = client.mk_eve_module()
    eve_rig_id = client.mk_eve_rig()
    eve_service_id = client.mk_eve_service()
    eve_subsystem_id = client.mk_eve_subsystem()
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(unlaunchable_drone_slot=True),
        passing=[
            eve_booster_id,
            eve_charge_id,
            eve_fighter_id,
            eve_implant_id,
            eve_module_id,
            eve_rig_id,
            eve_service_id,
            eve_subsystem_id],
        failing=[eve_drone_id])
