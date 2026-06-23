from fw import approx, check_no_field
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionFitDmg, StatsOptionItemDmg
from tests.stats.dmg import make_eve_ship, make_eve_smartbomb, setup_dmg_basics


def test_not_loaded(client, consts):
    eve_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    api_fighter = api_fit.add_fighter(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg
    assert api_module_dmg_stats is None
    api_drone_dmg_stats = api_drone.get_stats(options=ItemStatsOptions(dmg=True)).dmg
    assert api_drone_dmg_stats is None
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg
    assert api_fighter_dmg_stats is None


def test_incorrect_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_implant_id = make_eve_smartbomb(client=client, basic_info=eve_basic_info, dmgs=(120, 0, 0, 0), cycle_time=7500)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_implant_stats = api_implant.get_stats(options=ItemStatsOptions(dmg=True))
    assert api_implant_stats.dmg is None


def test_incorrect_projectee(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_smartbomb(client=client, basic_info=eve_basic_info, dmgs=(120, 0, 0, 0), cycle_time=7500)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info)
    eve_implant_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_implant = api_src_fit.add_implant(type_id=eve_implant_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_tmp = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_tgt_tmp.remove()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    # Verification - specifying incorrect projectee item IDs should fail only that specific option,
    # not whole stat batch
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(projectee_item_id=api_tgt_tmp.id),
        StatsOptionFitDmg(projectee_item_id=api_implant.id),
        StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dmg == [None, None, ([approx(16), 0, 0, 0], [approx(120), 0, 0, 0])]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(projectee_item_id=api_tgt_tmp.id),
        StatsOptionFitDmg(projectee_item_id=api_implant.id),
        StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dmg == [None, None, ([approx(16), 0, 0, 0], [approx(120), 0, 0, 0])]
    api_src_module_stats = api_src_module.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(projectee_item_id=api_tgt_tmp.id),
        StatsOptionItemDmg(projectee_item_id=api_implant.id),
        StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_stats.dmg == [None, None, ([approx(16), 0, 0, 0], [approx(120), 0, 0, 0])]


def test_not_requested(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_smartbomb(client=client, basic_info=eve_basic_info, dmgs=(45, 45, 45, 45), cycle_time=7500)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=False))
    with check_no_field():
        api_fleet_stats.dmg  # noqa: B018
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=False))
    with check_no_field():
        api_fit_stats.dmg  # noqa: B018
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=False))
    with check_no_field():
        api_module_stats.dmg  # noqa: B018
