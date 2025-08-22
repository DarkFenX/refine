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
from tests.tests.stats.dmg import make_eve_breacher, make_eve_launcher, make_eve_ship, setup_dmg_basics


def test_hp(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=25, cycle_time=10000, reload_time=30000)
    eve_charge_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=1000, dmg_rel=1, dmg_duration=75000, volume=0.5,
        speed=3000, flight_time=4000, mass=1000, agility=8)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=246)
    eve_tgt_ship1_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, hps=(100000, 300000, 100000), radius=3000, speed=1000, sig_radius=40)
    eve_tgt_ship2_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, hps=(10000, 30000, 10000), radius=3000, speed=1000, sig_radius=40)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 14985, 0), movement=(0, 0, 1))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification - target is close enough for breacher to reach in 100% of cases, and has high
    # enough HP for absolute limit to work
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one().breacher == approx(1000)
    assert api_fleet_stats.volley.one().breacher == approx(1000)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one().breacher == approx(1000)
    assert api_src_fit_stats.volley.one().breacher == approx(1000)
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one().breacher == approx(1000)
    assert api_charge_proj_stats.volley.one().breacher == approx(1000)
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one().breacher == approx(1000)
    assert api_charge_proj_stats.volley.one().breacher == approx(1000)
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one().breacher == approx(1000)
    assert api_charge_nonproj_stats.volley.one().breacher == approx(1000)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification - this target has less HP, so relative limit is used
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one().breacher == approx(500)
    assert api_fleet_stats.volley.one().breacher == approx(500)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one().breacher == approx(500)
    assert api_src_fit_stats.volley.one().breacher == approx(500)
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one().breacher == approx(500)
    assert api_charge_proj_stats.volley.one().breacher == approx(500)
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one().breacher == approx(500)
    assert api_charge_proj_stats.volley.one().breacher == approx(500)
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one().breacher == approx(500)
    assert api_charge_nonproj_stats.volley.one().breacher == approx(500)


def test_range(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=25, cycle_time=10000, reload_time=30000)
    eve_charge_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=1000, dmg_rel=1, dmg_duration=75000, volume=0.5,
        speed=3000, flight_time=4000, mass=1000, agility=8)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=246)
    eve_tgt_ship_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, hps=(10000, 30000, 10000), radius=3000, speed=1000, sig_radius=40)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 14985, 0), movement=(0, 0, 1))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification - in full dps range
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one().breacher == approx(500)
    assert api_fleet_stats.volley.one().breacher == approx(500)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one().breacher == approx(500)
    assert api_src_fit_stats.volley.one().breacher == approx(500)
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one().breacher == approx(500)
    assert api_charge_proj_stats.volley.one().breacher == approx(500)
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one().breacher == approx(500)
    assert api_charge_proj_stats.volley.one().breacher == approx(500)
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one().breacher == approx(500)
    assert api_charge_nonproj_stats.volley.one().breacher == approx(500)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 14990, 0))
    # Verification - slightly out of full dps range
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one().breacher == approx(41)
    assert api_fleet_stats.volley.one().breacher == approx(41)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one().breacher == approx(41)
    assert api_src_fit_stats.volley.one().breacher == approx(41)
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one().breacher == approx(41)
    assert api_charge_proj_stats.volley.one().breacher == approx(41)
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one().breacher == approx(41)
    assert api_charge_proj_stats.volley.one().breacher == approx(41)
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one().breacher == approx(41)
    assert api_charge_nonproj_stats.volley.one().breacher == approx(41)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 17990, 0))
    # Verification - range is out of reach altogether
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one().breacher is None
    assert api_fleet_stats.volley.one().breacher is None
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one().breacher is None
    assert api_src_fit_stats.volley.one().breacher is None
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one().breacher is None
    assert api_charge_proj_stats.volley.one().breacher is None
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one().breacher is None
    assert api_charge_proj_stats.volley.one().breacher is None
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one().breacher is None
    assert api_charge_nonproj_stats.volley.one().breacher is None
