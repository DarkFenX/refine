
from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions


def test_stacking(client, consts):
    eve_sensor_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    eve_sensor_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    eve_jam_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength_bonus)
    eve_jam_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength_bonus)
    eve_jam_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength_bonus)
    eve_jam_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_duration_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ecm_jam_duration)
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_jam_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.remote_ecm_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_jam_radar_attr_id: 12, eve_jam_magnet_attr_id: 3, eve_jam_grav_attr_id: 3, eve_jam_ladar_attr_id: 3,
            eve_cycle_time_attr_id: 20000, eve_optimal_attr_id: 50000, eve_falloff_attr_id: 25000},
        eff_ids=[eve_jam_effect_id],
        defeff_id=eve_jam_effect_id)
    eve_drone_id = client.mk_eve_drone(
        attrs={
            eve_jam_radar_attr_id: 1, eve_jam_magnet_attr_id: 1, eve_jam_grav_attr_id: 1, eve_jam_ladar_attr_id: 1,
            eve_cycle_time_attr_id: 20000, eve_duration_attr_id: 5000,
            eve_optimal_attr_id: 7500, eve_radius_attr_id: 15},
        eff_ids=[eve_jam_effect_id],
        defeff_id=eve_jam_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 300})
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_sensor_grav_attr_id: 25, eve_radius_attr_id: 150, eve_resist_attr_id: 0.5})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_sensor_radar_attr_id: 22, eve_radius_attr_id: 150, eve_resist_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_drone = api_src_fit.add_drone(
        type_id=eve_drone_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 0, 0))
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    api_src_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.0788), approx(0.0788)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.0788), approx(0.0788)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.5661157), approx(0.5661157)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.5661157), approx(0.5661157)]


def test_sensors_zero(client, consts):
    eve_sensor_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    eve_sensor_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength)
    eve_sensor_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    eve_sensor_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength)
    eve_jam_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength_bonus)
    eve_jam_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength_bonus)
    eve_jam_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength_bonus)
    eve_jam_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_duration_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ecm_jam_duration)
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_jam_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.entity_ecm_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id)
    eve_drone_id = client.mk_eve_drone(
        attrs={
            eve_jam_radar_attr_id: 1, eve_jam_magnet_attr_id: 1, eve_jam_grav_attr_id: 1, eve_jam_ladar_attr_id: 1,
            eve_cycle_time_attr_id: 20000, eve_duration_attr_id: 5000, eve_optimal_attr_id: 7500},
        eff_ids=[eve_jam_effect_id],
        defeff_id=eve_jam_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 300})
    eve_tgt_ship_id = client.mk_eve_ship(attrs={
        eve_sensor_radar_attr_id: 0, eve_sensor_magnet_attr_id: 0,
        eve_sensor_grav_attr_id: 0, eve_sensor_ladar_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id)
    api_src_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_src_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [1, approx(0.25)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [1, approx(0.25)]


def test_both_zero(client, consts):
    eve_sensor_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    eve_sensor_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength)
    eve_sensor_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    eve_sensor_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength)
    eve_jam_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength_bonus)
    eve_jam_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength_bonus)
    eve_jam_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength_bonus)
    eve_jam_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_duration_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ecm_jam_duration)
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_jam_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.entity_ecm_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id)
    eve_drone_id = client.mk_eve_drone(
        attrs={
            eve_jam_radar_attr_id: 0, eve_jam_magnet_attr_id: 0, eve_jam_grav_attr_id: 0, eve_jam_ladar_attr_id: 0,
            eve_cycle_time_attr_id: 20000, eve_duration_attr_id: 5000, eve_optimal_attr_id: 7500},
        eff_ids=[eve_jam_effect_id],
        defeff_id=eve_jam_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 300})
    eve_tgt_ship_id = client.mk_eve_ship(attrs={
        eve_sensor_radar_attr_id: 0, eve_sensor_magnet_attr_id: 0,
        eve_sensor_grav_attr_id: 0, eve_sensor_ladar_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id)
    api_src_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_src_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]


def test_cycle_time_zero(client, consts):
    eve_sensor_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    eve_jam_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength_bonus)
    eve_jam_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength_bonus)
    eve_jam_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength_bonus)
    eve_jam_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_jam_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.remote_ecm_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_jam_radar_attr_id: 12, eve_jam_magnet_attr_id: 3, eve_jam_grav_attr_id: 3, eve_jam_ladar_attr_id: 3,
            eve_cycle_time_attr_id: 0, eve_optimal_attr_id: 50000, eve_falloff_attr_id: 25000},
        eff_ids=[eve_jam_effect_id],
        defeff_id=eve_jam_effect_id)
    eve_tgt_ship_id = client.mk_eve_ship(attrs={eve_sensor_grav_attr_id: 25, eve_resist_attr_id: 0.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification - stats depend on cycle info, so can't be fetched if it's not available
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]


def test_cycle_time_undefined(client, consts):
    eve_sensor_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    eve_jam_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength_bonus)
    eve_jam_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength_bonus)
    eve_jam_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength_bonus)
    eve_jam_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_jam_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.remote_ecm_falloff,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_jam_radar_attr_id: 12, eve_jam_magnet_attr_id: 3, eve_jam_grav_attr_id: 3, eve_jam_ladar_attr_id: 3,
            eve_cycle_time_attr_id: 20000, eve_optimal_attr_id: 50000, eve_falloff_attr_id: 25000},
        eff_ids=[eve_jam_effect_id],
        defeff_id=eve_jam_effect_id)
    eve_tgt_ship_id = client.mk_eve_ship(attrs={eve_sensor_grav_attr_id: 25, eve_resist_attr_id: 0.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification - stats depend on cycle info, so can't be fetched if it's not available
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]


def test_not_loaded(client, consts):
    eve_jam_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength_bonus)
    eve_jam_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength_bonus)
    eve_jam_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength_bonus)
    eve_jam_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_jam_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.remote_ecm_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_jam_radar_attr_id: 12, eve_jam_magnet_attr_id: 3, eve_jam_grav_attr_id: 3, eve_jam_ladar_attr_id: 3,
            eve_cycle_time_attr_id: 20000, eve_optimal_attr_id: 50000, eve_falloff_attr_id: 25000},
        eff_ids=[eve_jam_effect_id],
        defeff_id=eve_jam_effect_id)
    eve_tgt_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam is None
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam is None
