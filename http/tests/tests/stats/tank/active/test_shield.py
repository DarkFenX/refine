from tests import ANY_VALUE, approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions
from tests.tests.stats.tank import (
    make_eve_drone_shield,
    make_eve_local_asb,
    make_eve_local_sb,
    make_eve_remote_asb,
    make_eve_remote_sb,
    make_eve_tankable,
    setup_tank_basics,
)


def test_state_ship(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 1000))
    eve_module_lsb_id = make_eve_local_sb(client=client, basic_info=eve_basic_info, rep_amount=228, cycle_time=3000)
    eve_module_lasb_id = make_eve_local_asb(client=client, basic_info=eve_basic_info, rep_amount=146, cycle_time=3000)
    eve_module_rsb_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=508, cycle_time=8000)
    eve_module_rasb_id = make_eve_remote_asb(client=client, basic_info=eve_basic_info, rep_amount=950, cycle_time=8000)
    eve_drone_id = make_eve_drone_shield(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rsb = api_src_fit.add_module(type_id=eve_module_rsb_id, state=consts.ApiModuleState.active)
    api_module_rasb = api_src_fit.add_module(type_id=eve_module_rasb_id, state=consts.ApiModuleState.active)
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_module_lsb = api_tgt_fit.add_module(type_id=eve_module_lsb_id, state=consts.ApiModuleState.active)
    api_module_lasb = api_tgt_fit.add_module(type_id=eve_module_lasb_id, state=consts.ApiModuleState.active)
    api_module_rsb.change_module(add_projs=[api_tgt_ship.id])
    api_module_rasb.change_module(add_projs=[api_tgt_ship.id])
    api_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [approx(124.666667), approx(196.65), ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [0, 0, ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [approx(124.666667), approx(196.65), ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [0, 0, ANY_VALUE]
    # Action
    api_module_lsb.change_module(state=consts.ApiModuleState.online)
    api_module_lasb.change_module(state=consts.ApiModuleState.online)
    api_module_rsb.change_module(state=consts.ApiModuleState.online)
    api_module_rasb.change_module(state=consts.ApiModuleState.online)
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
    api_module_lsb.change_module(state=consts.ApiModuleState.active)
    api_module_lasb.change_module(state=consts.ApiModuleState.active)
    api_module_rsb.change_module(state=consts.ApiModuleState.active)
    api_module_rasb.change_module(state=consts.ApiModuleState.active)
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [approx(124.666667), approx(196.65), ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [0, 0, ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [approx(124.666667), approx(196.65), ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [0, 0, ANY_VALUE]


def test_drone(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_tgt_drone_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 1000))
    eve_module_lsb_id = make_eve_local_sb(client=client, basic_info=eve_basic_info, rep_amount=228, cycle_time=3000)
    eve_module_lasb_id = make_eve_local_asb(client=client, basic_info=eve_basic_info, rep_amount=146, cycle_time=3000)
    eve_module_rsb_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=508, cycle_time=8000)
    eve_module_rasb_id = make_eve_remote_asb(client=client, basic_info=eve_basic_info, rep_amount=950, cycle_time=8000)
    eve_src_drone_id = make_eve_drone_shield(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rsb = api_src_fit.add_module(type_id=eve_module_rsb_id, state=consts.ApiModuleState.active)
    api_module_rasb = api_src_fit.add_module(type_id=eve_module_rasb_id, state=consts.ApiModuleState.active)
    api_src_drone = api_src_fit.add_drone(type_id=eve_src_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_drone = api_tgt_fit.add_drone(type_id=eve_tgt_drone_id)
    api_tgt_fit.add_module(type_id=eve_module_lsb_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_lasb_id, state=consts.ApiModuleState.active)
    api_module_rsb.change_module(add_projs=[api_tgt_drone.id])
    api_module_rasb.change_module(add_projs=[api_tgt_drone.id])
    api_src_drone.change_drone(add_projs=[api_tgt_drone.id])
    # Verification - local reps do not affect drones
    api_tgt_drone_stats = api_tgt_drone.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_drone_stats.rps.one().shield == [0, approx(196.65), ANY_VALUE]
    assert api_tgt_drone_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_drone_stats.rps.one().hull == [0, 0, ANY_VALUE]


def test_hp_limit_and_resist(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(50, 1000, 1000), rr_resist=0.1)
    eve_module_lsb_id = make_eve_local_sb(client=client, basic_info=eve_basic_info, rep_amount=228, cycle_time=3000)
    eve_module_lasb_id = make_eve_local_asb(client=client, basic_info=eve_basic_info, rep_amount=146, cycle_time=3000)
    eve_module_rsb_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=508, cycle_time=8000)
    eve_module_rasb_id = make_eve_remote_asb(client=client, basic_info=eve_basic_info, rep_amount=950, cycle_time=8000)
    eve_drone_id = make_eve_drone_shield(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rsb = api_src_fit.add_module(type_id=eve_module_rsb_id, state=consts.ApiModuleState.active)
    api_module_rasb = api_src_fit.add_module(type_id=eve_module_rasb_id, state=consts.ApiModuleState.active)
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_module_lsb_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_lasb_id, state=consts.ApiModuleState.active)
    api_module_rsb.change_module(add_projs=[api_tgt_ship.id])
    api_module_rasb.change_module(add_projs=[api_tgt_ship.id])
    api_drone.change_drone(add_projs=[api_tgt_ship.id])
    # Verification - local reps are not resisted but limited, remote reps resisted and limited,
    # except for drone which has low enough reps to be below limit
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [approx(33.333333), approx(13.94), ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [0, 0, ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [approx(33.333333), approx(13.94), ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [0, 0, ANY_VALUE]


def test_hp_limit_and_range(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(300, 1000, 1000))
    eve_module_lsb_id = make_eve_local_sb(client=client, basic_info=eve_basic_info, rep_amount=228, cycle_time=3000)
    eve_module_lasb_id = make_eve_local_asb(client=client, basic_info=eve_basic_info, rep_amount=146, cycle_time=3000)
    eve_module_rsb_id = make_eve_remote_sb(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=508,
        cycle_time=8000,
        optimal_range=10000,
        falloff_range=5000)
    eve_module_rasb_id = make_eve_remote_asb(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=950,
        cycle_time=8000,
        optimal_range=10000,
        falloff_range=5000)
    eve_drone_id = make_eve_drone_shield(
        client=client,
        basic_info=eve_basic_info,
        rep_amount=72,
        cycle_time=5000,
        optimal_range=10000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rsb = api_src_fit.add_module(type_id=eve_module_rsb_id, state=consts.ApiModuleState.active)
    api_module_rasb = api_src_fit.add_module(type_id=eve_module_rasb_id, state=consts.ApiModuleState.active)
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_module_lsb_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_lasb_id, state=consts.ApiModuleState.active)
    api_module_rsb.change_module(add_projs=[(api_tgt_ship.id, Range.s2s_to_api(val=15000))])
    api_module_rasb.change_module(add_projs=[(api_tgt_ship.id, Range.s2s_to_api(val=15000))])
    api_drone.change_drone(add_projs=[(api_tgt_ship.id, Range.s2s_to_api(val=10001))])
    # Verification - effect of local reps is not reduced altogether, drone effect is removed since
    # it's out of range, regular RR effect is reduced but not limited, ancillary RR effect is
    # reduced and limited (since its reduced RR amount is still more than target HP)
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps.one().shield == [approx(124.666667), approx(69.25), ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_fit_stats.rps.one().hull == [0, 0, ANY_VALUE]
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps.one().shield == [approx(124.666667), approx(69.25), ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().armor == [0, 0, ANY_VALUE]
    assert api_tgt_ship_stats.rps.one().hull == [0, 0, ANY_VALUE]


def test_zero_cycle_time(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_ship_id = make_eve_tankable(client=client, basic_info=eve_basic_info, hps=(3000, 1000, 1000))
    eve_module_lsb_id = make_eve_local_sb(client=client, basic_info=eve_basic_info, rep_amount=228, cycle_time=0)
    eve_module_lasb_id = make_eve_local_asb(client=client, basic_info=eve_basic_info, rep_amount=146, cycle_time=0)
    eve_module_rsb_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=508, cycle_time=0)
    eve_module_rasb_id = make_eve_remote_asb(client=client, basic_info=eve_basic_info, rep_amount=950, cycle_time=0)
    eve_drone_id = make_eve_drone_shield(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=0)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rsb = api_src_fit.add_module(type_id=eve_module_rsb_id, state=consts.ApiModuleState.active)
    api_module_rasb = api_src_fit.add_module(type_id=eve_module_rasb_id, state=consts.ApiModuleState.active)
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_module_lsb_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_lasb_id, state=consts.ApiModuleState.active)
    api_module_rsb.change_module(add_projs=[api_tgt_ship.id])
    api_module_rasb.change_module(add_projs=[api_tgt_ship.id])
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
    eve_module_lsb_id = make_eve_local_sb(client=client, basic_info=eve_basic_info, rep_amount=228, cycle_time=3000)
    eve_module_lasb_id = make_eve_local_asb(client=client, basic_info=eve_basic_info, rep_amount=146, cycle_time=3000)
    eve_module_rsb_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=508, cycle_time=8000)
    eve_module_rasb_id = make_eve_remote_asb(client=client, basic_info=eve_basic_info, rep_amount=950, cycle_time=8000)
    eve_drone_id = make_eve_drone_shield(client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rsb = api_src_fit.add_module(type_id=eve_module_rsb_id, state=consts.ApiModuleState.active)
    api_module_rasb = api_src_fit.add_module(type_id=eve_module_rasb_id, state=consts.ApiModuleState.active)
    api_drone = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_tgt_fit.add_module(type_id=eve_module_lsb_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_lasb_id, state=consts.ApiModuleState.active)
    api_module_rsb.change_module(add_projs=[api_tgt_ship.id])
    api_module_rasb.change_module(add_projs=[api_tgt_ship.id])
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
