from fw import approx
from fw.api import FitStatsOptions, ValOptions


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_use_attr_id: 150})
    eve_rig2_id = client.mk_eve_rig(attrs={eve_use_attr_id: 100})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(0), approx(125))
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig2_id], failing=[eve_rig1_id])


def test_fail_multiple_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_use_attr_id: 50})
    eve_rig2_id = client.mk_eve_rig(attrs={eve_use_attr_id: 100})
    eve_rig3_id = client.mk_eve_rig(attrs={eve_use_attr_id: 75})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig1_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(50), approx(125))
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig3_id], failing=[eve_rig2_id])


def test_fail_multiple_struct(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_rig(cat_id=consts.EveItemCat.structure_module, attrs={eve_use_attr_id: 50})
    eve_rig2_id = client.mk_eve_rig(cat_id=consts.EveItemCat.structure_module, attrs={eve_use_attr_id: 100})
    eve_rig3_id = client.mk_eve_rig(cat_id=consts.EveItemCat.structure_module, attrs={eve_use_attr_id: 75})
    eve_struct_id = client.mk_eve_struct(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_fit.add_rig(type_id=eve_rig1_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(50), approx(125))
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig3_id], failing=[eve_rig2_id])


def test_equal(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig_id = client.mk_eve_rig(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(0), approx(150))
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig_id], failing=[])


def test_known_failures(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_use_attr_id: 150})
    eve_rig2_id = client.mk_eve_rig(attrs={eve_use_attr_id: 100})
    eve_rig3_id = client.mk_eve_rig(attrs={eve_use_attr_id: -10})
    eve_rig4_id = client.mk_eve_rig(attrs={eve_use_attr_id: 0})
    eve_rig5_id = client.mk_eve_rig(attrs={eve_use_attr_id: 0.5})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig2_id)
    # Verification - items which do not use the resource do not fail it
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(250), approx(125))
    api_fit.check_try_fit_items(
        val_options=ValOptions(calibration=[api_rig1.id]),
        passing=[],
        failing=[eve_rig3_id, eve_rig4_id, eve_rig2_id, eve_rig5_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(calibration=[api_rig2.id]),
        passing=[],
        failing=[eve_rig3_id, eve_rig4_id, eve_rig2_id, eve_rig5_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(calibration=[api_rig1.id, api_rig2.id]),
        passing=[eve_rig3_id, eve_rig4_id],
        failing=[eve_rig2_id, eve_rig5_id])


def test_modified_use(client, consts):
    # Calibration use is never modified, so the lib just uses unmodified attributes for faster
    # access to the attr value
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig1_id = client.mk_eve_rig(attrs={eve_use_attr_id: 20})
    eve_rig2_id = client.mk_eve_rig(attrs={eve_use_attr_id: 150})
    eve_rig3_id = client.mk_eve_rig(attrs={eve_use_attr_id: 105})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -50}, eff_ids=[eve_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig1_id)
    # Verification
    assert api_rig.update().attrs[eve_use_attr_id].modified == approx(10)
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(20), approx(125))
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig3_id], failing=[eve_rig2_id])
    # Action
    api_implant.remove()
    # Verification
    assert api_rig.update().attrs[eve_use_attr_id].modified == approx(20)
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(20), approx(125))
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig3_id], failing=[eve_rig2_id])


def test_modified_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_rig(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 120})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].modified == approx(120)
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(0), approx(120))
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[], failing=[eve_rig_id])
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].modified == approx(180)
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(0), approx(180))
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig_id], failing=[])


def test_rounding(client, consts):
    # Calibration shouldn't have its sum or individual values rounded
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_use_attr_id: 0.002})
    eve_rig2_id = client.mk_eve_rig(attrs={eve_use_attr_id: 5.2215})
    eve_rig3_id = client.mk_eve_rig(attrs={eve_use_attr_id: 5.22})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 5.223})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig1_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(0.002), approx(5.223))
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig3_id], failing=[eve_rig2_id])


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_use_attr_id: 5})
    eve_rig2_id = client.mk_eve_rig()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(0), None)
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig2_id], failing=[eve_rig1_id])


def test_not_loaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_use_attr_id: 5})
    eve_rig2_id = client.mk_eve_rig()
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(0), None)
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig2_id], failing=[eve_rig1_id])


def test_non_positive(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_use_attr_id: 0})
    eve_rig2_id = client.mk_eve_rig(attrs={eve_use_attr_id: 150})
    eve_rig3_id = client.mk_eve_rig(attrs={eve_use_attr_id: -10})
    eve_rig4_id = client.mk_eve_rig(attrs={eve_use_attr_id: 130})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig3_id)
    # Verification - items with negative use reduce total use
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(-10), approx(125))
    api_fit.check_try_fit_items(
        val_options=ValOptions(calibration=True),
        passing=[eve_rig1_id, eve_rig3_id, eve_rig4_id],
        failing=[eve_rig2_id])


def test_no_value_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_use_attr_id: 150})
    eve_rig2_id = client.mk_eve_rig()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(0), approx(125))
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig2_id], failing=[eve_rig1_id])


def test_no_value_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_use_attr_id: 150})
    eve_rig2_id = client.mk_eve_rig()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(0), approx(0))
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig2_id], failing=[eve_rig1_id])


def test_no_attr_use(client, consts):
    eve_use_attr_id = consts.EveAttr.upgrade_cost
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig_id = client.mk_eve_rig(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - users are assumed to take no resource when attribute does not exist
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig_id], failing=[])


def test_no_attr_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = consts.EveAttr.upgrade_capacity
    eve_rig1_id = client.mk_eve_rig(attrs={eve_use_attr_id: 50})
    eve_rig2_id = client.mk_eve_rig()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - when output attr does not exist, it is assumed to be 0
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig2_id], failing=[eve_rig1_id])


def test_criterion_rig_state(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_rig1_id = client.mk_eve_rig(attrs={eve_use_attr_id: 150})
    eve_rig2_id = client.mk_eve_rig(attrs={eve_use_attr_id: 100})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig1_id, state=False)
    # Verification - disabled rigs do not use the resource
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(0), approx(125))
    api_fit.check_try_fit_items(val_options=ValOptions(calibration=True), passing=[eve_rig2_id], failing=[eve_rig1_id])


def test_criterion_item_kind(client, consts):
    # Validation applies only to rigs
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_cost)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_capacity)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_booster_id = client.mk_eve_booster(attrs={eve_use_attr_id: 150})
    eve_charge_id = client.mk_eve_charge(attrs={eve_use_attr_id: 150})
    eve_drone_id = client.mk_eve_drone(attrs={eve_use_attr_id: 150})
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_use_attr_id: 150})
    eve_implant_id = client.mk_eve_implant(attrs={eve_use_attr_id: 150})
    eve_module_id = client.mk_eve_module(attrs={eve_use_attr_id: 150})
    eve_rig_id = client.mk_eve_rig(attrs={eve_use_attr_id: 150})
    eve_service_id = client.mk_eve_service(attrs={eve_use_attr_id: 150})
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_use_attr_id: 150})
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125, eve_use_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(calibration=True))
    assert api_stats.calibration.one() == (approx(0), approx(125))
    api_fit.check_try_fit_items(
        val_options=ValOptions(calibration=True),
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
