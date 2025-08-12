from tests import approx
from tests.fw.api import (
    FitStatsOptions,
    ItemStatsOptions,
    StatRemoteRpsItemKinds,
    StatsOptionFitRemoteRps,
    StatsOptionItemRemoteRps,
)
from tests.tests.stats.tank import make_eve_drone_shield, make_eve_remote_asb, make_eve_remote_sb, setup_tank_basics


def test_state(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_normal_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=508, cycle_time=8000)
    eve_module_ancil_id = make_eve_remote_asb(client=client, basic_info=eve_basic_info, rep_amount=950, cycle_time=8000)
    eve_drone_id = make_eve_drone_shield(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module_normal = api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_module_ancil = api_fit.add_module(type_id=eve_module_ancil_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().shield == approx(196.65)
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_normal_stats.remote_rps.one().shield == approx(63.5)
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_ancil_stats.remote_rps.one().shield == approx(118.75)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps.one().shield == approx(14.4)
    # Action
    api_module_normal.change_module(state=consts.ApiModuleState.online)
    api_module_ancil.change_module(state=consts.ApiModuleState.online)
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().shield == 0
    api_stat_options = [StatsOptionItemRemoteRps(ignore_state=False), StatsOptionItemRemoteRps(ignore_state=True)]
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(remote_rps=(True, api_stat_options)))
    assert api_module_normal_stats.remote_rps.map(lambda i: i.shield) == [0, approx(63.5)]
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(remote_rps=(True, api_stat_options)))
    assert api_module_ancil_stats.remote_rps.map(lambda i: i.shield) == [0, approx(118.75)]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=(True, api_stat_options)))
    assert api_drone_stats.remote_rps.map(lambda i: i.shield) == [0, approx(14.4)]
    # Action
    api_module_normal.change_module(state=consts.ApiModuleState.active)
    api_module_ancil.change_module(state=consts.ApiModuleState.active)
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().shield == approx(196.65)
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_normal_stats.remote_rps.one().shield == approx(63.5)
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_ancil_stats.remote_rps.one().shield == approx(118.75)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps.one().shield == approx(14.4)


def test_item_kind(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_normal_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=508, cycle_time=8000)
    eve_module_ancil_id = make_eve_remote_asb(client=client, basic_info=eve_basic_info, rep_amount=950, cycle_time=8000)
    eve_drone_id = make_eve_drone_shield(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_fit.add_module(type_id=eve_module_ancil_id, state=consts.ApiModuleState.active)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=(True, [
        StatsOptionFitRemoteRps(),
        StatsOptionFitRemoteRps(item_kinds=StatRemoteRpsItemKinds(default=False, module=True)),
        StatsOptionFitRemoteRps(item_kinds=StatRemoteRpsItemKinds(default=False, minion=True))])))
    api_fit_rrps_default, api_fit_rrps_module, api_fit_rrps_minion = api_fit_stats.remote_rps
    assert api_fit_rrps_default.shield == approx(196.65)
    assert api_fit_rrps_module.shield == approx(182.25)
    assert api_fit_rrps_minion.shield == approx(14.4)


def test_zero_cycle_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_normal_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=508, cycle_time=0)
    eve_module_ancil_id = make_eve_remote_asb(client=client, basic_info=eve_basic_info, rep_amount=950, cycle_time=0)
    eve_drone_id = make_eve_drone_shield(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=0)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module_normal = api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_module_ancil = api_fit.add_module(type_id=eve_module_ancil_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().shield == 0
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_normal_stats.remote_rps.one().shield == 0
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_ancil_stats.remote_rps.one().shield == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps.one().shield == 0


def test_no_cycle_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts, effect_duration=False)
    eve_module_normal_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=508, cycle_time=8000)
    eve_module_ancil_id = make_eve_remote_asb(client=client, basic_info=eve_basic_info, rep_amount=950, cycle_time=8000)
    eve_drone_id = make_eve_drone_shield(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module_normal = api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_module_ancil = api_fit.add_module(type_id=eve_module_ancil_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().shield == 0
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_normal_stats.remote_rps.one().shield == 0
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_ancil_stats.remote_rps.one().shield == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps.one().shield == 0


def test_item_not_loaded(client, consts):
    setup_tank_basics(client=client, consts=consts)
    eve_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_stats.remote_rps.one().shield == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_stats.remote_rps is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps is None
