from tests import ANY_VALUE, approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions
from tests.tests.stats.tank import (
    make_eve_drone_hull,
    make_eve_local_hr,
    make_eve_remote_hr,
    make_eve_tankable,
    setup_tank_basics,
)


def test_state_ship(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 1000))
    eve_module_lhr_id = make_eve_local_hr(client=client, basic_info=eve_basic_info, rep_amount=120, cycle_time=24000)
    eve_module_rhr_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=230, cycle_time=6000)
    eve_drone_id = make_eve_drone_hull(client=client, basic_info=eve_basic_info, rep_amount=36, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rhr = api_src_fit.add_module(type_id=eve_module_rhr_id, state=consts.ApiModuleState.active)
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_module_lhr = api_tgt_fit.add_module(type_id=eve_module_lhr_id, state=consts.ApiModuleState.active)
    api_module_rhr.change_module(add_projs=[api_tgt_ship.id])
    api_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [approx(5), approx(45.533333), ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [approx(5), approx(45.533333), ANY_VALUE]
    # Action
    api_module_lhr.change_module(state=consts.ApiModuleState.online)
    api_module_rhr.change_module(state=consts.ApiModuleState.online)
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [0, 0, ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [0, 0, ANY_VALUE]
    # Action
    api_module_lhr.change_module(state=consts.ApiModuleState.active)
    api_module_rhr.change_module(state=consts.ApiModuleState.active)
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [approx(5), approx(45.533333), ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [approx(5), approx(45.533333), ANY_VALUE]


def test_drone(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_tgt_drone_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 1000))
    eve_module_lhr_id = make_eve_local_hr(client=client, basic_info=eve_basic_info, rep_amount=120, cycle_time=24000)
    eve_module_rhr_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=230, cycle_time=6000)
    eve_src_drone_id = make_eve_drone_hull(client=client, basic_info=eve_basic_info, rep_amount=36, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rhr = api_src_fit.add_module(type_id=eve_module_rhr_id, state=consts.ApiModuleState.active)
    api_src_drone = api_src_fit.add_drone(type_id=eve_src_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_fit.add_module(type_id=eve_module_lhr_id, state=consts.ApiModuleState.active)
    api_tgt_drone = api_tgt_fit.add_drone(type_id=eve_tgt_drone_id)
    api_module_rhr.change_module(add_projs=[api_tgt_drone.id])
    api_src_drone.change_drone(add_projs=[api_tgt_drone.id])
    # Verification - local reps do not affect drones
    api_tgt_drone_stats = api_tgt_drone.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_drone_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_drone_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_drone_stats.rps.one().hull == [0, approx(45.533333), ANY_VALUE]


def test_hp_limit_and_resist(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 30), rr_resist=0.5)
    eve_module_lhr_id = make_eve_local_hr(client=client, basic_info=eve_basic_info, rep_amount=50, cycle_time=24000)
    eve_module_rhr_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=230, cycle_time=6000)
    eve_drone_id = make_eve_drone_hull(client=client, basic_info=eve_basic_info, rep_amount=36, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rhr = api_src_fit.add_module(type_id=eve_module_rhr_id, state=consts.ApiModuleState.active)
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_module_lhr_id, state=consts.ApiModuleState.active)
    api_module_rhr.change_module(add_projs=[api_tgt_ship.id])
    api_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification - local rep is not resisted but limited, remote hull rep is resisted and limited,
    # hull bot is resisted but not limited
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [approx(1.25), approx(8.6), ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [approx(1.25), approx(8.6), ANY_VALUE]


def test_hp_limit_and_range(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 100))
    eve_module_lhr_id = make_eve_local_hr(client=client, basic_info=eve_basic_info, rep_amount=120, cycle_time=24000)
    eve_module_rhr_id = make_eve_remote_hr(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=230,
        cycle_time=6000,
        optimal_range=10000,
        falloff_range=5000)
    eve_drone_id = make_eve_drone_hull(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=36,
        cycle_time=5000,
        optimal_range=10000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_module_rhr = api_src_fit.add_module(type_id=eve_module_rhr_id, state=consts.ApiModuleState.active)
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id, coordinates=(15000, 0, 0))
    api_tgt_fit.add_module(type_id=eve_module_lhr_id, state=consts.ApiModuleState.active)
    api_module_rhr.change_module(add_projs=[api_tgt_ship.id])
    api_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification - local hull rep is limited, remote hull rep is reduced and limited, hull bot is
    # reduced to 0 by range, so not limited
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [approx(4.166667), approx(16.666667), ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [approx(4.166667), approx(16.666667), ANY_VALUE]


def test_zero_cycle_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 1000))
    eve_module_lhr_id = make_eve_local_hr(client=client, basic_info=eve_basic_info, rep_amount=120, cycle_time=0)
    eve_module_rhr_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=230, cycle_time=0)
    eve_drone_id = make_eve_drone_hull(client=client, basic_info=eve_basic_info, rep_amount=36, cycle_time=0)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rhr = api_src_fit.add_module(type_id=eve_module_rhr_id, state=consts.ApiModuleState.active)
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_module_lhr_id, state=consts.ApiModuleState.active)
    api_module_rhr.change_module(add_projs=[api_tgt_ship.id])
    api_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [0, 0, ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [0, 0, ANY_VALUE]


def test_no_cycle_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts, effect_duration=False)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 1000))
    eve_module_lhr_id = make_eve_local_hr(client=client, basic_info=eve_basic_info, rep_amount=120, cycle_time=24000)
    eve_module_rhr_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=230, cycle_time=6000)
    eve_drone_id = make_eve_drone_hull(client=client, basic_info=eve_basic_info, rep_amount=36, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rhr = api_src_fit.add_module(type_id=eve_module_rhr_id, state=consts.ApiModuleState.active)
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_module_lhr_id, state=consts.ApiModuleState.active)
    api_module_rhr.change_module(add_projs=[api_tgt_ship.id])
    api_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [0, 0, ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, 0, ANY_VALUE, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [0, 0, ANY_VALUE]
