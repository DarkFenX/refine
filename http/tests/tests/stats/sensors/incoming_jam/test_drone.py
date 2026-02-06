
from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionInJam, StatTimeBurst, StatTimeSim


def setup_drone_test(client, consts):
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
        attrs={eve_sensor_grav_attr_id: 0.01, eve_radius_attr_id: 150, eve_resist_attr_id: 0.0001})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_sensor_ladar_attr_id: 12, eve_radius_attr_id: 32.89, eve_resist_attr_id: 0.5})
    eve_tgt_ship3_id = client.mk_eve_ship(
        attrs={eve_sensor_grav_attr_id: 16.8, eve_radius_attr_id: 33, eve_resist_attr_id: 1})
    client.create_sources()
    return eve_drone_id, eve_src_ship_id, eve_tgt_ship1_id, eve_tgt_ship2_id, eve_tgt_ship3_id


def test_projection_and_resists(client, consts):
    eve_drone_id, eve_src_ship_id, eve_tgt_ship1_id, eve_tgt_ship2_id, eve_tgt_ship3_id = setup_drone_test(
        client=client, consts=consts)
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_drone = api_src_fit.add_drone(
        type_id=eve_drone_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 0, 0))
    api_src_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification - resist is above immunity threshold, can't jam
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.04166667), approx(0.01041667)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.04166667), approx(0.01041667)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship3_id)
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


def test_time(client, consts):
    eve_drone_id, eve_src_ship_id, eve_tgt_ship1_id, eve_tgt_ship2_id, _ = setup_drone_test(
        client=client, consts=consts)
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_drone = api_src_fit.add_drone(
        type_id=eve_drone_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship2_id, coordinates=(0, 0, 0))
    api_src_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification - burst stats (first cycle)
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeBurst())])))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.04166667), approx(0.01041667)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeBurst())])))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.04166667), approx(0.01041667)]
    # Sim stats without time - loop stats are exposed, any chance higher than 0% is exposed as 100%
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=None))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [1, approx(0.01041667)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=None))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [1, approx(0.01041667)]
    # Sim with time 1 second into jam effect
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=1))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.04166667), approx(0.04166667)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=1))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.04166667), approx(0.04166667)]
    # Sim with time 4 seconds into jam effect
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=4))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.04166667), approx(0.04166667)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=4))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.04166667), approx(0.04166667)]
    # Sim with time which covers first cycle completely, but does not reach the second one
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=19))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.04166667), approx(0.01096491)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=19))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.04166667), approx(0.01096491)]
    # Sim with time which covers first cycle completely, and 1 second of the second one
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=21))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.08159722), approx(0.01190476)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=21))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.08159722), approx(0.01190476)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship1_id)
    # Verification - when chance to jam is 0%, loop doesn't make it 100%
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=None))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=None))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]
