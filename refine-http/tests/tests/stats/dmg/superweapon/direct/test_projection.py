from fw import approx
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionFitDmg, StatsOptionItemDmg
from tests.stats.dmg import make_eve_dd_direct_amarr, make_eve_ship, setup_dmg_basics


def test_range(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_direct_amarr(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(4950000, 0, 0, 0),
        cycle_time=240000,
        delay=9000)
    eve_src_struct_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=150000)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, sig_radius=10000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_struct_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(41250), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(9900000), 0, 0, 0]
    api_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(41250), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(9900000), 0, 0, 0]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 1000000, 0))
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(41250), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(9900000), 0, 0, 0]
    api_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(41250), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(9900000), 0, 0, 0]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(4950000), 0, 0, 0]


def test_application(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_direct_amarr(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(4950000, 0, 0, 0),
        cycle_time=240000,
        delay=9000)
    eve_src_struct_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=150000)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=50, sig_radius=32, speed=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_struct_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    # Verification - always perfect application
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(41250), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(9900000), 0, 0, 0]
    api_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(41250), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(9900000), 0, 0, 0]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(4950000), 0, 0, 0]


def test_tgt_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_direct_amarr(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(4950000, 0, 0, 0),
        cycle_time=240000,
        delay=9000)
    eve_src_struct_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=150000)
    eve_tgt_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_struct_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    # Verification - direct DD effect does not rely on target properties whatsoever
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(41250), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(9900000), 0, 0, 0]
    api_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(41250), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(9900000), 0, 0, 0]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(4950000), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(20625), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(4950000), 0, 0, 0]
