
from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions


def test_general(client, consts):
    # Lockbreaker bombs ignore explosion radius of a bomb and signature radius of a target. Tested
    # on 2025-09-07 on Thunderdome by repeatedly bombing a Stiletto, 20+ runs broke lock
    eve_sensor_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    eve_sensor_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength)
    eve_sensor_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    eve_jam_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength_bonus)
    eve_jam_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength_bonus)
    eve_jam_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength_bonus)
    eve_jam_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength_bonus)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_reactivation_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.module_reactivation_delay)
    eve_flight_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_expl_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.emp_field_range)
    eve_expl_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.aoe_cloud_size)
    eve_resist_attr_id = client.mk_eve_attr()
    eve_resist_def_attr_id = client.mk_eve_attr(id_=consts.EveAttr.remote_resistance_id)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.use_missiles,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_bomb_effect_id = client.mk_eve_effect(id_=consts.EveEffect.bomb_launching, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_cycle_time_attr_id: 10000, eve_reactivation_time_attr_id: 67500, eve_capacity_attr_id: 300},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_charge_id = client.mk_eve_item(
        attrs={
            eve_jam_radar_attr_id: 12.5, eve_jam_magnet_attr_id: 12.5,
            eve_jam_grav_attr_id: 12.5, eve_jam_ladar_attr_id: 12.5,
            eve_flight_speed_attr_id: 4000, eve_flight_time_attr_id: 7500,
            eve_mass_attr_id: 1000, eve_agility_attr_id: 0.0000251,
            eve_expl_range_attr_id: 15000, eve_expl_radius_attr_id: 400,
            eve_resist_def_attr_id: eve_resist_attr_id, eve_volume_attr_id: 75},
        eff_ids=[eve_bomb_effect_id],
        defeff_id=eve_bomb_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 20.5})
    eve_tgt_ship1_id = client.mk_eve_ship(attrs={
        eve_sensor_ladar_attr_id: 30.7, eve_radius_attr_id: 258, eve_sig_radius_attr_id: 177, eve_resist_attr_id: 0.5})
    eve_tgt_ship2_id = client.mk_eve_ship(attrs={
        eve_sensor_grav_attr_id: 39.2, eve_radius_attr_id: 263, eve_sig_radius_attr_id: 266, eve_resist_attr_id: 1})
    eve_tgt_ship3_id = client.mk_eve_ship(attrs={
        eve_sensor_radar_attr_id: 4, eve_radius_attr_id: 220, eve_sig_radius_attr_id: 175, eve_resist_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 30000, 0))
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.2035831), 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.2035831), 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.3188776), 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.3188776), 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 12700, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 12800, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.1594388), 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.1594388), 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 16700, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.1594388), 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.1594388), 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 16800, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.3188776), 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.3188776), 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 43200, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.3188776), 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.3188776), 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 43300, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.1594388), 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.1594388), 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 47200, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.1594388), 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.1594388), 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 47300, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship3_id, coordinates=(0, 30000, 0))
    # Verification - chance cannot exceed 100%
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [1, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [1, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 45000, 0))
    # Verification - at this range, bomb has 50% chance to hit target, so regardless of how strong
    # bomb jam strength is, it cannot go higher than 50%
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.5), 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.5), 0]
