from fw import Muta, approx
from fw.api import FitStatsOptions, ValOptions


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_drone(attrs={eve_use_attr_id: 150})
    eve_drone2_id = client.mk_eve_drone(attrs={eve_use_attr_id: 100})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(0), approx(125))
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=True),
        passing=[eve_drone2_id],
        failing=[eve_drone1_id])


def test_fail_multiple_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_drone(attrs={eve_use_attr_id: 50})
    eve_drone2_id = client.mk_eve_drone(attrs={eve_use_attr_id: 100})
    eve_drone3_id = client.mk_eve_drone(attrs={eve_use_attr_id: 75})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone1_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(50), approx(125))
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=True),
        passing=[eve_drone3_id],
        failing=[eve_drone2_id])


def test_fail_multiple_struct(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_drone(attrs={eve_use_attr_id: 50})
    eve_drone2_id = client.mk_eve_drone(attrs={eve_use_attr_id: 100})
    eve_drone3_id = client.mk_eve_drone(attrs={eve_use_attr_id: 75})
    eve_struct_id = client.mk_eve_struct(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_fit.add_drone(type_id=eve_drone1_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(50), approx(125))
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=True),
        passing=[eve_drone3_id],
        failing=[eve_drone2_id])


def test_equal(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone_id = client.mk_eve_drone(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(0), approx(150))
    api_fit.check_try_fit_items(val_options=ValOptions(drone_bay_volume=True), passing=[eve_drone_id], failing=[])


def test_known_failures(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_drone(attrs={eve_use_attr_id: 150})
    eve_drone2_id = client.mk_eve_drone(attrs={eve_use_attr_id: 100})
    eve_drone3_id = client.mk_eve_drone(attrs={eve_use_attr_id: -10})
    eve_drone4_id = client.mk_eve_drone(attrs={eve_use_attr_id: 0})
    eve_drone5_id = client.mk_eve_drone(attrs={eve_use_attr_id: 0.5})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id)
    # Verification - items which do not use the resource do not fail it
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(250), approx(125))
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=[api_drone1.id]),
        passing=[],
        failing=[eve_drone3_id, eve_drone4_id, eve_drone2_id, eve_drone5_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=[api_drone2.id]),
        passing=[],
        failing=[eve_drone3_id, eve_drone4_id, eve_drone2_id, eve_drone5_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=[api_drone1.id, api_drone2.id]),
        passing=[eve_drone3_id, eve_drone4_id],
        failing=[eve_drone2_id, eve_drone5_id])


def test_modified_use(client, consts):
    # Drone volume is never modified, so the lib just uses unmodified attributes for faster access
    # to the attr value
    eve_skill_id = client.mk_eve_item()
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_drone1_id = client.mk_eve_drone(attrs={eve_use_attr_id: 20}, srqs={eve_skill_id: 1})
    eve_drone2_id = client.mk_eve_drone(attrs={eve_use_attr_id: 150}, srqs={eve_skill_id: 1})
    eve_drone3_id = client.mk_eve_drone(attrs={eve_use_attr_id: 105}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone1_id)
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].modified == approx(10)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(20), approx(125))
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=True),
        passing=[eve_drone3_id],
        failing=[eve_drone2_id])
    # Action
    api_implant.remove()
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].modified == approx(20)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(20), approx(125))
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=True),
        passing=[eve_drone3_id],
        failing=[eve_drone2_id])


def test_modified_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_drone_id = client.mk_eve_drone(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 120})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].modified == approx(120)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(0), approx(120))
    api_fit.check_try_fit_items(val_options=ValOptions(drone_bay_volume=True), passing=[], failing=[eve_drone_id])
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].modified == approx(180)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(0), approx(180))
    api_fit.check_try_fit_items(val_options=ValOptions(drone_bay_volume=True), passing=[eve_drone_id], failing=[])


