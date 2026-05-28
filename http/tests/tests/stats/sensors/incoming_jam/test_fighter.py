
from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionInJam, StatTimeBurst, StatTimeSim


def setup_fighter_test(client, consts):
    eve_sensor_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    eve_sensor_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength)
    eve_jam_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_strength_radar)
    eve_jam_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_strength_magnetometric)
    eve_jam_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_strength_gravimetric)
    eve_jam_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_strength_ladar)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_jam_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_ecm,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_abil_id = client.mk_eve_abil(id_=consts.EveAbil.ecm)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={
            eve_jam_radar_attr_id: 3.3, eve_jam_magnet_attr_id: 3.3,
            eve_jam_grav_attr_id: 3.3, eve_jam_ladar_attr_id: 3.3,
            eve_cycle_time_attr_id: 10000, eve_max_count_attr_id: 3,
            eve_optimal_attr_id: 9375, eve_falloff_attr_id: 8400,
            eve_radius_attr_id: 35},
        eff_ids=[eve_jam_effect_id],
        defeff_id=eve_jam_effect_id,
        abils=[client.mk_eve_item_abil(id_=eve_abil_id)])
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 300})
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_sensor_grav_attr_id: 0.01, eve_radius_attr_id: 150, eve_resist_attr_id: 0.0001})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_sensor_ladar_attr_id: 12, eve_radius_attr_id: 32.89, eve_resist_attr_id: 0.5})
    eve_tgt_ship3_id = client.mk_eve_ship(
        attrs={eve_sensor_grav_attr_id: 16.8, eve_radius_attr_id: 33, eve_resist_attr_id: 1})
    client.create_sources()
    return eve_fighter_id, eve_src_ship_id, eve_tgt_ship1_id, eve_tgt_ship2_id, eve_tgt_ship3_id


def test_projection_range_and_resists(client, consts):
    eve_fighter_id, eve_src_ship_id, eve_tgt_ship1_id, eve_tgt_ship2_id, eve_tgt_ship3_id = setup_fighter_test(
        client=client, consts=consts)
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_fighter = api_src_fit.add_fighter(
        type_id=eve_fighter_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 0, 0))
    api_src_fighter.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification - resist is above immunity threshold, can't jam
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.4125), approx(0.4125)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.4125), approx(0.4125)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship3_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.5892857), approx(0.5892857)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.5892857), approx(0.5892857)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 9443, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.5892857), approx(0.5892857)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.5892857), approx(0.5892857)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 17843, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.2946429), approx(0.2946429)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.2946429), approx(0.2946429)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 26243, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.03683036), approx(0.03683036)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.03683036), approx(0.03683036)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 34642, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.001151519), approx(0.001151519)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.001151519), approx(0.001151519)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 34644, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]


def test_time(client, consts):
    eve_fighter_id, eve_src_ship_id, eve_tgt_ship1_id, eve_tgt_ship2_id, _ = setup_fighter_test(
        client=client, consts=consts)
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_fighter = api_src_fit.add_fighter(
        type_id=eve_fighter_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship2_id, coordinates=(0, 0, 0))
    api_src_fighter.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification - burst stats (first cycle)
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeBurst())])))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.4125), approx(0.4125)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeBurst())])))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.4125), approx(0.4125)]
    # Sim stats without time - loop stats are exposed, any chance higher than 0% is exposed as 100%
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=None))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [1, approx(0.4125)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=None))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [1, approx(0.4125)]
    # Sim with time 1 second into jam effect
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=1))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.4125), approx(0.4125)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=1))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.4125), approx(0.4125)]
    # Sim with time just before first cycle is finished
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=9))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.4125), approx(0.4125)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=9))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.4125), approx(0.4125)]
    # Sim with time just after second cycle has started
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=11))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.6548438), approx(0.4125)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=11))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.6548438), approx(0.4125)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship1_id)
    # Verification - when chance to jam is 0%, loop doesn't make it 100%
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=None))])))
    assert api_tgt_fit_stats.incoming_jam.one() == [0, 0]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        incoming_jam=(True, [StatsOptionInJam(time_options=StatTimeSim(time=None))])))
    assert api_tgt_ship_stats.incoming_jam.one() == [0, 0]


def test_count_override(client, consts):
    eve_fighter_id, eve_src_ship_id, _, eve_tgt_ship2_id, _ = setup_fighter_test(
        client=client, consts=consts)
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_fighter = api_src_fit.add_fighter(
        type_id=eve_fighter_id, state=consts.ApiMinionState.engaging, count=2, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship2_id, coordinates=(0, 0, 0))
    api_src_fighter.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.275), approx(0.275)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.275), approx(0.275)]
    # Action
    api_src_fighter.change_fighter(count=4)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [approx(0.55), approx(0.55)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [approx(0.55), approx(0.55)]
    # Action
    api_src_fighter.change_fighter(count=100)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(incoming_jam=True))
    assert api_tgt_fit_stats.incoming_jam.one() == [1, 1]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(incoming_jam=True))
    assert api_tgt_ship_stats.incoming_jam.one() == [1, 1]
