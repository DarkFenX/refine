from tests import Range, Spool, approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionRps
from tests.tests.stats.tank import (
    make_eve_remote_ar,
    make_eve_remote_hr,
    make_eve_remote_sar,
    make_eve_remote_sb,
    make_eve_tankable,
    setup_tank_basics,
)


def test_layers(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(100000, 100000, 100000))
    eve_module_rsb_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=8594, cycle_time=5000)
    eve_module_rar_id = make_eve_remote_ar(client=client, basic_info=eve_basic_info, rep_amount=8594, cycle_time=5000)
    eve_module_rhr_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=8594, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)

    def add_reps():
        api_module_rsb = api_src_fit.add_module(type_id=eve_module_rsb_id, state=consts.ApiModuleState.active)
        api_module_rar = api_src_fit.add_module(type_id=eve_module_rar_id, state=consts.ApiModuleState.active)
        api_module_rhr = api_src_fit.add_module(type_id=eve_module_rhr_id, state=consts.ApiModuleState.active)
        api_module_rsb.change_module(add_projs=[api_tgt_ship.id])
        api_module_rar.change_module(add_projs=[api_tgt_ship.id])
        api_module_rhr.change_module(add_projs=[api_tgt_ship.id])

    add_reps()
    # Verification - due to penalty formula, one rep per layer is never penalized
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, approx(1718.8), approx(1718.8)]
    assert api_tgt_fit_stats.rps.one().armor == [0, approx(1718.8), approx(1718.8)]
    assert api_tgt_fit_stats.rps.one().hull == [0, approx(1718.8), approx(1718.8)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, approx(1718.8), approx(1718.8)]
    assert api_tgt_ship_stats.rps.one().armor == [0, approx(1718.8), approx(1718.8)]
    assert api_tgt_ship_stats.rps.one().hull == [0, approx(1718.8), approx(1718.8)]
    # Action
    add_reps()
    # Verification - penalties appear from 2nd rep
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, approx(3437.6), approx(3432.543077)]
    assert api_tgt_fit_stats.rps.one().armor == [0, approx(3437.6), approx(3432.543077)]
    assert api_tgt_fit_stats.rps.one().hull == [0, approx(3437.6), approx(3432.543077)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, approx(3437.6), approx(3432.543077)]
    assert api_tgt_ship_stats.rps.one().armor == [0, approx(3437.6), approx(3432.543077)]
    assert api_tgt_ship_stats.rps.one().hull == [0, approx(3437.6), approx(3432.543077)]
    # Action
    for _ in range(8):
        add_reps()
    # Verification - as reps get added, penalty increases
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, approx(17188), approx(15988.777992)]
    assert api_tgt_fit_stats.rps.one().armor == [0, approx(17188), approx(15988.777992)]
    assert api_tgt_fit_stats.rps.one().hull == [0, approx(17188), approx(15988.777992)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, approx(17188), approx(15988.777992)]
    assert api_tgt_ship_stats.rps.one().armor == [0, approx(17188), approx(15988.777992)]
    assert api_tgt_ship_stats.rps.one().hull == [0, approx(17188), approx(15988.777992)]


def test_rounding(client, consts):
    # For the sake of penalty multiplier calculation, cycle time is rounded down to integer seconds
    # for some reason. This has been tested sometime in 2024 and confirmed to work like this (like
    # the rest of pre-change RR penalization formula). First rep represents self-linked CONCORD RR
    # on minokawa, second module is same but t3c/CD-linked
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(100000, 100000, 100000))
    eve_module1_id = make_eve_remote_sb(
        client=client, basic_info=eve_basic_info, rep_amount=8593.75, cycle_time=4015.625)
    eve_module2_id = make_eve_remote_sb(
        client=client, basic_info=eve_basic_info, rep_amount=8593.75, cycle_time=3968.75)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_mods = [api_src_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active) for _ in range(40)]
    for api_mod in api_mods:
        api_mod.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, approx(85603.11284), approx(53065.524427)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, approx(85603.11284), approx(53065.524427)]
    # Action
    for api_mod in api_mods:
        api_mod.change_module(type_id=eve_module2_id)
    # Verification - when cycle in seconds goes past integer threshold, it increases rep power per
    # second in RR penalty multiplier calculation. In extreme cases like this, faster cycling rep
    # can lead to lower amount of RR/s applied to target ship
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, approx(86614.173228), approx(52827.782027)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, approx(86614.173228), approx(52827.782027)]


def test_range(client, consts):
    # Not tested in EVE, but the lib assumes that range-reduced rep amount is considered for
    # penalization
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(100000, 100000, 100000))
    eve_module_id = make_eve_remote_sb(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=8593.75,
        cycle_time=4015.625,
        optimal_range=34200,
        falloff_range=45300)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_mods = [api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active) for _ in range(20)]
    for api_mod in api_mods:
        api_mod.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, approx(42801.55642), approx(34544.40516)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, approx(42801.55642), approx(34544.40516)]
    # Action
    for api_mod in api_mods:
        api_mod.change_module(change_projs=[(api_tgt_ship.id, Range.s2s_to_api(val=79500))])
    # Verification - raw reps are cut in half, penalized are cut less
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [0, approx(21400.77821), approx(17830.309313)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [0, approx(21400.77821), approx(17830.309313)]


def test_spool(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(100000, 100000, 100000))
    eve_module_id = make_eve_remote_sar(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=1024,
        cycle_time=6000,
        spool_step=0.12,
        spool_max=1.8)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    for _ in range(20):
        api_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
        api_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification - as spool is increased, penalty is increased as well
    api_stat_options = [
        StatsOptionRps(spool=Spool.spool_scale_to_api(val=0)),
        StatsOptionRps(spool=Spool.spool_scale_to_api(val=0.5)),
        StatsOptionRps(spool=Spool.spool_scale_to_api(val=1))]
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=(True, api_stat_options)))
    api_tgt_fit_stats_prespool, api_tgt_fit_stats_midspool, api_tgt_fit_stats_spooled = api_tgt_fit_stats.rps
    assert api_tgt_fit_stats_prespool.armor == [0, approx(3413.333333), approx(3225.597)]
    assert api_tgt_fit_stats_midspool.armor == [0, approx(6690.133333), approx(6039.5067468)]
    assert api_tgt_fit_stats_spooled.armor == [0, approx(9557.333333), approx(8402.05019)]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=(True, api_stat_options)))
    api_tgt_ship_stats_prespool, api_tgt_ship_stats_midspool, api_tgt_ship_stats_spooled = api_tgt_ship_stats.rps
    assert api_tgt_ship_stats_prespool.armor == [0, approx(3413.333333), approx(3225.597)]
    assert api_tgt_ship_stats_midspool.armor == [0, approx(6690.133333), approx(6039.5067468)]
    assert api_tgt_ship_stats_spooled.armor == [0, approx(9557.333333), approx(8402.05019)]
