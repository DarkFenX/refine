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
from tests.stats.tank import make_eve_drone_hull, make_eve_remote_hr, make_eve_tankable, setup_tank_basics


def test_state(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=60, cycle_time=24000)
    eve_drone_id = make_eve_drone_hull(client=client, basic_info=eve_basic_info, rep_amount=36, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().hull == approx(9.7)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().hull == approx(9.7)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_stats.outgoing_rps.one().hull == approx(2.5)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps.one().hull == approx(7.2)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().hull == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().hull == 0
    api_stat_options = [StatsOptionItemOutRps(ignore_state=False), StatsOptionItemOutRps(ignore_state=True)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_rps=(True, api_stat_options)))
    assert api_module_stats.outgoing_rps.map(lambda i: i.hull) == [0, approx(2.5)]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=(True, api_stat_options)))
    assert api_drone_stats.outgoing_rps.map(lambda i: i.hull) == [0, approx(7.2)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().hull == approx(9.7)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().hull == approx(9.7)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_stats.outgoing_rps.one().hull == approx(2.5)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps.one().hull == approx(7.2)


def test_hp_limit_and_resist(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship1_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 50), rr_resist=0.5)
    eve_ship2_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 50), rr_resist=0.3)
    eve_module_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=115, cycle_time=6000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship1_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_rps.one().hull == approx(8.333333)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_rps.one().hull == approx(8.333333)
    api_src_module_stats = api_src_module.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_stats.outgoing_rps.one().hull == approx(8.333333)
    # Action
    api_tgt_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_rps.one().hull == approx(5.75)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_rps.one().hull == approx(5.75)
    api_src_module_stats = api_src_module.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_stats.outgoing_rps.one().hull == approx(5.75)


def test_hp_limit_and_range(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_src_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, radius=150)
    eve_tgt_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 50), radius=120)
    eve_module_id = make_eve_remote_hr(
        client=client, basic_info=eve_basic_info,
        rep_amount=115, cycle_time=6000, optimal_range=10500, falloff_range=6000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(16770, 0, 0))
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    # Verification - range is close enough to be limited by HP
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_rps.one().hull == approx(8.333333)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_rps.one().hull == approx(8.333333)
    api_src_module_stats = api_src_module.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_stats.outgoing_rps.one().hull == approx(8.333333)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 22770, 0))
    # Verification - range is far enough not to be limited by range
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_rps.one().hull == approx(1.197917)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_rps.one().hull == approx(1.197917)
    api_src_module_stats = api_src_module.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_stats.outgoing_rps.one().hull == approx(1.197917)


def test_item_kind(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=60, cycle_time=24000)
    eve_drone_id = make_eve_drone_hull(client=client, basic_info=eve_basic_info, rep_amount=36, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(),
        StatsOptionFitOutRps(item_kinds=StatOutRepItemKinds(default=False, module=True)),
        StatsOptionFitOutRps(item_kinds=StatOutRepItemKinds(default=False, minion=True))])))
    api_fleet_rrps_default, api_fleet_rrps_module, api_fleet_rrps_minion = api_fleet_stats.outgoing_rps
    assert api_fleet_rrps_default.hull == approx(9.7)
    assert api_fleet_rrps_module.hull == approx(2.5)
    assert api_fleet_rrps_minion.hull == approx(7.2)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(),
        StatsOptionFitOutRps(item_kinds=StatOutRepItemKinds(default=False, module=True)),
        StatsOptionFitOutRps(item_kinds=StatOutRepItemKinds(default=False, minion=True))])))
    api_fit_rrps_default, api_fit_rrps_module, api_fit_rrps_minion = api_fit_stats.outgoing_rps
    assert api_fit_rrps_default.hull == approx(9.7)
    assert api_fit_rrps_module.hull == approx(2.5)
    assert api_fit_rrps_minion.hull == approx(7.2)


def test_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=60, cycle_time=24000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification - burst stats
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeBurst())])))
    assert api_fleet_stats.outgoing_rps.one().hull == approx(2.5)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeBurst())])))
    assert api_fit_stats.outgoing_rps.one().hull == approx(2.5)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeBurst())])))
    assert api_module_stats.outgoing_rps.one().hull == approx(2.5)
    # Sim without specified time - looped stats
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeSim(time=None))])))
    assert api_fleet_stats.outgoing_rps.one().hull == approx(2.5)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeSim(time=None))])))
    assert api_fit_stats.outgoing_rps.one().hull == approx(2.5)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeSim(time=None))])))
    assert api_module_stats.outgoing_rps.one().hull == approx(2.5)
    # Sim with time before first cycle is completed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeSim(time=23))])))
    assert api_fleet_stats.outgoing_rps.one().hull == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeSim(time=23))])))
    assert api_fit_stats.outgoing_rps.one().hull == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeSim(time=23))])))
    assert api_module_stats.outgoing_rps.one().hull == 0
    # Sim with time just after first cycle is completed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeSim(time=25))])))
    assert api_fleet_stats.outgoing_rps.one().hull == approx(2.4)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeSim(time=25))])))
    assert api_fit_stats.outgoing_rps.one().hull == approx(2.4)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeSim(time=25))])))
    assert api_module_stats.outgoing_rps.one().hull == approx(2.4)


def test_zero_cycle_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=60, cycle_time=0)
    eve_drone_id = make_eve_drone_hull(client=client, basic_info=eve_basic_info, rep_amount=36, cycle_time=0)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().hull == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().hull == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_stats.outgoing_rps.one().hull == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps.one().hull == 0


def test_no_cycle_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts, effect_duration=False)
    eve_module_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=60, cycle_time=24000)
    eve_drone_id = make_eve_drone_hull(client=client, basic_info=eve_basic_info, rep_amount=36, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().hull == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().hull == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_stats.outgoing_rps.one().hull == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps.one().hull == 0


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
    assert api_fleet_stats.outgoing_rps.one().hull == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().hull == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_stats.outgoing_rps is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps is None
