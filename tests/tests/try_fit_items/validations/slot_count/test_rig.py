from fw import approx
from fw.api import FitStatsOptions, ValOptions


def test_fail_single(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_rig()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (0, 0)
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[], failing=[eve_rig_id])


def test_fail_multiple_ship(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_rig()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (1, 1)
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[], failing=[eve_rig_id])


def test_fail_multiple_struct(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_rig()
    eve_struct_id = client.mk_eve_struct(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (1, 1)
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[], failing=[eve_rig_id])


def test_equal(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_rig()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (0, 1)
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[eve_rig_id], failing=[])


def test_known_failures(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_rig()
    eve_module_id = client.mk_eve_module()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (2, 1)
    api_fit.check_try_fit_items(
        val_options=ValOptions(rig_slot_count=[api_rig1.id]),
        passing=[],
        failing=[eve_module_id, eve_rig_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(rig_slot_count=[api_rig2.id]),
        passing=[],
        failing=[eve_module_id, eve_rig_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(rig_slot_count=[api_rig1.id, api_rig2.id]),
        passing=[eve_module_id],
        failing=[eve_rig_id])


def test_modified_max(client, consts):
    # Unrealistic scenario, but modification of max count is supported
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_rig_id = client.mk_eve_rig()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].modified == approx(0)
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (0, 0)
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[], failing=[eve_rig_id])
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].modified == approx(1)
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (0, 1)
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[eve_rig_id], failing=[])


def test_fractional_max(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_rig()
    eve_ship1_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0.4})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (0, 0)
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[], failing=[eve_rig_id])
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (0, 1)
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[eve_rig_id], failing=[])


def test_no_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_rig()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (0, None)
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[], failing=[eve_rig_id])


def test_no_value_max(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_rig()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (0, 0)
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[], failing=[eve_rig_id])


def test_no_attr_max(client, consts):
    eve_max_attr_id = consts.EveAttr.upgrade_slots_left
    eve_rig_id = client.mk_eve_rig()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - when output attr does not exist, it is assumed to be 0
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[], failing=[eve_rig_id])


def test_not_loaded_user(client, consts):
    # Not loaded rigs still take slot
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 1})
    eve_rig1_id = client.alloc_item_id()
    eve_rig2_id = client.mk_eve_rig()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig1_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (1, 1)
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[], failing=[eve_rig2_id])


def test_not_loaded_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_rig()
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (0, None)
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[], failing=[eve_rig_id])


def test_criterion_rig_state(client, consts):
    # Slot is taken even when rig is disabled
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_rig()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (1, 1)
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[], failing=[eve_rig_id])
    # Action
    api_rig.change_rig(state=False)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (1, 1)
    api_fit.check_try_fit_items(val_options=ValOptions(rig_slot_count=True), passing=[], failing=[eve_rig_id])


def test_criterion_item_kind(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
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
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rig_slots=True))
    assert api_stats.rig_slots.one() == (0, 0)
    api_fit.check_try_fit_items(
        val_options=ValOptions(rig_slot_count=True),
        passing=[
            eve_booster_id,
            eve_charge_id,
            eve_drone_id,
            eve_fighter_id,
            eve_implant_id,
            eve_module_id,
            eve_service_id,
            eve_subsystem_id],
        failing=[eve_rig_id])
