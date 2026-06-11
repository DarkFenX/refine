
from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionInJam, StatTimeBurst, StatTimeSim


def setup_burst_projector_test(client, consts):
    eve_sensor_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    eve_sensor_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    eve_jam_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength_bonus)
    eve_jam_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength_bonus)
    eve_jam_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength_bonus)
    eve_jam_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_delay_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_warning_duration)
    eve_aoe_duration_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_aoe_duration)
    eve_aoe_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_aoe_range)
    eve_optimal_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_range)
    eve_resist_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_jam_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_aoe_ecm,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_jam_radar_attr_id: 5, eve_jam_magnet_attr_id: 5, eve_jam_grav_attr_id: 5, eve_jam_ladar_attr_id: 5,
            eve_cycle_time_attr_id: 168750, eve_delay_attr_id: 10000, eve_aoe_duration_attr_id: 40000,
            eve_optimal_attr_id: 500000, eve_aoe_range_attr_id: 10000},
        eff_ids=[eve_jam_effect_id],
        defeff_id=eve_jam_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 4032})
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_sensor_radar_attr_id: 0.01, eve_radius_attr_id: 150, eve_resist_attr_id: 0.0001})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_sensor_grav_attr_id: 50, eve_radius_attr_id: 150, eve_resist_attr_id: 0.5})
    eve_tgt_ship3_id = client.mk_eve_ship(
        attrs={eve_sensor_radar_attr_id: 35, eve_radius_attr_id: 150, eve_resist_attr_id: 1})
    client.create_sources()
    return eve_module_id, eve_src_ship_id, eve_tgt_ship1_id, eve_tgt_ship2_id, eve_tgt_ship3_id


def test_projection_range_and_resists(client, consts):
    eve_module_id, eve_src_ship_id, eve_tgt_ship1_id, eve_tgt_ship2_id, eve_tgt_ship3_id = setup_burst_projector_test(
        client=client, consts=consts)
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 0, 0))
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification - resist is above immunity threshold, can't jam
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.05), approx(0.01185185)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.05), approx(0.01185185)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship3_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.1428571), approx(0.03386243)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.1428571), approx(0.03386243)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 510149, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.1428571), approx(0.03386243)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.1428571), approx(0.03386243)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 510151, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]


def test_time(client, consts):
    eve_module_id, eve_src_ship_id, eve_tgt_ship1_id, eve_tgt_ship2_id, _ = setup_burst_projector_test(
        client=client, consts=consts)
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship2_id, coordinates=(0, 0, 0))
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification - burst stats (first cycle)
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeBurst())])))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.05), approx(0.01185185)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeBurst())])))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.05), approx(0.01185185)]
    # Sim stats without time - loop stats are exposed, any chance higher than 0% is exposed as 100%
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=None))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [1, approx(0.01185185)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=None))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [1, approx(0.01185185)]
    # Sim with time before actual jam effect starts
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=9))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=9))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]
    # Sim with time 1 second into jam effect
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=11))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.05), approx(0.004545455)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=11))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.05), approx(0.004545455)]
    # Sim with time 39 seconds into jam effect
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=49))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.05), approx(0.03979592)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=49))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.05), approx(0.03979592)]
    # Sim with time which covers first cycle completely, but does not reach the second one
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=178))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.05), approx(0.01123596)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=178))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.05), approx(0.01123596)]
    # Sim with time which covers first cycle completely, and 0.25 seconds of the second one
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=179))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.0975), approx(0.01124302)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=179))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.0975), approx(0.01124302)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship1_id)
    # Verification - when chance to jam is 0%, loop doesn't make it 100%
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=None))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=None))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]
