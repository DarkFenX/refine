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
from tests.tests.stats.dmg import make_eve_launcher, make_eve_missile_fof, make_eve_ship, setup_dmg_basics


def test_range_below_max(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=0.78, cycle_time=2500, reload_time=35000)
    eve_charge_id = make_eve_missile_fof(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 830), volume=0.03,
        speed=6500, flight_time=9750, mass=1000, agility=0.00015, max_range=100000,
        exp_radius=99.75, exp_speed=127.575, drf=0.682)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=800)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=1000)
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
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 61490, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, 0, approx(664)]
    assert api_fleet_stats.volley.one() == [0, 0, 0, approx(1660)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, 0, approx(664)]
    assert api_src_fit_stats.volley.one() == [0, 0, 0, approx(1660)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, approx(332)]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, approx(830)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, approx(332)]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, approx(830)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 61510, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, 0, approx(579.723077)]
    assert api_fleet_stats.volley.one() == [0, 0, 0, approx(1449.307692)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, 0, approx(579.723077)]
    assert api_src_fit_stats.volley.one() == [0, 0, 0, approx(1449.307692)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, approx(289.861538)]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, approx(724.653846)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, approx(289.861538)]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, approx(724.653846)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 68010, 0))
    # Verification
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
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_range_above_max(client, consts):
    # FoF missile is limited by c2s range. Tested on 2025-08-12 on Thunderdome, using civilian LML
    # Minokawa (3k radius) with HG hydra + MGCs + hydraulics vs chremoas and dagon at 96900 and
    # 97100 overview range
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=0.78, cycle_time=2500, reload_time=35000)
    eve_charge_id = make_eve_missile_fof(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 830), volume=0.03,
        speed=8300, flight_time=16800, mass=1000, agility=0.00015, max_range=100000,
        exp_radius=99.75, exp_speed=127.575, drf=0.682)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=800)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=1000)
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
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 102999, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, 0, approx(664)]
    assert api_fleet_stats.volley.one() == [0, 0, 0, approx(1660)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, 0, approx(664)]
    assert api_src_fit_stats.volley.one() == [0, 0, 0, approx(1660)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, approx(332)]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, approx(830)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, approx(332)]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, approx(830)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 103001, 0))
    # Verification
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
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_application(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=0.78, cycle_time=2500, reload_time=35000)
    eve_charge_id = make_eve_missile_fof(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 830), volume=0.03,
        speed=6500, flight_time=9750, mass=1000, agility=0.00015, max_range=100000,
        exp_radius=99.75, exp_speed=127.575, drf=0.682)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=800)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=50)
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
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, 0, approx(332.83208)]
    assert api_fleet_stats.volley.one() == [0, 0, 0, approx(832.0802)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, 0, approx(332.83208)]
    assert api_src_fit_stats.volley.one() == [0, 0, 0, approx(832.0802)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, approx(166.41604)]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, approx(416.0401)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, approx(166.41604)]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, approx(416.0401)]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 0.5))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, 0, approx(163.320839)]
    assert api_fleet_stats.volley.one() == [0, 0, 0, approx(408.302097)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, 0, approx(163.320839)]
    assert api_src_fit_stats.volley.one() == [0, 0, 0, approx(408.302097)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, approx(81.660419)]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, approx(204.151049)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, approx(81.660419)]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, approx(204.151049)]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, 0, approx(101.797976)]
    assert api_fleet_stats.volley.one() == [0, 0, 0, approx(254.494941)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, 0, approx(101.797976)]
    assert api_src_fit_stats.volley.one() == [0, 0, 0, approx(254.494941)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, approx(50.898988)]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, approx(127.247471)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, approx(50.898988)]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, approx(127.247471)]
