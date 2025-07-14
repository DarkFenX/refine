from tests.fw.api import FitStatsOptions, ItemStatsOptions
from tests.tests.stats.tank import (
    make_eve_drone_hull,
    make_eve_drone_shield,
    make_eve_local_asb,
    make_eve_local_hr,
    make_eve_local_sb,
    make_eve_remote_asb,
    make_eve_remote_hr,
    make_eve_remote_sb,
    setup_tank_basics,
)


def test_item_not_loaded(client, consts):
    eve_basic_info = setup_tank_basics(client=client, consts=consts)
    eve_item_id = client.alloc_item_id()
    eve_module_lsb_id = make_eve_local_sb(client=client, basic_info=eve_basic_info, rep_amount=228, cycle_time=3000)
    eve_module_lasb_id = make_eve_local_asb(client=client, basic_info=eve_basic_info, rep_amount=146, cycle_time=3000)
    eve_module_rsb_id = make_eve_remote_sb(client=client, basic_info=eve_basic_info, rep_amount=508, cycle_time=8000)
    eve_module_rasb_id = make_eve_remote_asb(client=client, basic_info=eve_basic_info, rep_amount=950, cycle_time=8000)
    eve_module_lhr_id = make_eve_local_hr(client=client, basic_info=eve_basic_info, rep_amount=120, cycle_time=24000)
    eve_module_rhr_id = make_eve_remote_hr(client=client, basic_info=eve_basic_info, rep_amount=230, cycle_time=6000)
    eve_drone_shield_id = make_eve_drone_shield(
        client=client, basic_info=eve_basic_info, rep_amount=72, cycle_time=5000)
    eve_drone_hull_id = make_eve_drone_hull(client=client, basic_info=eve_basic_info, rep_amount=36, cycle_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_module_rsb = api_src_fit.add_module(type_id=eve_module_rsb_id, state=consts.ApiModuleState.active)
    api_module_rasb = api_src_fit.add_module(type_id=eve_module_rasb_id, state=consts.ApiModuleState.active)
    api_module_rhr = api_src_fit.add_module(type_id=eve_module_rhr_id, state=consts.ApiModuleState.active)
    api_src_drone_shield = api_src_fit.add_drone(type_id=eve_drone_shield_id, state=consts.ApiMinionState.engaging)
    api_src_drone_hull = api_src_fit.add_drone(type_id=eve_drone_hull_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_item_id)
    api_tgt_drone = api_tgt_fit.add_drone(type_id=eve_item_id)
    api_tgt_fighter = api_tgt_fit.add_fighter(type_id=eve_item_id)
    api_tgt_fit.add_module(type_id=eve_module_lsb_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_lasb_id, state=consts.ApiModuleState.active)
    api_tgt_fit.add_module(type_id=eve_module_lhr_id, state=consts.ApiModuleState.active)
    api_module_rsb.change_module(add_projs=[api_tgt_ship.id, api_tgt_drone.id, api_tgt_fighter.id])
    api_module_rasb.change_module(add_projs=[api_tgt_ship.id, api_tgt_drone.id, api_tgt_fighter.id])
    api_module_rhr.change_module(add_projs=[api_tgt_ship.id, api_tgt_drone.id, api_tgt_fighter.id])
    api_src_drone_shield.change_drone(add_projs=[api_tgt_ship.id, api_tgt_drone.id, api_tgt_fighter.id])
    api_src_drone_hull.change_drone(add_projs=[api_tgt_ship.id, api_tgt_drone.id, api_tgt_fighter.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(rps=True))
    assert api_tgt_fit_stats.rps is None
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_ship_stats.rps is None
    api_tgt_drone_stats = api_tgt_drone.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_drone_stats.rps is None
    api_tgt_fighter_stats = api_tgt_fighter.get_stats(options=ItemStatsOptions(rps=True))
    assert api_tgt_fighter_stats.rps is None
