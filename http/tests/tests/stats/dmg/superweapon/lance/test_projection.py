from fw import approx
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionFitDmg, StatsOptionItemDmg
from tests.stats.dmg import make_eve_dd_lance, make_eve_drone, make_eve_ship, setup_dmg_basics


def test_range(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(103125, 0, 0, 0),
        cycle_time=240000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        range_optimal=200000,
        dmg_radius=5000)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=6800)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, sig_radius=13000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 3799, 0), movement=(0, 0, 0))
    # Verification - within attacking ship radius
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 3801, 0))
    # Verification - slightly goes out of attacking ship radius
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(12890.625), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(206250), 0, 0, 0]
    api_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(12890.625), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(206250), 0, 0, 0]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(6445.3125), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(103125), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(6445.3125), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(103125), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 209799, 0))
    # Verification - within surface-to-surface range
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(12890.625), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(206250), 0, 0, 0]
    api_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(12890.625), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(206250), 0, 0, 0]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(6445.3125), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(103125), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(6445.3125), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(103125), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 209801, 0))
    # Verification - slightly out of surface-to-surface range
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_application(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(103125, 0, 0, 0),
        cycle_time=240000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        range_optimal=200000,
        dmg_radius=5000)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=6800)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=250, speed=522, sig_radius=296)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 50000, 0), movement=(0, 0, 1))
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(763.125), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(12210), 0, 0, 0]
    api_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(763.125), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(12210), 0, 0, 0]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(381.5625), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(6105), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(381.5625), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(6105), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 0))
    # Verification - no changes, since application does not depend on speed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(763.125), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(12210), 0, 0, 0]
    api_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(763.125), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(12210), 0, 0, 0]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(381.5625), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(6105), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(381.5625), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(6105), 0, 0, 0]


def test_npc_prop_mode(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(103125, 0, 0, 0),
        cycle_time=240000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        range_optimal=200000,
        dmg_radius=5000)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=6800)
    eve_tgt_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, radius=35,
        speed_cruise=500, speed_chase=1000, sig_radius=100, prop_sig_radius_mult=5)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_drone = api_tgt_fit.add_drone(
        type_id=eve_tgt_drone_id,
        coordinates=(0, 50000, 0),
        movement=(0, 0, 1),
        npc_prop=consts.ApiNpcProp.cruise)
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(128.90625), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(2062.5), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(128.90625), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(2062.5), 0, 0, 0]
    # Action
    api_tgt_drone.change_drone(npc_prop=consts.ApiNpcProp.chase)
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(644.53125), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(10312.5), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(644.53125), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(10312.5), 0, 0, 0]
    # Action
    api_tgt_drone.change_drone(npc_prop=consts.ApiNpcProp.cruise)
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(128.90625), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(2062.5), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(128.90625), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(2062.5), 0, 0, 0]


def test_dd_attr_range_absent(client, consts):
    # No optimal range defined - it is considered equal to be 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(103125, 0, 0, 0),
        cycle_time=240000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        dmg_radius=5000)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=6800)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, sig_radius=13000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 3799, 0), movement=(0, 0, 0))
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 3801, 0))
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(6445.3125), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(103125), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(6445.3125), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(103125), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 9799, 0))
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(6445.3125), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(103125), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(6445.3125), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(103125), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 9801, 0))
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_dd_attr_radius_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(103125, 0, 0, 0),
        cycle_time=240000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        range_optimal=200000)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=6800)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=250, sig_radius=296)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 50000, 0), movement=(0, 0, 0))
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(6445.3125), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(103125), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(6445.3125), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(103125), 0, 0, 0]


def test_dd_ship_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(103125, 0, 0, 0),
        cycle_time=240000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        range_optimal=200000,
        dmg_radius=100)
    eve_src_ship_id = client.alloc_item_id()
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=250, sig_radius=296)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 50000, 0), movement=(0, 0, 0))
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(6445.3125), 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [approx(103125), 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(6445.3125), 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [approx(103125), 0, 0, 0]


def test_tgt_attr_sig_radius_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(103125, 0, 0, 0),
        cycle_time=240000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        range_optimal=200000,
        dmg_radius=100)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=6800)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=250)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 50000, 0), movement=(0, 0, 0))
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_tgt_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(103125, 0, 0, 0),
        cycle_time=240000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        range_optimal=200000,
        dmg_radius=100)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=6800)
    eve_tgt_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 50000, 0), movement=(0, 0, 0))
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [0, 0, 0, 0]
