from fw import approx
from fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatsOptionFitDps,
    StatsOptionFitVolley,
    StatsOptionItemDps,
    StatsOptionItemVolley,
)
from tests.stats.dmg import make_eve_drone, make_eve_ship, setup_dmg_basics


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


def test_application(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 64, 0), dmg_mult=8.6, cycle_time=4000,
        range_optimal=6000, range_falloff=5000, tracking=0.97, sig_resolution=400,
        speed_cruise=336, speed_chase=2670, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, speed=1770, sig_radius=126, radius=143)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_drone_nonproj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 678, 0), movement=(0, 0, 0.2))
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification - hard to hit at close range despite low speed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(27.113054), 0]
    assert api_fleet_stats.volley.one() == [0, 0, approx(108.452217), 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(27.113054), 0]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(108.452217), 0]
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(13.556527), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(54.226108), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(13.556527), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(54.226108), 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 3178, 0))
    # Verification - easier to hit at a bit higher range
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(244.321028), 0]
    assert api_fleet_stats.volley.one() == [0, 0, approx(977.284111), 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(244.321028), 0]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(977.284111), 0]
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(122.160514), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(488.642055), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(122.160514), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(488.642055), 0]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification - hard to hit at high speed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(21.747335), 0]
    assert api_fleet_stats.volley.one() == [0, 0, approx(86.989339), 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(21.747335), 0]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(86.989339), 0]
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(10.873667), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(43.49467), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(10.873667), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(43.49467), 0]
    # Action
    api_tgt_ship.change_ship(movement=(90, 0, 1))
    # Verification - no misses due to tracking, since target is moving directly away
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
    api_tgt_ship.change_ship(movement=(85, 0, 1))
    # Verification - movement is at angle, so harder to hit again
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(272.272103), 0]
    assert api_fleet_stats.volley.one() == [0, 0, approx(1089.088412), 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(272.272103), 0]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(1089.088412), 0]
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(136.136051), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(544.544206), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(136.136051), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(544.544206), 0]
    # Action
    api_src_drone_proj.change_drone(movement=(85, 0, 1))
    api_src_drone_nonproj.change_drone(movement=(85, 0, 1))
    # Verification - attacker attempts to transmatch, hits become a bit better
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(274.678174), 0]
    assert api_fleet_stats.volley.one() == [0, 0, approx(1098.712696), 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(274.678174), 0]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(1098.712696), 0]
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(137.339087), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(549.356348), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(137.339087), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(549.356348), 0]
    # Action
    api_src_drone_proj.change_drone(prop_mode=consts.ApiNpcPropMode.chase)
    api_src_drone_nonproj.change_drone(prop_mode=consts.ApiNpcPropMode.chase)
    # Verification - drone speed becomes too high, but still easier to hit target due to lower speed
    # difference, and lower angular velocity
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(277.494088), 0]
    assert api_fleet_stats.volley.one() == [0, 0, approx(1109.976352), 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(277.494088), 0]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(1109.976352), 0]
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(138.747044), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(554.988176), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(138.747044), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(554.988176), 0]
    # Action
    api_src_drone_proj.change_drone(coordinates=(0, 2000, 0))
    api_src_drone_nonproj.change_drone(coordinates=(0, 2000, 0))
    # Verification - drones are moved closer to target, and target is harder to hit due to it
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(266.207424), 0]
    assert api_fleet_stats.volley.one() == [0, 0, approx(1064.829694), 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(266.207424), 0]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(1064.829694), 0]
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(133.103712), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(532.414847), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(133.103712), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(532.414847), 0]
    # Action
    api_src_drone_proj.change_drone(prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_drone_nonproj.change_drone(prop_mode=consts.ApiNpcPropMode.cruise)
    # Verification - with drones getting slower, speed difference becomes higher again
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(247.350615), 0]
    assert api_fleet_stats.volley.one() == [0, 0, approx(989.40246), 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(247.350614), 0]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(989.40246), 0]
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(123.675307), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(494.70123), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(123.675307), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(494.70123), 0]


def test_tgt_npc_prop_mode(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_src_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 64, 0), dmg_mult=8.6, cycle_time=4000,
        range_optimal=6000, range_falloff=5000, tracking=0.97, sig_resolution=400,
        speed_cruise=336, speed_chase=2670, radius=35)
    eve_tgt_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, radius=35,
        speed_cruise=500, speed_chase=1000, sig_radius=100, prop_sig_radius_mult=5)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(
        type_id=eve_src_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_drone_nonproj = api_src_fit.add_drone(
        type_id=eve_src_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_drone = api_tgt_fit.add_drone(
        type_id=eve_tgt_drone_id,
        coordinates=(0, 1070, 0),
        movement=(0, 0, 0.5),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_drone_proj.change_drone(add_projs=[api_tgt_drone.id])
    # Verification
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(57.874072), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(231.496286), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(57.874072), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(231.496286), 0]
    # Action
    api_tgt_drone.change_drone(prop_mode=consts.ApiNpcPropMode.chase)
    # Verification
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(120.271915), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(481.08766), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(120.271915), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(481.08766), 0]
    # Action
    api_tgt_drone.change_drone(prop_mode=consts.ApiNpcPropMode.cruise)
    # Verification
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(57.874072), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(231.496286), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(57.874072), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(231.496286), 0]


def test_drone_effect_range_optimal_absent(client, consts):
    # No falloff range defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts, effect_range=False)
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
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 250, 0), movement=(0, 0, 0))
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
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
    api_tgt_ship.change_ship(coordinates=(0, 5250, 0))
    # Verification - dps is more than halved at 1 falloff
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


