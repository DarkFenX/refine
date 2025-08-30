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
from tests.tests.stats.dmg import make_eve_charge_crystal, make_eve_ship, make_eve_turret_laser, setup_dmg_basics


def test_range(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_laser(
        client=client, basic_info=eve_basic_info, dmg_mult=36.25, capacity=1, cycle_time=6400, reload_time=0.01,
        range_optimal=220000, range_falloff=42200, tracking=0.6, sig_resolution=40000)
    eve_charge_id = make_eve_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(20, 12, 0, 0), volume=1,
        get_damaged=1, hp=1, vol_dmg=0.01, vol_chance=0.1)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=1550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=215, speed=1600, sig_radius=1880)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 220715, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(229.972266), approx(137.983359), 0, 0]
    assert api_fleet_stats.volley.one() == [approx(1471.8225), approx(883.0935), 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(229.972266), approx(137.983359), 0, 0]
    assert api_src_fit_stats.volley.one() == [approx(1471.8225), approx(883.0935), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [approx(114.986133), approx(68.99168), 0, 0]
    assert api_module_proj_stats.volley.one() == [approx(735.91125), approx(441.54675), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [approx(114.986133), approx(68.99168), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [approx(735.91125), approx(441.54675), 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 262915, 0))
    # Verification - dps is more than halved at 1 falloff
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(89.503516), approx(53.702109), 0, 0]
    assert api_fleet_stats.volley.one() == [approx(572.8225), approx(343.6935), 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(89.503516), approx(53.702109), 0, 0]
    assert api_src_fit_stats.volley.one() == [approx(572.8225), approx(343.6935), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [approx(44.751758), approx(26.851055), 0, 0]
    assert api_module_proj_stats.volley.one() == [approx(286.41125), approx(171.84675), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [approx(44.751758), approx(26.851055), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [approx(286.41125), approx(171.84675), 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 305115, 0))
    # Verification - 2 falloffs
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(13.056372), approx(7.833823), 0, 0]
    assert api_fleet_stats.volley.one() == [approx(83.560781), approx(50.136469), 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(13.056372), approx(7.833823), 0, 0]
    assert api_src_fit_stats.volley.one() == [approx(83.560781), approx(50.136469), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [approx(6.528186), approx(3.916912), 0, 0]
    assert api_module_proj_stats.volley.one() == [approx(41.780391), approx(25.068234), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [approx(6.528186), approx(3.916912), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [approx(41.780391), approx(25.068234), 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 350000, 0))
    # Verification - more than 3 falloffs, still some damage applied
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(1.016018), approx(0.6096108), 0, 0]
    assert api_fleet_stats.volley.one() == [approx(6.502515), approx(3.901509), 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(1.016018), approx(0.6096108), 0, 0]
    assert api_src_fit_stats.volley.one() == [approx(6.502515), approx(3.901509), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [approx(0.508009), approx(0.3048054), 0, 0]
    assert api_module_proj_stats.volley.one() == [approx(3.251258), approx(1.950755), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [approx(0.508009), approx(0.3048054), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [approx(3.251258), approx(1.950755), 0, 0]


def test_application(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_laser(
        client=client, basic_info=eve_basic_info, dmg_mult=36.25, capacity=1, cycle_time=6400, reload_time=0.01,
        range_optimal=220000, range_falloff=42200, tracking=0.6, sig_resolution=40000)
    eve_charge_id = make_eve_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(20, 12, 0, 0), volume=1,
        get_damaged=1, hp=1, vol_dmg=0.01, vol_chance=0.1)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=1550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=215, speed=1600, sig_radius=1880)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_ship = api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 10715, 0), movement=(0, 0, 0.2))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification - hard to hit at close range despite low speed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(80.626988), approx(48.376193), 0, 0]
    assert api_fleet_stats.volley.one() == [approx(516.012724), approx(309.607634), 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(80.626988), approx(48.376193), 0, 0]
    assert api_src_fit_stats.volley.one() == [approx(516.012724), approx(309.607634), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [approx(40.313494), approx(24.188096), 0, 0]
    assert api_module_proj_stats.volley.one() == [approx(258.006362), approx(154.803817), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [approx(40.313494), approx(24.188096), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [approx(258.006362), approx(154.803817), 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 60715, 0))
    # Verification - easier to hit at a bit higher range
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(221.961718), approx(133.177031), 0, 0]
    assert api_fleet_stats.volley.one() == [approx(1420.554998), approx(852.332999), 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(221.961718), approx(133.177031), 0, 0]
    assert api_src_fit_stats.volley.one() == [approx(1420.554998), approx(852.332999), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [approx(110.980859), approx(66.588516), 0, 0]
    assert api_module_proj_stats.volley.one() == [approx(710.277499), approx(426.166499), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [approx(110.980859), approx(66.588516), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [approx(710.277499), approx(426.166499), 0, 0]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification - harder to hit at full speed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(100.039089), approx(60.023453), 0, 0]
    assert api_fleet_stats.volley.one() == [approx(640.250167), approx(384.1501), 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(100.039089), approx(60.023453), 0, 0]
    assert api_src_fit_stats.volley.one() == [approx(640.250167), approx(384.1501), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [approx(50.019544), approx(30.011727), 0, 0]
    assert api_module_proj_stats.volley.one() == [approx(320.125084), approx(192.07505), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [approx(50.019544), approx(30.011727), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [approx(320.125084), approx(192.07505), 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 3715, 0), movement=(90, 0, 1))
    # Verification - no misses due to tracking, since target is moving directly away
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(229.972266), approx(137.983359), 0, 0]
    assert api_fleet_stats.volley.one() == [approx(1471.8225), approx(883.0935), 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(229.972266), approx(137.983359), 0, 0]
    assert api_src_fit_stats.volley.one() == [approx(1471.8225), approx(883.0935), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [approx(114.986133), approx(68.99168), 0, 0]
    assert api_module_proj_stats.volley.one() == [approx(735.91125), approx(441.54675), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [approx(114.986133), approx(68.99168), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [approx(735.91125), approx(441.54675), 0, 0]
    # Action
    api_tgt_ship.change_ship(movement=(85, 0, 1))
    # Verification - movement is at angle, so harder to hit again
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(47.900016), approx(28.74001), 0, 0]
    assert api_fleet_stats.volley.one() == [approx(306.560105), approx(183.936063), 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(47.900016), approx(28.74001), 0, 0]
    assert api_src_fit_stats.volley.one() == [approx(306.560105), approx(183.936063), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [approx(23.950008), approx(14.370005), 0, 0]
    assert api_module_proj_stats.volley.one() == [approx(153.280052), approx(91.968031), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [approx(23.950008), approx(14.370005), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [approx(153.280052), approx(91.968031), 0, 0]
    # Action
    api_src_ship.change_ship(movement=(85, 0, 1))
    # Verification - attacker attempts to transmatch, hits become much better
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(229.567801), approx(137.740680), 0, 0]
    assert api_fleet_stats.volley.one() == [approx(1469.233925), approx(881.540355), 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(229.567801), approx(137.740680), 0, 0]
    assert api_src_fit_stats.volley.one() == [approx(1469.233925), approx(881.540355), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [approx(114.7839), approx(68.87034), 0, 0]
    assert api_module_proj_stats.volley.one() == [approx(734.616962), approx(440.770177), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [approx(114.7839), approx(68.87034), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [approx(734.616962), approx(440.770177), 0, 0]
    # Action
    api_src_ship.change_ship(coordinates=(0, 3000, 0))
    # Verification - attacker is moved closer to target, hits become a bit worse again
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(219.332265), approx(131.599359), 0, 0]
    assert api_fleet_stats.volley.one() == [approx(1403.726496), approx(842.235897), 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(219.332265), approx(131.599359), 0, 0]
    assert api_src_fit_stats.volley.one() == [approx(1403.726496), approx(842.235897), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [approx(109.666132), approx(65.799679), 0, 0]
    assert api_module_proj_stats.volley.one() == [approx(701.863248), approx(421.117949), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [approx(109.666132), approx(65.799679), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [approx(701.863248), approx(421.117949), 0, 0]
