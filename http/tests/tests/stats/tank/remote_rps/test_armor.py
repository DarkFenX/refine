from tests import Spool, approx
from tests.fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatRemoteRepItemKinds,
    StatsOptionFitRemoteRps,
    StatsOptionItemRemoteRps,
)
from tests.tests.stats.tank import (
    make_eve_drone_armor,
    make_eve_remote_aar,
    make_eve_remote_ar,
    make_eve_remote_sar,
    setup_tank_basics,
)


def test_state(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_normal_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
    eve_module_ancil_id = make_eve_remote_aar(client=client, basic_info=eve_basic_info, rep_amount=145, cycle_time=6000)
    eve_module_spool_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, spool_step=0.12, spool_max=1.8, cycle_time=6000)
    eve_drone_id = make_eve_drone_armor(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module_normal = api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_module_ancil = api_fit.add_module(
        type_id=eve_module_ancil_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_paste_id)
    api_module_spool = api_fit.add_module(
        type_id=eve_module_spool_id,
        state=consts.ApiModuleState.active,
        spool=Spool.spool_scale_to_api(val=1))
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_rps=True))
    assert api_fleet_stats.remote_rps.one().armor == approx(388.5)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().armor == approx(388.5)
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_normal_stats.remote_rps.one().armor == approx(62.666667)
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_ancil_stats.remote_rps.one().armor == approx(72.5)
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_spool_stats.remote_rps.one().armor == approx(238.933333)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps.one().armor == approx(14.4)
    # Action
    api_module_normal.change_module(state=consts.ApiModuleState.online)
    api_module_ancil.change_module(state=consts.ApiModuleState.online)
    api_module_spool.change_module(state=consts.ApiModuleState.online)
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_rps=True))
    assert api_fleet_stats.remote_rps.one().armor == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().armor == 0
    api_stat_options = [StatsOptionItemRemoteRps(ignore_state=False), StatsOptionItemRemoteRps(ignore_state=True)]
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(remote_rps=(True, api_stat_options)))
    assert api_module_normal_stats.remote_rps.map(lambda i: i.armor) == [0, approx(62.666667)]
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(remote_rps=(True, api_stat_options)))
    assert api_module_ancil_stats.remote_rps.map(lambda i: i.armor) == [0, approx(72.5)]
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(remote_rps=(True, api_stat_options)))
    assert api_module_spool_stats.remote_rps.map(lambda i: i.armor) == [0, approx(238.933333)]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=(True, api_stat_options)))
    assert api_drone_stats.remote_rps.map(lambda i: i.armor) == [0, approx(14.4)]
    # Action
    api_module_normal.change_module(state=consts.ApiModuleState.active)
    api_module_ancil.change_module(state=consts.ApiModuleState.active)
    api_module_spool.change_module(state=consts.ApiModuleState.active)
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_rps=True))
    assert api_fleet_stats.remote_rps.one().armor == approx(388.5)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().armor == approx(388.5)
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_normal_stats.remote_rps.one().armor == approx(62.666667)
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_ancil_stats.remote_rps.one().armor == approx(72.5)
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_spool_stats.remote_rps.one().armor == approx(238.933333)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps.one().armor == approx(14.4)


def test_item_kind(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_normal_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
    eve_module_ancil_id = make_eve_remote_aar(client=client, basic_info=eve_basic_info, rep_amount=145, cycle_time=6000)
    eve_module_spool_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, spool_step=0.12, spool_max=1.8, cycle_time=6000)
    eve_drone_id = make_eve_drone_armor(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_fit.add_module(type_id=eve_module_ancil_id, state=consts.ApiModuleState.active, charge_type_id=eve_paste_id)
    api_fit.add_module(
        type_id=eve_module_spool_id,
        state=consts.ApiModuleState.active,
        spool=Spool.spool_scale_to_api(val=1))
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_rps=(True, [
        StatsOptionFitRemoteRps(),
        StatsOptionFitRemoteRps(item_kinds=StatRemoteRepItemKinds(default=False, module=True)),
        StatsOptionFitRemoteRps(item_kinds=StatRemoteRepItemKinds(default=False, minion=True))])))
    api_fleet_rrps_default, api_fleet_rrps_module, api_fleet_rrps_minion = api_fleet_stats.remote_rps
    assert api_fleet_rrps_default.armor == approx(388.5)
    assert api_fleet_rrps_module.armor == approx(374.1)
    assert api_fleet_rrps_minion.armor == approx(14.4)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=(True, [
        StatsOptionFitRemoteRps(),
        StatsOptionFitRemoteRps(item_kinds=StatRemoteRepItemKinds(default=False, module=True)),
        StatsOptionFitRemoteRps(item_kinds=StatRemoteRepItemKinds(default=False, minion=True))])))
    api_fit_rrps_default, api_fit_rrps_module, api_fit_rrps_minion = api_fit_stats.remote_rps
    assert api_fit_rrps_default.armor == approx(388.5)
    assert api_fit_rrps_module.armor == approx(374.1)
    assert api_fit_rrps_minion.armor == approx(14.4)