def test_drone_effect_range_falloff_absent(client, consts):
    # No falloff range defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts, effect_falloff=False)
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
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 6249, 0), movement=(0, 0, 0))
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
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
    api_tgt_ship.change_ship(coordinates=(0, 6251, 0))
    # Verification - dps is more than halved at 1 falloff
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, 0, 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_drone_effect_tracking_absent(client, consts):
    # No tracking defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts, effect_tracking=False)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 64, 0), dmg_mult=8.6, cycle_time=4000,
        range_optimal=6000, range_falloff=5000, tracking=0.97, sig_resolution=400,
        speed_cruise=336, speed_chase=2670, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, speed=1770, sig_radius=126, radius=143)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_drone_nonproj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 678, 0), movement=(0, 0, 0))
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification - hard to hit at close range despite low speed
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, 0, 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_drone_attr_range_optimal_absent(client, consts):
    # No falloff range defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 64, 0), dmg_mult=8.6, cycle_time=4000,
        range_falloff=5000, tracking=0.97, sig_resolution=400, speed_chase=2670, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, speed=1600, sig_radius=1880, radius=215)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(
        type_id=eve_drone_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_drone_nonproj = api_src_fit.add_drone(
        type_id=eve_drone_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 250, 0), movement=(0, 0, 0))
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
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
    api_tgt_ship.change_ship(coordinates=(0, 5250, 0))
    # Verification - dps is more than halved at 1 falloff
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


def test_drone_attr_range_falloff_absent(client, consts):
    # No falloff range defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 64, 0), dmg_mult=8.6, cycle_time=4000,
        range_optimal=6000, tracking=0.97, sig_resolution=400, speed_chase=2670, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, speed=1600, sig_radius=1880, radius=215)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(
        type_id=eve_drone_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_drone_nonproj = api_src_fit.add_drone(
        type_id=eve_drone_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 6249, 0), movement=(0, 0, 0))
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
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
    api_tgt_ship.change_ship(coordinates=(0, 6251, 0))
    # Verification - dps is more than halved at 1 falloff
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, 0, 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_drone_attr_tracking_absent(client, consts):
    # No tracking defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 64, 0), dmg_mult=8.6, cycle_time=4000,
        range_optimal=6000, range_falloff=5000, sig_resolution=400,
        speed_cruise=336, speed_chase=2670, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, speed=1770, sig_radius=126, radius=143)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_drone_nonproj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 678, 0), movement=(0, 0, 0))
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification - hard to hit at close range despite low speed
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, 0, 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_drone_attr_sig_res_absent(client, consts):
    # No signature resolution defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 64, 0), dmg_mult=8.6, cycle_time=4000,
        range_optimal=6000, range_falloff=5000, tracking=0.97,
        speed_cruise=336, speed_chase=2670, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, speed=1770, sig_radius=126, radius=143)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_drone_nonproj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 678, 0), movement=(0, 0, 0))
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
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
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification - perfect application even if target is moving due to 0 sig res
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


def test_drone_attr_speed_cruise_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 64, 0), dmg_mult=8.6, cycle_time=4000,
        range_optimal=6000, range_falloff=5000, tracking=0.97, sig_resolution=400,
        speed_chase=2670, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, speed=1770, sig_radius=126, radius=143)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 1),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_drone_nonproj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 1),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 6178, 0), movement=(0, 0, 0))
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
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
    api_src_drone_proj.change_drone(prop_mode=consts.ApiNpcPropMode.chase)
    api_src_drone_nonproj.change_drone(prop_mode=consts.ApiNpcPropMode.chase)
    # Verification
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(24.592144), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(98.368575), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(24.592144), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(98.368575), 0]


def test_drone_attr_speed_chase_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 64, 0), dmg_mult=8.6, cycle_time=4000,
        range_optimal=6000, range_falloff=5000, tracking=0.97, sig_resolution=400,
        speed_cruise=336, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, speed=1770, sig_radius=126, radius=143)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 1),
        prop_mode=consts.ApiNpcPropMode.chase)
    api_src_drone_nonproj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 1),
        prop_mode=consts.ApiNpcPropMode.chase)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 6178, 0), movement=(0, 0, 0))
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
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
    api_src_drone_proj.change_drone(prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_drone_nonproj.change_drone(prop_mode=consts.ApiNpcPropMode.cruise)
    # Verification
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, approx(135.249975), 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, approx(540.999898), 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, approx(135.249975), 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, approx(540.999898), 0]


def test_tgt_attr_speed_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 64, 0), dmg_mult=8.6, cycle_time=4000,
        range_optimal=6000, range_falloff=5000, tracking=0.97, sig_resolution=400,
        speed_cruise=336, speed_chase=2670, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, sig_radius=126, radius=143)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_drone_nonproj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 678, 0), movement=(0, 0, 0))
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
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
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification
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


def test_tgt_attr_sig_radius_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 64, 0), dmg_mult=8.6, cycle_time=4000,
        range_optimal=6000, range_falloff=5000, tracking=0.97, sig_resolution=400,
        speed_cruise=336, speed_chase=2670, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, speed=1770, radius=143)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_drone_nonproj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 678, 0), movement=(0, 0, 0))
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification - in case angular is 0 and sig radius is 0, the lib nullifies damage
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, 0, 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_tgt_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 64, 0), dmg_mult=8.6, cycle_time=4000,
        range_optimal=6000, range_falloff=5000, tracking=0.97, sig_resolution=400,
        speed_cruise=336, speed_chase=2670, radius=35)
    eve_tgt_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_drone_nonproj = api_src_fit.add_drone(
        type_id=eve_drone_id,
        state=consts.ApiMinionState.engaging,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 678, 0), movement=(0, 0, 0))
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_drone_proj_stats.volley.one() == [0, 0, 0, 0]
    api_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_drone_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_drone_nonproj_stats.volley.one() == [0, 0, 0, 0]
