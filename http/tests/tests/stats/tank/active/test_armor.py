from tests import ANY_VALUE, Spool, approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionRps
from tests.tests.stats.tank import (
    make_eve_drone_armor,
    make_eve_local_aar,
    make_eve_local_ar,
    make_eve_remote_aar,
    make_eve_remote_ar,
    make_eve_remote_sar,
    make_eve_tankable,
    setup_tank_basics,
)


def test_state_ship(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 1000))
    eve_module_lar_id = make_eve_local_ar(client=client, basic_info=eve_basic_info, rep_amount=538, cycle_time=12000)
    eve_module_laar_id = make_eve_local_aar(client=client, basic_info=eve_basic_info, rep_amount=207, cycle_time=12000)
    eve_module_rar_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
    eve_module_raar_id = make_eve_remote_aar(client=client, basic_info=eve_basic_info, rep_amount=145, cycle_time=6000)
    eve_module_rsar_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, cycle_time=6000, spool_step=0.12, spool_max=1.8)
    eve_drone_id = make_eve_drone_armor(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rar = api_src_fit.add_module(type_id=eve_module_rar_id, state=consts.ApiModuleState.active)
    api_module_raar = api_src_fit.add_module(
        type_id=eve_module_raar_id, state=consts.ApiModuleState.active, charge_type_id=eve_paste_id)
    api_module_rsar = api_src_fit.add_module(
        type_id=eve_module_rsar_id, state=consts.ApiModuleState.active, spool=Spool.spool_scale_to_api(val=0))
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_module_lar = api_tgt_fit.add_module(type_id=eve_module_lar_id, state=consts.ApiModuleState.active)
    api_module_laar = api_tgt_fit.add_module(
        type_id=eve_module_laar_id, state=consts.ApiModuleState.active, charge_type_id=eve_paste_id)
    api_module_rar.change_module(add_projs=[api_tgt_ship.id])
    api_module_raar.change_module(add_projs=[api_tgt_ship.id])
    api_module_rsar.change_module(add_projs=[api_tgt_ship.id])
    api_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [approx(96.583333), approx(234.9), ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [0, 0, ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [approx(96.583333), approx(234.9), ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [0, 0, ANY_VALUE]
    # Action
    api_module_lar.change_module(state=consts.ApiModuleState.online)
    api_module_laar.change_module(state=consts.ApiModuleState.online)
    api_module_rar.change_module(state=consts.ApiModuleState.online)
    api_module_raar.change_module(state=consts.ApiModuleState.online)
    api_module_rsar.change_module(state=consts.ApiModuleState.online)
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [0, 0, ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [0, 0, ANY_VALUE]
    # Action
    api_module_lar.change_module(state=consts.ApiModuleState.active)
    api_module_laar.change_module(state=consts.ApiModuleState.active)
    api_module_rar.change_module(state=consts.ApiModuleState.active)
    api_module_raar.change_module(state=consts.ApiModuleState.active)
    api_module_rsar.change_module(state=consts.ApiModuleState.active)
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [approx(96.583333), approx(234.9), ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [0, 0, ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [approx(96.583333), approx(234.9), ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [0, 0, ANY_VALUE]


def test_drone(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_tgt_drone_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 1000))
    eve_module_lar_id = make_eve_local_ar(client=client, basic_info=eve_basic_info, rep_amount=538, cycle_time=12000)
    eve_module_laar_id = make_eve_local_aar(client=client, basic_info=eve_basic_info, rep_amount=207, cycle_time=12000)
    eve_module_rar_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
    eve_module_raar_id = make_eve_remote_aar(client=client, basic_info=eve_basic_info, rep_amount=145, cycle_time=6000)
    eve_module_rsar_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, cycle_time=6000, spool_step=0.12, spool_max=1.8)
    eve_src_drone_id = make_eve_drone_armor(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rar = api_src_fit.add_module(type_id=eve_module_rar_id, state=consts.ApiModuleState.active)
    api_module_raar = api_src_fit.add_module(
        type_id=eve_module_raar_id, state=consts.ApiModuleState.active, charge_type_id=eve_paste_id)
    api_module_rsar = api_src_fit.add_module(
        type_id=eve_module_rsar_id, state=consts.ApiModuleState.active, spool=Spool.spool_scale_to_api(val=0))
    api_src_drone = api_src_fit.add_drone(type_id=eve_src_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_drone = api_tgt_fit.add_drone(type_id=eve_tgt_drone_id)
    api_tgt_fit.add_module(type_id=eve_module_lar_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_laar_id, state=consts.ApiModuleState.active, charge_type_id=eve_paste_id)
    api_module_rar.change_module(add_projs=[api_tgt_drone.id])
    api_module_raar.change_module(add_projs=[api_tgt_drone.id])
    api_module_rsar.change_module(add_projs=[api_tgt_drone.id])
    api_src_drone.change_drone(add_projs=[api_tgt_drone.id])
    # Verification - local reps do not affect drones
    api_tgt_drone_stats = api_tgt_drone.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_drone_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_drone_stats.rps.one().armor == [0, approx(234.9), ANY_VALUE]
    assert api_tgt_drone_stats.rps.one().hull == [0, 0, ANY_VALUE]


def test_hp_limit_and_resist(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 210, 600), rr_resist=0.5)
    eve_module_lar_id = make_eve_local_ar(client=client, basic_info=eve_basic_info, rep_amount=538, cycle_time=12000)
    eve_module_laar_id = make_eve_local_aar(client=client, basic_info=eve_basic_info, rep_amount=207, cycle_time=12000)
    eve_module_rar_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
    eve_module_raar_id = make_eve_remote_aar(client=client, basic_info=eve_basic_info, rep_amount=145, cycle_time=6000)
    eve_module_rsar_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, cycle_time=6000, spool_step=0.12, spool_max=1.8)
    eve_drone_id = make_eve_drone_armor(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rar = api_src_fit.add_module(type_id=eve_module_rar_id, state=consts.ApiModuleState.active)
    api_module_raar = api_src_fit.add_module(
        type_id=eve_module_raar_id, state=consts.ApiModuleState.active, charge_type_id=eve_paste_id)
    api_module_rsar = api_src_fit.add_module(
        type_id=eve_module_rsar_id, state=consts.ApiModuleState.active, spool=Spool.spool_scale_to_api(val=0))
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_module_lar_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_laar_id, state=consts.ApiModuleState.active, charge_type_id=eve_paste_id)
    api_module_rar.change_module(add_projs=[api_tgt_ship.id])
    api_module_raar.change_module(add_projs=[api_tgt_ship.id])
    api_module_rsar.change_module(add_projs=[api_tgt_ship.id])
    api_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification - both local reps are limited by ship HP, regular remote armor rep is resisted
    # but not limited, ancillary is boosted enough by paste to be limited despite resistance, spool
    # rep is resisted and limited, drone is just resisted
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [approx(35), approx(108.533333), ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [0, 0, ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [approx(35), approx(108.533333), ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [0, 0, ANY_VALUE]


def test_hp_limit_and_range(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 210, 1000))
    eve_module_lar_id = make_eve_local_ar(client=client, basic_info=eve_basic_info, rep_amount=538, cycle_time=12000)
    eve_module_laar_id = make_eve_local_aar(client=client, basic_info=eve_basic_info, rep_amount=207, cycle_time=12000)
    eve_module_rar_id = make_eve_remote_ar(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=376,
        cycle_time=6000,
        optimal_range=10000,
        falloff_range=5000)
    eve_module_raar_id = make_eve_remote_aar(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=145,
        cycle_time=6000,
        optimal_range=10000,
        falloff_range=5000)
    eve_module_rsar_id = make_eve_remote_sar(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=512,
        cycle_time=6000,
        spool_step=0.12,
        spool_max=1.8,
        optimal_range=10000)
    eve_drone_id = make_eve_drone_armor(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=72,
        cycle_time=5000,
        optimal_range=10000)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rar = api_src_fit.add_module(type_id=eve_module_rar_id, state=consts.ApiModuleState.active)
    api_module_raar = api_src_fit.add_module(
        type_id=eve_module_raar_id, state=consts.ApiModuleState.active, charge_type_id=eve_paste_id)
    api_module_rsar = api_src_fit.add_module(
        type_id=eve_module_rsar_id, state=consts.ApiModuleState.active, spool=Spool.spool_scale_to_api(val=0))
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_module_lar_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(
        type_id=eve_module_laar_id, state=consts.ApiModuleState.active, charge_type_id=eve_paste_id)
    api_module_rar.change_module(add_projs=[(api_tgt_ship.id, Range.s2s_to_api(val=15000))])
    api_module_raar.change_module(add_projs=[(api_tgt_ship.id, Range.s2s_to_api(val=15000))])
    api_module_rsar.change_module(add_projs=[(api_tgt_ship.id, Range.s2s_to_api(val=10001))])
    api_drone.change_drone(add_projs=[(api_tgt_ship.id, Range.s2s_to_api(val=10001))])
    # Verification - range completely disables drone and spool rep, the rest work the same as in the
    # resistance test
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [approx(35), approx(66.333333), ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [0, 0, ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [approx(35), approx(66.333333), ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [0, 0, ANY_VALUE]


def test_hp_limit_and_spool(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 900, 1000))
    eve_module_rsar_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, cycle_time=6000, spool_step=0.12, spool_max=1.8)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rsar = api_src_fit.add_module(
        type_id=eve_module_rsar_id, state=consts.ApiModuleState.active, spool=Spool.spool_scale_to_api(val=0.5))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_module_rsar.change_module(add_projs=[api_tgt_ship.id])
    # Verification - at zero spool isn't limited, at on-module default of 0.5 slightly limited, at
    # max spool is limited as well
    api_stat_options = [
        StatsOptionRps(spool=Spool.spool_scale_to_api(val=0)),
        StatsOptionRps(),
        StatsOptionRps(spool=Spool.spool_scale_to_api(val=1))]
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=(True, api_stat_options)))
    assert api_tgt_fit_stats.rps.map(lambda i: i.armor.remote) == [approx(85.333333), approx(150), approx(150)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=(True, api_stat_options)))
    assert api_tgt_ship_stats.rps.map(lambda i: i.armor.remote) == [approx(85.333333), approx(150), approx(150)]


def test_zero_cycle_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 1000))
    eve_module_lar_id = make_eve_local_ar(client=client, basic_info=eve_basic_info, rep_amount=538, cycle_time=0)
    eve_module_laar_id = make_eve_local_aar(client=client, basic_info=eve_basic_info, rep_amount=207, cycle_time=0)
    eve_module_rar_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=0)
    eve_module_raar_id = make_eve_remote_aar(client=client, basic_info=eve_basic_info, rep_amount=145, cycle_time=0)
    eve_module_rsar_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, cycle_time=0, spool_step=0.12, spool_max=1.8)
    eve_drone_id = make_eve_drone_armor(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=0)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rar = api_src_fit.add_module(type_id=eve_module_rar_id, state=consts.ApiModuleState.active)
    api_module_raar = api_src_fit.add_module(
        type_id=eve_module_raar_id, state=consts.ApiModuleState.active, charge_type_id=eve_paste_id)
    api_module_rsar = api_src_fit.add_module(
        type_id=eve_module_rsar_id, state=consts.ApiModuleState.active, spool=Spool.spool_scale_to_api(val=0))
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_module_lar_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_laar_id, state=consts.ApiModuleState.active, charge_type_id=eve_paste_id)
    api_module_rar.change_module(add_projs=[api_tgt_ship.id])
    api_module_raar.change_module(add_projs=[api_tgt_ship.id])
    api_module_rsar.change_module(add_projs=[api_tgt_ship.id])
    api_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [0, 0, ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [0, 0, ANY_VALUE]


def test_no_cycle_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts, effect_duration=False)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 1000))
    eve_module_lar_id = make_eve_local_ar(client=client, basic_info=eve_basic_info, rep_amount=538, cycle_time=12000)
    eve_module_laar_id = make_eve_local_aar(client=client, basic_info=eve_basic_info, rep_amount=207, cycle_time=12000)
    eve_module_rar_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=376, cycle_time=6000)
    eve_module_raar_id = make_eve_remote_aar(client=client, basic_info=eve_basic_info, rep_amount=145, cycle_time=6000)
    eve_module_rsar_id = make_eve_remote_sar(
        client=client, basic_info=eve_basic_info, rep_amount=512, cycle_time=6000, spool_step=0.12, spool_max=1.8)
    eve_drone_id = make_eve_drone_armor(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rar = api_src_fit.add_module(type_id=eve_module_rar_id, state=consts.ApiModuleState.active)
    api_module_raar = api_src_fit.add_module(
        type_id=eve_module_raar_id, state=consts.ApiModuleState.active, charge_type_id=eve_paste_id)
    api_module_rsar = api_src_fit.add_module(
        type_id=eve_module_rsar_id, state=consts.ApiModuleState.active, spool=Spool.spool_scale_to_api(val=0))
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_module_lar_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_laar_id, state=consts.ApiModuleState.active, charge_type_id=eve_paste_id)
    api_module_rar.change_module(add_projs=[api_tgt_ship.id])
    api_module_raar.change_module(add_projs=[api_tgt_ship.id])
    api_module_rsar.change_module(add_projs=[api_tgt_ship.id])
    api_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [0, 0, ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [0, 0, ANY_VALUE]