def test_spool(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_spool_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, spool_step=0.12, spool_max=1.8, cycle_time=6000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_spool_id,
        state=consts.ApiModuleState.active,
        spool=Spool.spool_scale_to_api(val=0.5))
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_rps=(True, [
        StatsOptionFitRemoteRps(spool=Spool.spool_scale_to_api(val=0)),
        StatsOptionFitRemoteRps(),
        StatsOptionFitRemoteRps(spool=Spool.spool_scale_to_api(val=1))])))
    assert api_fleet_stats.remote_rps.map(lambda i: i.armor) == [
        approx(85.333333),
        approx(167.253333),
        approx(238.933333)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=(True, [
        StatsOptionFitRemoteRps(spool=Spool.spool_scale_to_api(val=0)),
        StatsOptionFitRemoteRps(),
        StatsOptionFitRemoteRps(spool=Spool.spool_scale_to_api(val=1))])))
    assert api_fit_stats.remote_rps.map(lambda i: i.armor) == [
        approx(85.333333),
        approx(167.253333),
        approx(238.933333)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_rps=(True, [
        StatsOptionItemRemoteRps(spool=Spool.spool_scale_to_api(val=0)),
        StatsOptionItemRemoteRps(),
        StatsOptionItemRemoteRps(spool=Spool.spool_scale_to_api(val=1))])))
    assert api_module_stats.remote_rps.map(lambda i: i.armor) == [
        approx(85.333333),
        approx(167.253333),
        approx(238.933333)]


def test_zero_cycle_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_normal_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=0)
    eve_module_ancil_id = make_eve_remote_aar(client=client, basic_info=eve_basic_info, rep_amount=145, cycle_time=0)
    eve_module_spool_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, spool_step=0.12, spool_max=1.8, cycle_time=0)
    eve_drone_id = make_eve_drone_armor(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=0)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module_normal = api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_module_ancil = api_fit.add_module(
        type_id=eve_module_ancil_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_paste_id)
    api_module_spool = api_fit.add_module(
        type_id=eve_module_spool_id,
        state=consts.ApiModuleState.active,
        spool=Spool.spool_scale_to_api(val=1))
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_rps=True))
    assert api_fleet_stats.remote_rps.one().armor == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().armor == 0
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_normal_stats.remote_rps.one().armor == 0
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_ancil_stats.remote_rps.one().armor == 0
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_spool_stats.remote_rps.one().armor == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps.one().armor == 0


def test_no_cycle_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts, effect_duration=False)
    eve_module_normal_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
    eve_module_ancil_id = make_eve_remote_aar(client=client, basic_info=eve_basic_info, rep_amount=145, cycle_time=6000)
    eve_module_spool_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, spool_step=0.12, spool_max=1.8, cycle_time=6000)
    eve_drone_id = make_eve_drone_armor(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module_normal = api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_module_ancil = api_fit.add_module(
        type_id=eve_module_ancil_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_paste_id)
    api_module_spool = api_fit.add_module(
        type_id=eve_module_spool_id,
        state=consts.ApiModuleState.active,
        spool=Spool.spool_scale_to_api(val=1))
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_rps=True))
    assert api_fleet_stats.remote_rps.one().armor == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().armor == 0
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_normal_stats.remote_rps.one().armor == 0
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_ancil_stats.remote_rps.one().armor == 0
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_spool_stats.remote_rps.one().armor == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps.one().armor == 0


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
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_rps=True))
    assert api_fleet_stats.remote_rps.one().armor == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_rps=True))
    assert api_fit_stats.remote_rps.one().armor == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_module_stats.remote_rps is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_rps=True))
    assert api_drone_stats.remote_rps is None
