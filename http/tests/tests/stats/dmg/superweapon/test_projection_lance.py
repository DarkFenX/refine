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
from tests.tests.stats.dmg import make_eve_dd_lance_debuff, make_eve_drone, make_eve_ship, setup_dmg_basics


def test_range(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance_debuff(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(0, 25500, 0, 0),
        cycle_time=300000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        range_optimal=100000,
        dmg_radius=100)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=1613)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=250, sig_radius=296)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 1362, 0), movement=(0, 0, 0))
    # Verification - within attacking ship radius
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, 0, 0]
    assert api_fleet_stats.volley.one() == [0, 0, 0, 0]
    api_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
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
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 1364, 0))
    # Verification - slightly goes out of attacking ship radius
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, approx(2550), 0, 0]
    assert api_fleet_stats.volley.one() == [0, approx(51000), 0, 0]
    api_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fit_stats.dps.one() == [0, approx(2550), 0, 0]
    assert api_fit_stats.volley.one() == [0, approx(51000), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, approx(1275), 0, 0]
    assert api_module_proj_stats.volley.one() == [0, approx(25500), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, approx(1275), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, approx(25500), 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 101862, 0))
    # Verification - within surface-to-surface range
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, approx(2550), 0, 0]
    assert api_fleet_stats.volley.one() == [0, approx(51000), 0, 0]
    api_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fit_stats.dps.one() == [0, approx(2550), 0, 0]
    assert api_fit_stats.volley.one() == [0, approx(51000), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, approx(1275), 0, 0]
    assert api_module_proj_stats.volley.one() == [0, approx(25500), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, approx(1275), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, approx(25500), 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 101864, 0))
    # Verification - slightly out of surface-to-surface range
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, 0, 0]
    assert api_fleet_stats.volley.one() == [0, 0, 0, 0]
    api_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
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


def test_application(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance_debuff(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(0, 25500, 0, 0),
        cycle_time=300000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        range_optimal=100000,
        dmg_radius=2500)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=1613)
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
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, approx(301.92), 0, 0]
    assert api_fleet_stats.volley.one() == [0, approx(6038.4), 0, 0]
    api_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fit_stats.dps.one() == [0, approx(301.92), 0, 0]
    assert api_fit_stats.volley.one() == [0, approx(6038.4), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, approx(150.96), 0, 0]
    assert api_module_proj_stats.volley.one() == [0, approx(3019.2), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, approx(150.96), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, approx(3019.2), 0, 0]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 0))
    # Verification - no changes, since application does not depend on speed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, approx(301.92), 0, 0]
    assert api_fleet_stats.volley.one() == [0, approx(6038.4), 0, 0]
    api_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fit_stats.dps.one() == [0, approx(301.92), 0, 0]
    assert api_fit_stats.volley.one() == [0, approx(6038.4), 0, 0]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, approx(150.96), 0, 0]
    assert api_module_proj_stats.volley.one() == [0, approx(3019.2), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, approx(150.96), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, approx(3019.2), 0, 0]


def test_npc_prop_mode(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance_debuff(
        client=client,
        basic_info=eve_basic_info,
            dmgs=(0, 25500, 0, 0),
        cycle_time=300000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        range_optimal=100000,
        dmg_radius=2500)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=1613)
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
        prop_mode=consts.ApiNpcPropMode.cruise)
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_module_proj_stats.dps.one() == [0, approx(51), 0, 0]
    assert api_module_proj_stats.volley.one() == [0, approx(1020), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, approx(51), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, approx(1020), 0, 0]
    # Action
    api_tgt_drone.change_drone(prop_mode=consts.ApiNpcPropMode.chase)
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_module_proj_stats.dps.one() == [0, approx(255), 0, 0]
    assert api_module_proj_stats.volley.one() == [0, approx(5100), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, approx(255), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, approx(5100), 0, 0]
    # Action
    api_tgt_drone.change_drone(prop_mode=consts.ApiNpcPropMode.cruise)
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_module_proj_stats.dps.one() == [0, approx(51), 0, 0]
    assert api_module_proj_stats.volley.one() == [0, approx(1020), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, approx(51), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, approx(1020), 0, 0]


def test_lance_attr_range_absent(client, consts):
    # No optimal range defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance_debuff(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(0, 25500, 0, 0),
        cycle_time=300000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        dmg_radius=100)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=1613)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=250, sig_radius=296)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 1362, 0), movement=(0, 0, 0))
    # Verification
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
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 1364, 0))
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, approx(1275), 0, 0]
    assert api_module_proj_stats.volley.one() == [0, approx(25500), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, approx(1275), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, approx(25500), 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 1862, 0))
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, approx(1275), 0, 0]
    assert api_module_proj_stats.volley.one() == [0, approx(25500), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, approx(1275), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, approx(25500), 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 1864, 0))
    # Verification
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


def test_lance_attr_radius_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance_debuff(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(0, 25500, 0, 0),
        cycle_time=300000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        range_optimal=100000)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=1613)
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
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, approx(1275), 0, 0]
    assert api_module_proj_stats.volley.one() == [0, approx(25500), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, approx(1275), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, approx(25500), 0, 0]


def test_lance_ship_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance_debuff(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(0, 25500, 0, 0),
        cycle_time=300000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        range_optimal=100000,
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
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, approx(1275), 0, 0]
    assert api_module_proj_stats.volley.one() == [0, approx(25500), 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, approx(1275), 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, approx(25500), 0, 0]


def test_tgt_attr_sig_radius_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance_debuff(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(0, 25500, 0, 0),
        cycle_time=300000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        range_optimal=100000,
        dmg_radius=100)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=1613)
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


def test_tgt_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_lance_debuff(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(0, 25500, 0, 0),
        cycle_time=300000,
        delay=15000,
        dmg_interval=1000,
        dmg_duration=15000,
        range_optimal=100000,
        dmg_radius=100)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=1613)
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
