from tests import approx
from tests.fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatsOptionFitDps,
    StatsOptionFitVolley,
    StatsOptionItemDps,
    StatsOptionItemVolley,
)
from tests.tests.stats.dmg import make_eve_drone, make_eve_ship, setup_dmg_basics


def test_range(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 64, 0), dmg_mult=8.6, cycle_time=4000,
        range_optimal=6000, range_falloff=5000, tracking=0.97, sig_resolution=400, speed_chase=2670, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, speed=1600, sig_radius=1880, radius=215)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(
        type_id=eve_drone_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_drone_nonproj = api_src_fit.add_drone(
        type_id=eve_drone_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 6250, 0), movement=(0, 0, 0))
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(279.34176), 0]
    assert api_fleet_stats.volley.one() == [0, 0, approx(1117.36704), 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(279.34176), 0]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(1117.36704), 0]
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(139.67088), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(558.68352), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(139.67088), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(558.68352), 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 11250, 0))
    # Verification - dps is more than halved at 1 falloff
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(108.71776), 0]
    assert api_fleet_stats.volley.one() == [0, 0, approx(434.87104), 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(108.71776), 0]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(434.87104), 0]
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(54.35888), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(217.43552), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(54.35888), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(217.43552), 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 16250, 0))
    # Verification - 2 falloffs
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(15.85926), 0]
    assert api_fleet_stats.volley.one() == [0, 0, approx(63.43704), 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(15.85926), 0]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(63.43704), 0]
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(7.92963), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(31.71852), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(7.92963), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(31.71852), 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 22000, 0))
    # Verification - more than 3 falloffs, still some damage applied
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(0.8507453), 0]
    assert api_fleet_stats.volley.one() == [0, 0, approx(3.402981), 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(0.8507453), 0]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(3.402981), 0]
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(0.4253726), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(1.701491), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(0.4253726), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(1.701491), 0]
