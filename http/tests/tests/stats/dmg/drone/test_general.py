from tests import approx
from tests.fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatDmgItemKinds,
    StatsOptionFitDps,
    StatsOptionFitVolley,
    StatsOptionItemDps,
    StatsOptionItemVolley,
)
from tests.tests.stats.dmg import make_eve_drone, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 13, 19, 0), dmg_mult=41, cycle_time=4000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [0, approx(133.25), approx(194.75), 0]
    assert api_fleet_stats.volley.one() == [0, approx(533), approx(779), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, approx(133.25), approx(194.75), 0]
    assert api_fit_stats.volley.one() == [0, approx(533), approx(779), 0]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_drone_stats.dps.one() == [0, approx(133.25), approx(194.75), 0]
    assert api_drone_stats.volley.one() == [0, approx(533), approx(779), 0]
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [0, 0, 0, 0]
    assert api_fleet_stats.volley.one() == [0, 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(), StatsOptionItemDps(ignore_state=True)]),
        volley=(True, [StatsOptionItemVolley(), StatsOptionItemVolley(ignore_state=True)])))
    api_drone_dps_normal, api_drone_dps_ignored = api_drone_stats.dps
    assert api_drone_dps_normal == [0, 0, 0, 0]
    assert api_drone_dps_ignored == [0, approx(133.25), approx(194.75), 0]
    api_drone_volley_normal, api_drone_volley_ignored = api_drone_stats.volley
    assert api_drone_volley_normal == [0, 0, 0, 0]
    assert api_drone_volley_ignored == [0, approx(533), approx(779), 0]
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [0, approx(133.25), approx(194.75), 0]
    assert api_fleet_stats.volley.one() == [0, approx(533), approx(779), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, approx(133.25), approx(194.75), 0]
    assert api_fit_stats.volley.one() == [0, approx(533), approx(779), 0]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_drone_stats.dps.one() == [0, approx(133.25), approx(194.75), 0]
    assert api_drone_stats.volley.one() == [0, approx(533), approx(779), 0]


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
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [0, approx(266.5), approx(506.5), approx(171)]
    assert api_fleet_stats.volley.one() == [0, approx(1066), approx(2026), approx(684)]
    api_fit1_stats = api_fit1.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit1_stats.dps.one() == [0, approx(133.25), approx(311.75), approx(171)]
    assert api_fit1_stats.volley.one() == [0, approx(533), approx(1247), approx(684)]
    api_fit2_stats = api_fit2.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit2_stats.dps.one() == [0, approx(133.25), approx(194.75), 0]
    assert api_fit2_stats.volley.one() == [0, approx(533), approx(779), 0]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone1_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 13, 19, 0), dmg_mult=41, cycle_time=4000, speed=2500)
    eve_drone2_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(64, 0, 0, 0), dmg_mult=6.2, cycle_time=4000, speed=0.00001)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.engaging)
    api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [
            StatsOptionFitDps(),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, minion_mobile=True)),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, minion_static=True))]),
        volley=(True, [
            StatsOptionFitVolley(),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, minion_mobile=True)),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, minion_static=True))])))
    api_fleet_dps_default, api_fleet_dps_mobile, api_fleet_dps_sentry = api_fleet_stats.dps
    assert api_fleet_dps_default == [approx(99.2), approx(133.25), approx(194.75), 0]
    assert api_fleet_dps_mobile == [0, approx(133.25), approx(194.75), 0]
    assert api_fleet_dps_sentry == [approx(99.2), 0, 0, 0]
    api_fleet_volley_default, api_fleet_volley_mobile, api_fleet_volley_sentry = api_fleet_stats.volley
    assert api_fleet_volley_default == [approx(396.8), approx(533), approx(779), 0]
    assert api_fleet_volley_mobile == [0, approx(533), approx(779), 0]
    assert api_fleet_volley_sentry == [approx(396.8), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [
            StatsOptionFitDps(),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, minion_mobile=True)),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, minion_static=True))]),
        volley=(True, [
            StatsOptionFitVolley(),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, minion_mobile=True)),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, minion_static=True))])))
    api_fit_dps_default, api_fit_dps_mobile, api_fit_dps_sentry = api_fit_stats.dps
    assert api_fit_dps_default == [approx(99.2), approx(133.25), approx(194.75), 0]
    assert api_fit_dps_mobile == [0, approx(133.25), approx(194.75), 0]
    assert api_fit_dps_sentry == [approx(99.2), 0, 0, 0]
    api_fit_volley_default, api_fit_volley_mobile, api_fit_volley_sentry = api_fit_stats.volley
    assert api_fit_volley_default == [approx(396.8), approx(533), approx(779), 0]
    assert api_fit_volley_mobile == [0, approx(533), approx(779), 0]
    assert api_fit_volley_sentry == [approx(396.8), 0, 0, 0]
