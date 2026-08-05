from fw import Spool, approx
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionFitDmg, StatsOptionItemDmg
from tests.stats.dmg import make_eve_charge_normal, make_eve_ship, make_eve_turret_spool, setup_dmg_basics


def test_range(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=2.65, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=3500, reload_time=0.01,
        range_optimal=21825, tracking=5.5, sig_resolution=40000)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 630, 0, 448), volume=0.01)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=720)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=88, speed=2600, sig_radius=550)
    client.create_sources()
    api_sol = client.create_sol(default_spool=Spool.spool_scale_to_api(val=1))
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 22462, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(3026.117813), 0, approx(2151.906)]
    assert api_fleet_dmg_stats.volley == [0, approx(10591.412344), 0, approx(7531.671)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, approx(3026.117813), 0, approx(2151.906)]
    assert api_src_fit_dmg_stats.volley == [0, approx(10591.412344), 0, approx(7531.671)]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, approx(1513.058906), 0, approx(1075.953)]
    assert api_module_proj_dmg_stats.volley == [0, approx(5295.706172), 0, approx(3765.8355)]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, approx(1513.058906), 0, approx(1075.953)]
    assert api_module_nonproj_dmg_stats.volley == [0, approx(5295.706172), 0, approx(3765.8355)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 22464, 0))
    # Verification - no falloff, no damage past optimal
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_application(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=2.65, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=3500, reload_time=0.01,
        range_optimal=21825, tracking=5.5, sig_resolution=40000)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 630, 0, 448), volume=0.01)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=720)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=88, speed=2600, sig_radius=550)
    client.create_sources()
    api_sol = client.create_sol(default_spool=Spool.spool_scale_to_api(val=1))
    api_src_fit = api_sol.create_fit()
    api_src_ship = api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_proj_item_ids=[api_tgt_ship.id])
    # Verification - same coordinates + no movement - no mitigation
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(3026.117813), 0, approx(2151.906)]
    assert api_fleet_dmg_stats.volley == [0, approx(10591.412344), 0, approx(7531.671)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, approx(3026.117813), 0, approx(2151.906)]
    assert api_src_fit_dmg_stats.volley == [0, approx(10591.412344), 0, approx(7531.671)]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, approx(1513.058906), 0, approx(1075.953)]
    assert api_module_proj_dmg_stats.volley == [0, approx(5295.706172), 0, approx(3765.8355)]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, approx(1513.058906), 0, approx(1075.953)]
    assert api_module_nonproj_dmg_stats.volley == [0, approx(5295.706172), 0, approx(3765.8355)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 1638, 0), movement=(0, 0, 0.05))
    # Verification - hard to hit at close range despite low speed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(1079.339077), 0, approx(767.530011)]
    assert api_fleet_dmg_stats.volley == [0, approx(3777.686771), 0, approx(2686.355037)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, approx(1079.339077), 0, approx(767.530011)]
    assert api_src_fit_dmg_stats.volley == [0, approx(3777.686771), 0, approx(2686.355037)]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, approx(539.669539), 0, approx(383.765005)]
    assert api_module_proj_dmg_stats.volley == [0, approx(1888.843385), 0, approx(1343.177518)]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, approx(539.669539), 0, approx(383.765005)]
    assert api_module_nonproj_dmg_stats.volley == [0, approx(1888.843385), 0, approx(1343.177518)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 10638, 0))
    # Verification - easier to hit at a bit higher range
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(2946.922422), 0, approx(2095.589278)]
    assert api_fleet_dmg_stats.volley == [0, approx(10314.228477), 0, approx(7334.562472)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, approx(2946.922422), 0, approx(2095.589278)]
    assert api_src_fit_dmg_stats.volley == [0, approx(10314.228477), 0, approx(7334.562472)]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, approx(1473.461211), 0, approx(1047.794639)]
    assert api_module_proj_dmg_stats.volley == [0, approx(5157.114238), 0, approx(3667.281236)]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, approx(1473.461211), 0, approx(1047.794639)]
    assert api_module_nonproj_dmg_stats.volley == [0, approx(5157.114238), 0, approx(3667.281236)]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification - almost impossible to hit at high speed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(6.417292), 0, approx(4.563408)]
    assert api_fleet_dmg_stats.volley == [0, approx(22.460523), 0, approx(15.971927)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, approx(6.417292), 0, approx(4.563408)]
    assert api_src_fit_dmg_stats.volley == [0, approx(22.460523), 0, approx(15.971927)]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, approx(3.208646), 0, approx(2.281704)]
    assert api_module_proj_dmg_stats.volley == [0, approx(11.230261), 0, approx(7.985964)]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, approx(3.208646), 0, approx(2.281704)]
    assert api_module_nonproj_dmg_stats.volley == [0, approx(11.230261), 0, approx(7.985964)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 5638, 0), movement=(90, 0, 1))
    # Verification - no misses due to tracking, since target is moving directly away
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(3026.117813), 0, approx(2151.906)]
    assert api_fleet_dmg_stats.volley == [0, approx(10591.412344), 0, approx(7531.671)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, approx(3026.117813), 0, approx(2151.906)]
    assert api_src_fit_dmg_stats.volley == [0, approx(10591.412344), 0, approx(7531.671)]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, approx(1513.058906), 0, approx(1075.953)]
    assert api_module_proj_dmg_stats.volley == [0, approx(5295.706172), 0, approx(3765.8355)]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, approx(1513.058906), 0, approx(1075.953)]
    assert api_module_nonproj_dmg_stats.volley == [0, approx(5295.706172), 0, approx(3765.8355)]
    # Action
    api_tgt_ship.change_ship(movement=(85, 0, 1))
    # Verification - movement is at angle, so harder to hit again
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(2283.393337), 0, approx(1623.746373)]
    assert api_fleet_dmg_stats.volley == [0, approx(7991.876678), 0, approx(5683.112304)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, approx(2283.393337), 0, approx(1623.746373)]
    assert api_src_fit_dmg_stats.volley == [0, approx(7991.876678), 0, approx(5683.112304)]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, approx(1141.696668), 0, approx(811.873186)]
    assert api_module_proj_dmg_stats.volley == [0, approx(3995.938339), 0, approx(2841.556152)]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, approx(1141.696668), 0, approx(811.873186)]
    assert api_module_nonproj_dmg_stats.volley == [0, approx(3995.938339), 0, approx(2841.556152)]
    # Action
    api_src_ship.change_ship(movement=(85, 0, 1))
    # Verification - attacker attempts to transmatch, hits become much better
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(2608.016658), 0, approx(1854.589623)]
    assert api_fleet_dmg_stats.volley == [0, approx(9128.058302), 0, approx(6491.063682)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, approx(2608.016658), 0, approx(1854.589623)]
    assert api_src_fit_dmg_stats.volley == [0, approx(9128.058302), 0, approx(6491.063682)]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, approx(1304.008329), 0, approx(927.294812)]
    assert api_module_proj_dmg_stats.volley == [0, approx(4564.029151), 0, approx(3245.531841)]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, approx(1304.008329), 0, approx(927.294812)]
    assert api_module_nonproj_dmg_stats.volley == [0, approx(4564.029151), 0, approx(3245.531841)]
    # Action
    api_src_ship.change_ship(coordinates=(0, 4000, 0))
    # Verification - attacker is moved closer to target, hits become a bit worse again
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(640.909635), 0, approx(455.757963)]
    assert api_fleet_dmg_stats.volley == [0, approx(2243.183722), 0, approx(1595.152869)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, approx(640.909635), 0, approx(455.757963)]
    assert api_src_fit_dmg_stats.volley == [0, approx(2243.183722), 0, approx(1595.152869)]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, approx(320.454817), 0, approx(227.878981)]
    assert api_module_proj_dmg_stats.volley == [0, approx(1121.591861), 0, approx(797.576434)]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, approx(320.454817), 0, approx(227.878981)]
    assert api_module_nonproj_dmg_stats.volley == [0, approx(1121.591861), 0, approx(797.576434)]


