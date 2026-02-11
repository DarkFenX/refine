from fw import Spool, approx
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
from tests.stats.tank import (
    make_eve_drone_armor,
    make_eve_remote_aar,
    make_eve_remote_ar,
    make_eve_remote_sar,
    make_eve_tankable,
    setup_tank_basics,
)


def test_state(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_normal_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
    eve_module_ancil_id = make_eve_remote_aar(
        client=client, basic_info=eve_basic_info, rep_amount=145, cycle_time=6000, capacity=0.32, charge_rate=4)
    eve_module_spool_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, spool_step=0.12, spool_max=1.8, cycle_time=6000)
    eve_drone_id = make_eve_drone_armor(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    eve_paste_id = client.mk_eve_item(
        id_=consts.EveItem.nanite_repair_paste,
        attrs={eve_basic_info.volume_attr_id: 0.01})
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
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().armor == approx(388.5)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().armor == approx(388.5)
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_normal_stats.outgoing_rps.one().armor == approx(62.666667)
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_ancil_stats.outgoing_rps.one().armor == approx(72.5)
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_spool_stats.outgoing_rps.one().armor == approx(238.933333)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps.one().armor == approx(14.4)
    # Action
    api_module_normal.change_module(state=consts.ApiModuleState.online)
    api_module_ancil.change_module(state=consts.ApiModuleState.online)
    api_module_spool.change_module(state=consts.ApiModuleState.online)
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().armor == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().armor == 0
    api_stat_options = [StatsOptionItemOutRps(ignore_state=False), StatsOptionItemOutRps(ignore_state=True)]
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, api_stat_options)))
    assert api_module_normal_stats.outgoing_rps.map(lambda i: i.armor) == [0, approx(62.666667)]
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=(True, api_stat_options)))
    assert api_module_ancil_stats.outgoing_rps.map(lambda i: i.armor) == [0, approx(72.5)]
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(outgoing_rps=(True, api_stat_options)))
    assert api_module_spool_stats.outgoing_rps.map(lambda i: i.armor) == [0, approx(238.933333)]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=(True, api_stat_options)))
    assert api_drone_stats.outgoing_rps.map(lambda i: i.armor) == [0, approx(14.4)]
    # Action
    api_module_normal.change_module(state=consts.ApiModuleState.active)
    api_module_ancil.change_module(state=consts.ApiModuleState.active)
    api_module_spool.change_module(state=consts.ApiModuleState.active)
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().armor == approx(388.5)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().armor == approx(388.5)
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_normal_stats.outgoing_rps.one().armor == approx(62.666667)
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_ancil_stats.outgoing_rps.one().armor == approx(72.5)
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_spool_stats.outgoing_rps.one().armor == approx(238.933333)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps.one().armor == approx(14.4)


def test_hp_limit_and_resist(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship1_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 150, 1000), rr_resist=0.5)
    eve_ship2_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 150, 1000), rr_resist=0.3)
    eve_module_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
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
    assert api_fleet_stats.outgoing_rps.one().armor == approx(25)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_rps.one().armor == approx(25)
    api_src_module_stats = api_src_module.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_stats.outgoing_rps.one().armor == approx(25)
    # Action
    api_tgt_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_rps.one().armor == approx(18.8)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_rps.one().armor == approx(18.8)
    api_src_module_stats = api_src_module.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_stats.outgoing_rps.one().armor == approx(18.8)


def test_hp_limit_and_range(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_src_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, radius=150)
    eve_tgt_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 150, 1000), radius=120)
    eve_module_id = make_eve_remote_ar(
        client=client, basic_info=eve_basic_info,
        rep_amount=376, cycle_time=6000, optimal_range=12688, falloff_range=3625)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 16583, 0))
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    # Verification - range is close enough to be limited by HP
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_rps.one().armor == approx(25)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_rps.one().armor == approx(25)
    api_src_module_stats = api_src_module.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_stats.outgoing_rps.one().armor == approx(25)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 20208, 0))
    # Verification - range is far enough not to be limited by range
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_rps.one().armor == approx(3.916667)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_rps.one().armor == approx(3.916667)
    api_src_module_stats = api_src_module.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_stats.outgoing_rps.one().armor == approx(3.916667)