def test_mutation_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_base_drone_id = client.mk_eve_drone(attrs={eve_use_attr_id: 120})
    eve_mutated_drone_id = client.mk_eve_drone(attrs={eve_use_attr_id: 130})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_drone_id], eve_mutated_drone_id)],
        attrs={eve_use_attr_id: (0.8, 1.2)})
    eve_drone1_id = client.mk_eve_drone(attrs={eve_use_attr_id: 70})
    eve_drone2_id = client.mk_eve_drone(attrs={eve_use_attr_id: 71})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(
        type_id=eve_base_drone_id,
        mutation=(eve_mutator_id, {eve_use_attr_id: Muta.roll_to_api(val=0.8)}))
    # Verification - rolled base attribute value is in attributes, but value from mutated drone type
    # is used for stats/validation
    assert api_drone.update().attrs[eve_use_attr_id].modified == approx(134.4)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(130), approx(200))
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=True),
        passing=[eve_drone1_id],
        failing=[eve_drone2_id])


def test_rounding(client, consts):
    # Volume/capacity shouldn't have its sum or individual values rounded
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_drone(attrs={eve_use_attr_id: 0.002})
    eve_drone2_id = client.mk_eve_drone(attrs={eve_use_attr_id: 5.2215})
    eve_drone3_id = client.mk_eve_drone(attrs={eve_use_attr_id: 5.22})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 5.223})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone1_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(0.002), approx(5.223))
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=True),
        passing=[eve_drone3_id],
        failing=[eve_drone2_id])


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_drone(attrs={eve_use_attr_id: 5})
    eve_drone2_id = client.mk_eve_drone(attrs={eve_use_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(0), None)
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=True),
        passing=[eve_drone2_id],
        failing=[eve_drone1_id])


def test_no_attr_use(client, consts):
    eve_use_attr_id = consts.EveAttr.volume
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone_id = client.mk_eve_drone(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - users are assumed to take no resource when attribute does not exist
    api_fit.check_try_fit_items(val_options=ValOptions(drone_bay_volume=True), passing=[eve_drone_id], failing=[])


def test_no_attr_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = consts.EveAttr.drone_capacity
    eve_drone1_id = client.mk_eve_drone(attrs={eve_use_attr_id: 50})
    eve_drone2_id = client.mk_eve_drone(attrs={eve_use_attr_id: 0})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification - when output attr does not exist, it is assumed to be 0
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=True),
        passing=[eve_drone2_id],
        failing=[eve_drone1_id])


def test_not_loaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_drone(attrs={eve_use_attr_id: 5})
    eve_drone2_id = client.mk_eve_drone(attrs={eve_use_attr_id: 0})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(0), None)
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=True),
        passing=[eve_drone2_id],
        failing=[eve_drone1_id])


def test_non_positive(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_drone(attrs={eve_use_attr_id: 0})
    eve_drone2_id = client.mk_eve_drone(attrs={eve_use_attr_id: 150})
    eve_drone3_id = client.mk_eve_drone(attrs={eve_use_attr_id: -10})
    eve_drone4_id = client.mk_eve_drone(attrs={eve_use_attr_id: 130})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone3_id)
    # Verification - items with negative and 0 use do not affect total use
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(0), approx(125))
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=True),
        passing=[eve_drone1_id, eve_drone3_id],
        failing=[eve_drone2_id, eve_drone4_id])


def test_no_value_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_drone(attrs={eve_use_attr_id: 150})
    eve_drone2_id = client.mk_eve_drone()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(0), approx(125))
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=True),
        passing=[eve_drone2_id],
        failing=[eve_drone1_id])


def test_no_value_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_drone(attrs={eve_use_attr_id: 150})
    eve_drone2_id = client.mk_eve_drone(attrs={eve_use_attr_id: 0})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(0), approx(0))
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=True),
        passing=[eve_drone2_id],
        failing=[eve_drone1_id])


def test_criterion_item_kind(client, consts):
    # Validation applies only to drones
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
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
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bay_volume=True))
    assert api_stats.drone_bay_volume.one() == (approx(0), approx(125))
    api_fit.check_try_fit_items(
        val_options=ValOptions(drone_bay_volume=True),
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