def test_crit(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=2.65, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=3500, reload_time=0.01,
        range_optimal=21825, tracking=5.5, sig_resolution=40000)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 630, 0, 448), volume=0.01)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=720)
    eve_tgt_ship_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, radius=88, speed=2600, sig_radius=550)
    client.create_sources()
    api_sol = client.create_sol(default_spool=Spool.spool_scale_to_api(val=1))
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 11500, 0), movement=(0, 0, 0))
    api_src_module.change_module(add_proj_item_ids=[api_tgt_ship.id])
    # Verification - ensure perfectly applied damage equals to paper damage
    api_module_stats = api_src_module.get_stats(options=ItemStatsOptions(dmg=[
        StatsOptionItemDmg(),
        StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemDmg(crits=consts.ApiStatCrits.exclude),
        StatsOptionItemDmg(crits=consts.ApiStatCrits.exclude, projectee_item_id=api_tgt_ship.id),
        StatsOptionItemDmg(crits=consts.ApiStatCrits.include),
        StatsOptionItemDmg(crits=consts.ApiStatCrits.include, projectee_item_id=api_tgt_ship.id)]))
    (api_module_stats_default_nonproj,
     api_module_stats_default_proj,
     api_module_stats_excluded_nonproj,
     api_module_stats_excluded_proj,
     api_module_stats_included_nonproj,
     api_module_stats_included_proj) = api_module_stats.dmg
    assert api_module_stats_default_nonproj.dps.thermal == approx(api_module_stats_default_proj.dps.thermal)
    assert api_module_stats_default_nonproj.dps.explosive == approx(api_module_stats_default_proj.dps.explosive)
    assert api_module_stats_default_nonproj.volley.thermal == approx(api_module_stats_default_proj.volley.thermal)
    assert api_module_stats_default_nonproj.volley.explosive == approx(api_module_stats_default_proj.volley.explosive)
    assert api_module_stats_excluded_nonproj.dps.thermal == approx(api_module_stats_excluded_proj.dps.thermal)
    assert api_module_stats_excluded_nonproj.dps.explosive == approx(api_module_stats_excluded_proj.dps.explosive)
    assert api_module_stats_excluded_nonproj.volley.thermal == approx(api_module_stats_excluded_proj.volley.thermal)
    assert api_module_stats_excluded_nonproj.volley.explosive == approx(api_module_stats_excluded_proj.volley.explosive)
    assert api_module_stats_included_nonproj.dps.thermal == approx(api_module_stats_included_proj.dps.thermal)
    assert api_module_stats_included_nonproj.dps.explosive == approx(api_module_stats_included_proj.dps.explosive)
    assert api_module_stats_included_nonproj.volley.thermal == approx(api_module_stats_included_proj.volley.thermal)
    assert api_module_stats_included_nonproj.volley.explosive == approx(api_module_stats_included_proj.volley.explosive)
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 0.5))
    # Verification - non-perfect application due to target moving
    api_module_stats = api_src_module.get_stats(options=ItemStatsOptions(dmg=[
        StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemDmg(crits=consts.ApiStatCrits.exclude, projectee_item_id=api_tgt_ship.id),
        StatsOptionItemDmg(crits=consts.ApiStatCrits.include, projectee_item_id=api_tgt_ship.id)]))
    api_module_stats_default, api_module_stats_excluded, api_module_stats_included = api_module_stats.dmg
    assert api_module_stats_default.dps == [0, approx(226.217602), 0, approx(160.86585)]
    assert api_module_stats_default.volley == [0, approx(791.761607), 0, approx(563.030476)]
    assert api_module_stats_excluded.dps == [0, approx(192.671947), 0, approx(137.011162)]
    assert api_module_stats_excluded.volley == [0, approx(674.351815), 0, approx(479.539069)]
    assert api_module_stats_included.dps == [0, approx(226.217602), 0, approx(160.86585)]
    assert api_module_stats_included.volley == [0, approx(791.761607), 0, approx(563.030476)]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification - even worse application due to target moving at full speed; since chance to hit
    # is low, difference is very stark between crit and non-crit
    api_module_stats = api_src_module.get_stats(options=ItemStatsOptions(dmg=[
        StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemDmg(crits=consts.ApiStatCrits.exclude, projectee_item_id=api_tgt_ship.id),
        StatsOptionItemDmg(crits=consts.ApiStatCrits.include, projectee_item_id=api_tgt_ship.id)]))
    api_module_stats_default, api_module_stats_excluded, api_module_stats_included = api_module_stats.dmg
    assert api_module_stats_default.dps == [0, approx(9.120279), 0, approx(6.485532)]
    assert api_module_stats_default.volley == [0, approx(31.920978), 0, approx(22.699362)]
    assert api_module_stats_excluded.dps == [0, approx(1.53077), 0, approx(1.088547)]
    assert api_module_stats_excluded.volley == [0, approx(5.357693), 0, approx(3.809915)]
    assert api_module_stats_included.dps == [0, approx(9.120279), 0, approx(6.485532)]
    assert api_module_stats_included.volley == [0, approx(31.920978), 0, approx(22.699362)]