def test_hp_limit_and_time_burst_spool(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1300, 1000))
    eve_module_spool_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, spool_step=0.12, spool_max=1.8, cycle_time=6000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(
        type_id=eve_module_spool_id,
        state=consts.ApiModuleState.active,
        spool=Spool.spool_scale_to_api(val=0.5))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    # Verification - limited by HP at max spool (1433.6 > 1300)
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(
            time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0)), projectee_item_id=api_tgt_ship.id),
        StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id),
        StatsOptionFitOutRps(
            time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)), projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_rps.map(lambda i: i.armor) == [
        approx(85.333333), approx(167.253333), approx(216.666667)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(
            time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0)), projectee_item_id=api_tgt_ship.id),
        StatsOptionFitOutRps(projectee_item_id=api_tgt_ship.id),
        StatsOptionFitOutRps(
            time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)), projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_rps.map(lambda i: i.armor) == [
        approx(85.333333), approx(167.253333), approx(216.666667)]
    api_src_module_stats = api_src_module.get_stats(options=ItemStatsOptions(outgoing_rps=(True, [
        StatsOptionItemOutRps(
            time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0)), projectee_item_id=api_tgt_ship.id),
        StatsOptionItemOutRps(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemOutRps(
            time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)), projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_stats.outgoing_rps.map(lambda i: i.armor) == [
        approx(85.333333), approx(167.253333), approx(216.666667)]


def test_item_kind(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_normal_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
    eve_module_ancil_id = make_eve_remote_aar(
        client=client, basic_info=eve_basic_info, rep_amount=145, cycle_time=6000, capacity=0.32, charge_rate=4)
    eve_module_spool_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, spool_step=0.12, spool_max=1.8, cycle_time=6000)
    eve_drone_id = make_eve_drone_armor(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    eve_paste_id = client.mk_eve_item(
        id_=consts.EveItem.nanite_repair_paste,
        attrs={eve_basic_info.volume_attr_id: 0.01})
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
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(),
        StatsOptionFitOutRps(item_kinds=StatOutRepItemKinds(default=False, module=True)),
        StatsOptionFitOutRps(item_kinds=StatOutRepItemKinds(default=False, minion=True))])))
    api_fleet_rrps_default, api_fleet_rrps_module, api_fleet_rrps_minion = api_fleet_stats.outgoing_rps
    assert api_fleet_rrps_default.armor == approx(388.5)
    assert api_fleet_rrps_module.armor == approx(374.1)
    assert api_fleet_rrps_minion.armor == approx(14.4)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(),
        StatsOptionFitOutRps(item_kinds=StatOutRepItemKinds(default=False, module=True)),
        StatsOptionFitOutRps(item_kinds=StatOutRepItemKinds(default=False, minion=True))])))
    api_fit_rrps_default, api_fit_rrps_module, api_fit_rrps_minion = api_fit_stats.outgoing_rps
    assert api_fit_rrps_default.armor == approx(388.5)
    assert api_fit_rrps_module.armor == approx(374.1)
    assert api_fit_rrps_minion.armor == approx(14.4)


