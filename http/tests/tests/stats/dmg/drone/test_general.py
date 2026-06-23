from fw import approx
from fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatDmgItemKinds,
    StatsOptionFitDmg,
    StatsOptionItemDmg,
    StatTimeBurst,
    StatTimeSim,
)
from tests.stats.dmg import make_eve_drone, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 13, 19, 0), dmg_mult=41, cycle_time=4000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_fleet_dmg_stats.volley == [0, approx(533), approx(779), 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_fit_dmg_stats.volley == [0, approx(533), approx(779), 0]
    api_drone_dmg_stats = api_drone.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_drone_dmg_stats.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_drone_dmg_stats.volley == [0, approx(533), approx(779), 0]
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_drone_dmg_normal, api_drone_dmg_ignored = api_drone_stats.dmg
    assert api_drone_dmg_normal.dps == [0, 0, 0, 0]
    assert api_drone_dmg_normal.volley == [0, 0, 0, 0]
    assert api_drone_dmg_ignored.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_drone_dmg_ignored.volley == [0, approx(533), approx(779), 0]
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_fleet_dmg_stats.volley == [0, approx(533), approx(779), 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_fit_dmg_stats.volley == [0, approx(533), approx(779), 0]
    api_drone_dmg_stats = api_drone.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_drone_dmg_stats.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_drone_dmg_stats.volley == [0, approx(533), approx(779), 0]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone1_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 13, 19, 0), dmg_mult=41, cycle_time=4000)
    eve_drone2_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 13, 19), dmg_mult=36, cycle_time=4000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.engaging)
    api_fit1.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.engaging)
    api_fit2 = api_sol.create_fit()
    api_fit2.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(266.5), approx(506.5), approx(171)]
    assert api_fleet_dmg_stats.volley == [0, approx(1066), approx(2026), approx(684)]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [0, approx(133.25), approx(311.75), approx(171)]
    assert api_fit1_dmg_stats.volley == [0, approx(533), approx(1247), approx(684)]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_fit2_dmg_stats.volley == [0, approx(533), approx(779), 0]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone1_id = make_eve_drone(
        client=client, basic_info=eve_basic_info,
        dmgs=(0, 13, 19, 0), dmg_mult=41, cycle_time=4000, speed_chase=2500)
    eve_drone2_id = make_eve_drone(
        client=client, basic_info=eve_basic_info,
        dmgs=(64, 0, 0, 0), dmg_mult=6.2, cycle_time=4000, speed_chase=0.00001)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.engaging)
    api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, minion_mobile=True)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, minion_static=True))])))
    api_fleet_dmg_default, api_fleet_dmg_mobile, api_fleet_dmg_sentry = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [approx(99.2), approx(133.25), approx(194.75), 0]
    assert api_fleet_dmg_default.volley == [approx(396.8), approx(533), approx(779), 0]
    assert api_fleet_dmg_mobile.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_fleet_dmg_mobile.volley == [0, approx(533), approx(779), 0]
    assert api_fleet_dmg_sentry.dps == [approx(99.2), 0, 0, 0]
    assert api_fleet_dmg_sentry.volley == [approx(396.8), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, minion_mobile=True)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, minion_static=True))])))
    api_fit_dmg_default, api_fit_dmg_mobile, api_fit_dmg_sentry = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [approx(99.2), approx(133.25), approx(194.75), 0]
    assert api_fit_dmg_default.volley == [approx(396.8), approx(533), approx(779), 0]
    assert api_fit_dmg_mobile.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_fit_dmg_mobile.volley == [0, approx(533), approx(779), 0]
    assert api_fit_dmg_sentry.dps == [approx(99.2), 0, 0, 0]
    assert api_fit_dmg_sentry.volley == [approx(396.8), 0, 0, 0]


def test_time(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 13, 19, 0), dmg_mult=41, cycle_time=4000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification - burst stats
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_fleet_dmg_stats.volley == [0, approx(533), approx(779), 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_fit_dmg_stats.volley == [0, approx(533), approx(779), 0]
    api_drone_dmg_stats = api_drone.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_drone_dmg_stats.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_drone_dmg_stats.volley == [0, approx(533), approx(779), 0]
    # Verification - sim without time
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_fleet_dmg_stats.volley == [0, approx(533), approx(779), 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_fit_dmg_stats.volley == [0, approx(533), approx(779), 0]
    api_drone_dmg_stats = api_drone.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_drone_dmg_stats.dps == [0, approx(133.25), approx(194.75), 0]
    assert api_drone_dmg_stats.volley == [0, approx(533), approx(779), 0]
    # Verification - sim with time after first hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(533), approx(779), 0]
    assert api_fleet_dmg_stats.volley == [0, approx(533), approx(779), 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(533), approx(779), 0]
    assert api_fit_dmg_stats.volley == [0, approx(533), approx(779), 0]
    api_drone_dmg_stats = api_drone.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_drone_dmg_stats.dps == [0, approx(533), approx(779), 0]
    assert api_drone_dmg_stats.volley == [0, approx(533), approx(779), 0]
    # Verification - sim with time before second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=3))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(177.666667), approx(259.666667), 0]
    assert api_fleet_dmg_stats.volley == [0, approx(533), approx(779), 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=3))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(177.666667), approx(259.666667), 0]
    assert api_fit_dmg_stats.volley == [0, approx(533), approx(779), 0]
    api_drone_dmg_stats = api_drone.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=3))]))).dmg.one()
    assert api_drone_dmg_stats.dps == [0, approx(177.666667), approx(259.666667), 0]
    assert api_drone_dmg_stats.volley == [0, approx(533), approx(779), 0]
    # Verification - sim with time after second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=5))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(213.2), approx(311.6), 0]
    assert api_fleet_dmg_stats.volley == [0, approx(533), approx(779), 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=5))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(213.2), approx(311.6), 0]
    assert api_fit_dmg_stats.volley == [0, approx(533), approx(779), 0]
    api_drone_dmg_stats = api_drone.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=5))]))).dmg.one()
    assert api_drone_dmg_stats.dps == [0, approx(213.2), approx(311.6), 0]
    assert api_drone_dmg_stats.volley == [0, approx(533), approx(779), 0]
