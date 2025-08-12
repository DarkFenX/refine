from tests import approx
from tests.fw.api import (
    FitStatsOptions,
    ItemStatsOptions,
    StatRemoteRpsItemKinds,
    StatsOptionFitRemoteRps,
    StatsOptionItemRemoteRps,
)
from tests.tests.stats.tank import make_eve_drone_hull, make_eve_remote_hr, setup_tank_basics


def test_state(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=60, cycle_time=24000)
    eve_drone_id = make_eve_drone_hull(client=client, basic_info=eve_basic_info, rep_amount=36, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().hull == approx(9.7)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_stats.remote_rps.one().hull == approx(2.5)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps.one().hull == approx(7.2)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().hull == 0
    api_stat_options = [StatsOptionItemRemoteRps(ignore_state=False), StatsOptionItemRemoteRps(ignore_state=True)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_rps=(True, api_stat_options)))
    assert api_module_stats.remote_rps.map(lambda i: i.hull) == [0, approx(2.5)]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=(True, api_stat_options)))
    assert api_drone_stats.remote_rps.map(lambda i: i.hull) == [0, approx(7.2)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().hull == approx(9.7)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_stats.remote_rps.one().hull == approx(2.5)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps.one().hull == approx(7.2)


def test_item_kind(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=60, cycle_time=24000)
    eve_drone_id = make_eve_drone_hull(client=client, basic_info=eve_basic_info, rep_amount=36, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=(True, [
        StatsOptionFitRemoteRps(),
        StatsOptionFitRemoteRps(item_kinds=StatRemoteRpsItemKinds(default=False, module=True)),
        StatsOptionFitRemoteRps(item_kinds=StatRemoteRpsItemKinds(default=False, minion=True))])))
    api_fit_rrps_default, api_fit_rrps_module, api_fit_rrps_minion = api_fit_stats.remote_rps
    assert api_fit_rrps_default.hull == approx(9.7)
    assert api_fit_rrps_module.hull == approx(2.5)
    assert api_fit_rrps_minion.hull == approx(7.2)


def test_zero_cycle_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=60, cycle_time=0)
    eve_drone_id = make_eve_drone_hull(client=client, basic_info=eve_basic_info, rep_amount=36, cycle_time=0)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().hull == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_stats.remote_rps.one().hull == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps.one().hull == 0


def test_no_cycle_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts, effect_duration=False)
    eve_module_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=60, cycle_time=24000)
    eve_drone_id = make_eve_drone_hull(client=client, basic_info=eve_basic_info, rep_amount=36, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().hull == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_stats.remote_rps.one().hull == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps.one().hull == 0


def test_item_not_loaded(client, consts):
    setup_tank_basics(client=client, consts=consts)
    eve_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().hull == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_stats.remote_rps is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps is None