def test_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_module_normal_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
    eve_module_ancil_id = make_eve_remote_aar(
        client=client, basic_info=eve_basic_info,
        rep_amount=145, cycle_time=6000, capacity=0.32, charge_rate=4, reload_time=60000)
    eve_module_spool_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, spool_step=0.12, spool_max=1.8, cycle_time=6000)
    eve_paste_id = client.mk_eve_item(
        id_=consts.EveItem.nanite_repair_paste,
        attrs={eve_basic_info.volume_attr_id: 0.01})
    client.create_sources()
    api_sol = client.create_sol(default_spool=Spool.spool_scale_to_api(val=0.5))
    api_fit = api_sol.create_fit()
    api_module_normal = api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_module_ancil = api_fit.add_module(
        type_id=eve_module_ancil_id, state=consts.ApiModuleState.active, charge_type_id=eve_paste_id)
    api_module_spool = api_fit.add_module(type_id=eve_module_spool_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification - burst stats. For spool rep, on-sol value is taken, since neither on-module
    # value nor stats request override it.
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeBurst())])))
    assert api_fleet_stats.outgoing_rps.one().armor == approx(302.42)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeBurst())])))
    assert api_fit_stats.outgoing_rps.one().armor == approx(302.42)
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeBurst())])))
    assert api_module_normal_stats.outgoing_rps.one().armor == approx(62.666667)
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeBurst())])))
    assert api_module_ancil_stats.outgoing_rps.one().armor == approx(72.5)
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeBurst())])))
    assert api_module_spool_stats.outgoing_rps.one().armor == approx(167.253333)
    # Sim without specified time - looped stats. Spool value is ignored and just max spool is taken
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionFitOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.on_empty)),
    ])))
    assert api_fleet_stats.outgoing_rps.map(lambda i: i.armor) == [approx(325.766667), approx(333.822222)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionFitOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.on_empty)),
    ])))
    assert api_fit_stats.outgoing_rps.map(lambda i: i.armor) == [approx(325.766667), approx(333.822222)]
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(outgoing_rps=(True, [
        StatsOptionItemOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.on_empty)),
    ])))
    assert api_module_normal_stats.outgoing_rps.map(lambda i: i.armor) == [approx(62.666667), approx(62.666667)]
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=(True, [
        StatsOptionItemOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.on_empty)),
    ])))
    assert api_module_ancil_stats.outgoing_rps.map(lambda i: i.armor) == [approx(24.166667), approx(32.222222)]
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(outgoing_rps=(True, [
        StatsOptionItemOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.on_empty)),
    ])))
    assert api_module_spool_stats.outgoing_rps.map(lambda i: i.armor) == [approx(238.933333), approx(238.933333)]
    # Sim with time before any of rep cycles complete
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeSim(time=5))])))
    assert api_fleet_stats.outgoing_rps.one().armor == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeSim(time=5))])))
    assert api_fit_stats.outgoing_rps.one().armor == 0
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeSim(time=5))])))
    assert api_module_normal_stats.outgoing_rps.one().armor == 0
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeSim(time=5))])))
    assert api_module_ancil_stats.outgoing_rps.one().armor == 0
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeSim(time=5))])))
    assert api_module_spool_stats.outgoing_rps.one().armor == 0
    # Sim with time just after first rep cycle has completed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeSim(time=7))])))
    assert api_fleet_stats.outgoing_rps.one().armor == approx(189)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_rps=(True, [StatsOptionFitOutRps(time_options=StatTimeSim(time=7))])))
    assert api_fit_stats.outgoing_rps.one().armor == approx(189)
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeSim(time=75))])))
    assert api_module_normal_stats.outgoing_rps.one().armor == approx(60.16)
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeSim(time=7))])))
    assert api_module_ancil_stats.outgoing_rps.one().armor == approx(62.142857)
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(
        outgoing_rps=(True, [StatsOptionItemOutRps(time_options=StatTimeSim(time=7))])))
    assert api_module_spool_stats.outgoing_rps.one().armor == approx(73.142857)
    # Sim with time when AAR exhausted its clip, and trig rep spooled a bit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(time_options=StatTimeSim(time=79, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionFitOutRps(time_options=StatTimeSim(time=79, optional_reloads=consts.ApiOptionalReload.on_empty))])))
    assert api_fleet_stats.outgoing_rps.map(lambda i: i.armor) == [approx(274.700253), approx(250.839494)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(time_options=StatTimeSim(time=79, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionFitOutRps(time_options=StatTimeSim(time=79, optional_reloads=consts.ApiOptionalReload.on_empty))])))
    assert api_fit_stats.outgoing_rps.map(lambda i: i.armor) == [approx(274.700253), approx(250.839494)]
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(outgoing_rps=(True, [
        StatsOptionItemOutRps(time_options=StatTimeSim(time=79, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=79, optional_reloads=consts.ApiOptionalReload.on_empty))])))
    assert api_module_normal_stats.outgoing_rps.map(lambda i: i.armor) == [approx(61.873418), approx(61.873418)]
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=(True, [
        StatsOptionItemOutRps(time_options=StatTimeSim(time=79, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=79, optional_reloads=consts.ApiOptionalReload.on_empty))])))
    assert api_module_ancil_stats.outgoing_rps.map(lambda i: i.armor) == [approx(67.911392), approx(44.050633)]
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(outgoing_rps=(True, [
        StatsOptionItemOutRps(time_options=StatTimeSim(time=79, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=79, optional_reloads=consts.ApiOptionalReload.on_empty))])))
    assert api_module_spool_stats.outgoing_rps.map(lambda i: i.armor) == [approx(144.915443), approx(144.915443)]
    # Action
    api_module_ancil.change_module(charge_type_id=None)
    # Verification - ancil rep in all the modes
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=(True, [
        StatsOptionItemOutRps(time_options=StatTimeBurst()),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=None, optional_reloads=consts.ApiOptionalReload.on_empty)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=5)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=7)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=79, optional_reloads=consts.ApiOptionalReload.disabled)),
        StatsOptionItemOutRps(time_options=StatTimeSim(time=79, optional_reloads=consts.ApiOptionalReload.on_empty))])))
    assert api_module_ancil_stats.outgoing_rps.map(lambda i: i.armor) == [
        approx(24.166667),
        approx(24.166667),
        approx(24.166667),
        0,
        approx(20.714286),
        approx(23.860759),
        approx(23.860759)]


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
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().armor == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().armor == 0
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_normal_stats.outgoing_rps.one().armor == 0
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_ancil_stats.outgoing_rps.one().armor == 0
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_spool_stats.outgoing_rps.one().armor == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps.one().armor == 0


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
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_rps=True))
    assert api_fleet_stats.outgoing_rps.one().armor == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().armor == 0
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_normal_stats.outgoing_rps.one().armor == 0
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_ancil_stats.outgoing_rps.one().armor == 0
    api_module_spool_stats = api_module_spool.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_spool_stats.outgoing_rps.one().armor == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps.one().armor == 0


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
    assert api_fleet_stats.outgoing_rps.one().armor == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_rps=True))
    assert api_fit_stats.outgoing_rps.one().armor == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_module_stats.outgoing_rps is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(outgoing_rps=True))
    assert api_drone_stats.outgoing_rps is None
