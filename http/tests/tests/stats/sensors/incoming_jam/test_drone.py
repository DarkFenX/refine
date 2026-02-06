
from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions


def test_general(client, consts):
    eve_sensor_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    eve_sensor_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength)
    eve_jam_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength_bonus)
    eve_jam_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength_bonus)
    eve_jam_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength_bonus)
    eve_jam_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_duration_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ecm_jam_duration)
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_jam_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.entity_ecm_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_drone_id = client.mk_eve_drone(
        attrs={
            eve_jam_radar_attr_id: 1, eve_jam_magnet_attr_id: 1, eve_jam_grav_attr_id: 1, eve_jam_ladar_attr_id: 1,
            eve_cycle_time_attr_id: 20000, eve_duration_attr_id: 5000,
            eve_optimal_attr_id: 7500, eve_radius_attr_id: 15},
        eff_ids=[eve_jam_effect_id],
        defeff_id=eve_jam_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 300})
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_sensor_ladar_attr_id: 12, eve_radius_attr_id: 32.89, eve_resist_attr_id: 0.5})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_sensor_grav_attr_id: 16.8, eve_radius_attr_id: 33, eve_resist_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_drone = api_src_fit.add_drone(
        type_id=eve_drone_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 0, 0))
    api_src_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.04166667), approx(0.01041667)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.04166667), approx(0.01041667)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.05952381), approx(0.01488095)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.05952381), approx(0.01488095)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 7547, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.05952381), approx(0.01488095)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.05952381), approx(0.01488095)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 7549, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]
