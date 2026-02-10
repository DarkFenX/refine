from fw import approx
from fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatOutRepItemKinds,
    StatsOptionFitOutRps,
    StatsOptionItemOutRps,
    StatTimeBurst,
    StatTimeSim,
)
from tests.stats.tank import make_eve_drone_shield, make_eve_remote_asb, make_eve_remote_sb, setup_tank_basics


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
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().shield == approx(196.65)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().shield == approx(196.65)
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_normal_stats.outgoing_rps.one().shield == approx(63.5)
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_ancil_stats.outgoing_rps.one().shield == approx(118.75)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps.one().shield == approx(14.4)
    # Action
    api_module_normal.change_module(state=consts.ApiModuleState.online)
    api_module_ancil.change_module(state=consts.ApiModuleState.online)
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().shield == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().shield == 0
    api_stat_options = [StatsOptionItemOutRps(ignore_state=False), StatsOptionItemOutRps(ignore_state=True)]
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, api_stat_options)))
    assert api_module_normal_stats.outgoing_rps.map(lambda i: i.shield) == [0, approx(63.5)]
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=(True, api_stat_options)))
    assert api_module_ancil_stats.outgoing_rps.map(lambda i: i.shield) == [0, approx(118.75)]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=(True, api_stat_options)))
    assert api_drone_stats.outgoing_rps.map(lambda i: i.shield) == [0, approx(14.4)]
    # Action
    api_module_normal.change_module(state=consts.ApiModuleState.active)
    api_module_ancil.change_module(state=consts.ApiModuleState.active)
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().shield == approx(196.65)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().shield == approx(196.65)
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_normal_stats.outgoing_rps.one().shield == approx(63.5)
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_ancil_stats.outgoing_rps.one().shield == approx(118.75)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps.one().shield == approx(14.4)


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
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(),
        StatsOptionFitOutRps(item_kinds=StatOutRepItemKinds(default=False, module=True)),
        StatsOptionFitOutRps(item_kinds=StatOutRepItemKinds(default=False, minion=True))])))
    api_fleet_rrps_default, api_fleet_rrps_module, api_fleet_rrps_minion = api_fleet_stats.outgoing_rps
    assert api_fleet_rrps_default.shield == approx(196.65)
    assert api_fleet_rrps_module.shield == approx(182.25)
    assert api_fleet_rrps_minion.shield == approx(14.4)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(),
        StatsOptionFitOutRps(item_kinds=StatOutRepItemKinds(default=False, module=True)),
        StatsOptionFitOutRps(item_kinds=StatOutRepItemKinds(default=False, minion=True))])))
    api_fit_rrps_default, api_fit_rrps_module, api_fit_rrps_minion = api_fit_stats.outgoing_rps
    assert api_fit_rrps_default.shield == approx(196.65)
    assert api_fit_rrps_module.shield == approx(182.25)
    assert api_fit_rrps_minion.shield == approx(14.4)


def test_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_normal_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=508, cycle_time=8000)
    eve_module_ancil_id = make_eve_remote_asb(
        client=client, basic_info=eve_basic_info, rep_amount=950, cycle_time=8000, capacity=42, reload_time=60000)
    eve_charge_id = client.mk_eve_item(attrs={eve_basic_info.volume_attr_id: 4.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module_normal = api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_module_ancil = api_fit.add_module(
        type_id=eve_module_ancil_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification - burst stats
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeBurst())])))
    assert api_fleet_stats.outgoing_rps.one().shield == approx(182.25)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeBurst())])))
    assert api_fit_stats.outgoing_rps.one().shield == approx(182.25)
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeBurst())])))
    assert api_module_normal_stats.outgoing_rps.one().shield == approx(63.5)
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeBurst())])))
    assert api_module_ancil_stats.outgoing_rps.one().shield == approx(118.75)
    # Sim without specified time - looped stats
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionFitOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.on_empty)),
    ])))
    assert api_fleet_stats.outgoing_rps.map(lambda i: i.shield) == [approx(182.25), approx(128.272727)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionFitOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.on_empty)),
    ])))
    assert api_fit_stats.outgoing_rps.map(lambda i: i.shield) == [approx(182.25), approx(128.272727)]
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(outgoing_rps=(True, [
        StatsOptionItemOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.on_empty)),
    ])))
    assert api_module_normal_stats.outgoing_rps.map(lambda i: i.shield) == [approx(63.5), approx(63.5)]
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=(True, [
        StatsOptionItemOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.on_empty)),
    ])))
    assert api_module_ancil_stats.outgoing_rps.map(lambda i: i.shield) == [approx(118.75), approx(64.772727)]
    # Sim with time right after first cycle has started
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeSim(time=1))])))
    assert api_fleet_stats.outgoing_rps.one().shield == approx(1458)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeSim(time=1))])))
    assert api_fit_stats.outgoing_rps.one().shield == approx(1458)
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeSim(time=1))])))
    assert api_module_normal_stats.outgoing_rps.one().shield == approx(508)
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeSim(time=1))])))
    assert api_module_ancil_stats.outgoing_rps.one().shield == approx(950)
    # Sim with time right after second cycle has started
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeSim(time=9))])))
    assert api_fleet_stats.outgoing_rps.one().shield == approx(324)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeSim(time=9))])))
    assert api_fit_stats.outgoing_rps.one().shield == approx(324)
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeSim(time=9))])))
    assert api_module_normal_stats.outgoing_rps.one().shield == approx(112.888889)
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeSim(time=9))])))
    assert api_module_ancil_stats.outgoing_rps.one().shield == approx(211.111111)
    # Sim with time when ASB exhausted its clip
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(time_options=StatTimeSim(time=102, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionFitOutRps(time_options=StatTimeSim(time=102, optional_reloads=consts.ApiOptionalReload.on_empty)),
    ])))
    assert api_fleet_stats.outgoing_rps.map(lambda i: i.shield) == [approx(185.823529), approx(148.568627)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(time_options=StatTimeSim(time=102, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionFitOutRps(time_options=StatTimeSim(time=102, optional_reloads=consts.ApiOptionalReload.on_empty)),
    ])))
    assert api_fit_stats.outgoing_rps.map(lambda i: i.shield) == [approx(185.823529), approx(148.568627)]
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(outgoing_rps=(True, [
        StatsOptionItemOutRps(time_options=StatTimeSim(time=102, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=102, optional_reloads=consts.ApiOptionalReload.on_empty)),
    ])))
    assert api_module_normal_stats.outgoing_rps.map(lambda i: i.shield) == [approx(64.745098), approx(64.745098)]
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=(True, [
        StatsOptionItemOutRps(time_options=StatTimeSim(time=102, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=102, optional_reloads=consts.ApiOptionalReload.on_empty)),
    ])))
    assert api_module_ancil_stats.outgoing_rps.map(lambda i: i.shield) == [approx(121.078431), approx(83.823529)]



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
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().shield == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().shield == 0
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_normal_stats.outgoing_rps.one().shield == 0
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_ancil_stats.outgoing_rps.one().shield == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps.one().shield == 0


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
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().shield == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().shield == 0
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_normal_stats.outgoing_rps.one().shield == 0
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_ancil_stats.outgoing_rps.one().shield == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps.one().shield == 0


def test_item_not_loaded(client, consts):
    setup_tank_basics(client=client, consts=consts)
    eve_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().shield == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().shield == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_stats.outgoing_rps is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps is None
