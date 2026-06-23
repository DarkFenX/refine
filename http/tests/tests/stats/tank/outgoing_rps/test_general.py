from fw import approx, check_no_field
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionFitOutRps, StatsOptionItemOutRps
from tests.stats.tank import make_eve_remote_ar, make_eve_tankable, setup_tank_basics


def test_item_not_loaded(client, consts):
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().shield == 0
    assert api_fleet_stats.outgoing_rps.one().armor == 0
    assert api_fleet_stats.outgoing_rps.one().hull == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().shield == 0
    assert api_fit_stats.outgoing_rps.one().armor == 0
    assert api_fit_stats.outgoing_rps.one().hull == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_stats.outgoing_rps is None


def test_incorrect_item_kind(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_implant_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_implant_stats = api_implant.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_implant_stats.outgoing_rps is None


def test_incorrect_projectee(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
    eve_implant_id = client.mk_eve_item()
    eve_tgt_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(1000, 1000, 1000))
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_tmp = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_tgt_tmp.remove()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_implant = api_tgt_fit.add_implant(type_id=eve_implant_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    # Verification - specifying incorrect projectee item IDs should fail only that specific option,
    # not whole stat batch
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(projectee_item_id=api_tgt_tmp.id),
        StatsOptionFitOutRps(projectee_item_id=api_implant.id),
        StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_rps == [None, None, (0, approx(62.666667), 0)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(projectee_item_id=api_tgt_tmp.id),
        StatsOptionFitOutRps(projectee_item_id=api_implant.id),
        StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_rps == [None, None, (0, approx(62.666667), 0)]
    api_src_module_stats = api_src_module.get_stats(options=ItemStatsOptions(outgoing_rps=(True, [
        StatsOptionItemOutRps(projectee_item_id=api_tgt_tmp.id),
        StatsOptionItemOutRps(projectee_item_id=api_implant.id),
        StatsOptionItemOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_stats.outgoing_rps == [None, None, (0, approx(62.666667), 0)]


def test_not_requested(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=False))
    with check_no_field():
        api_fleet_stats.outgoing_rps  # noqa: B018
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=False))
    with check_no_field():
        api_fit_stats.outgoing_rps  # noqa: B018
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_rps=False))
    with check_no_field():
        api_module_stats.outgoing_rps  # noqa: B018
