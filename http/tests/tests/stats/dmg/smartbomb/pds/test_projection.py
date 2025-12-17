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
from tests.stats.dmg import make_eve_charge_normal, make_eve_pds, make_eve_ship, setup_dmg_basics


def test_range(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_pds(
        client=client, basic_info=eve_basic_info, dmg_mult=1.25, cycle_time=12000,
        capacity=1000, reload_time=180000, range_optimal=25500)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(250, 250, 250, 250), volume=1)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=150000)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=40)
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
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 178499, 0), movement=(0, 0, 1))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification - surface-to-surface range is used, with perfect application
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(52.083333), approx(52.083333), approx(52.083333), approx(52.083333)]
    assert api_fleet_stats.volley.one() == [approx(625), approx(625), approx(625), approx(625)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(52.083333), approx(52.083333), approx(52.083333), approx(52.083333)]
    assert api_src_fit_stats.volley.one() == [approx(625), approx(625), approx(625), approx(625)]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [
        approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_module_proj_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [
        approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_module_nonproj_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 178501, 0))
    # Verification - since target is now barely out of range, PDS deals no damage
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, 0, 0]
    assert api_fleet_stats.volley.one() == [0, 0, 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_src_fit_stats.volley.one() == [0, 0, 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_proj_stats.volley.one() == [0, 0, 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, 0, 0, 0]
