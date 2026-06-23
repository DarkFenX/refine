from fw import approx
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionFitDmg, StatsOptionItemDmg
from tests.stats.dmg import make_eve_ship, make_eve_smartbomb, setup_dmg_basics


def test_range(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_smartbomb(
        client=client, basic_info=eve_basic_info, dmgs=(45, 45, 45, 45), cycle_time=7500, range_optimal=7200)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=40)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 10700, 0), movement=(0, 0, 1))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification - surface-to-surface range is used, with perfect application
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(12), approx(12), approx(12), approx(12)]
    assert api_fleet_dmg_stats.volley == [approx(90), approx(90), approx(90), approx(90)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(12), approx(12), approx(12), approx(12)]
    assert api_src_fit_dmg_stats.volley == [approx(90), approx(90), approx(90), approx(90)]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(6), approx(6), approx(6), approx(6)]
    assert api_module_proj_dmg_stats.volley == [approx(45), approx(45), approx(45), approx(45)]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(6), approx(6), approx(6), approx(6)]
    assert api_module_nonproj_dmg_stats.volley == [approx(45), approx(45), approx(45), approx(45)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 10800, 0))
    # Verification - since target is now barely out of range, smartbomb deals no damage
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_smartbomb_effect_range_absent(client, consts):
    # No range reference on effect is considered as 0 range
    eve_basic_info = setup_dmg_basics(client=client, consts=consts, effect_range=False)
    eve_module_id = make_eve_smartbomb(
        client=client, basic_info=eve_basic_info, dmgs=(45, 45, 45, 45), cycle_time=7500, range_optimal=7200)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=40)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 3550, 0), movement=(0, 0, 1))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(6), approx(6), approx(6), approx(6)]
    assert api_module_proj_dmg_stats.volley == [approx(45), approx(45), approx(45), approx(45)]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(6), approx(6), approx(6), approx(6)]
    assert api_module_nonproj_dmg_stats.volley == [approx(45), approx(45), approx(45), approx(45)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 3551, 0))
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_smartbomb_attr_range_absent(client, consts):
    # No range attr is considered as 0 range
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_smartbomb(
        client=client, basic_info=eve_basic_info, dmgs=(45, 45, 45, 45), cycle_time=7500)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=40)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 3550, 0), movement=(0, 0, 1))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(6), approx(6), approx(6), approx(6)]
    assert api_module_proj_dmg_stats.volley == [approx(45), approx(45), approx(45), approx(45)]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(6), approx(6), approx(6), approx(6)]
    assert api_module_nonproj_dmg_stats.volley == [approx(45), approx(45), approx(45), approx(45)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 3551, 0))
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_smartbomb_ship_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_smartbomb(
        client=client, basic_info=eve_basic_info, dmgs=(45, 45, 45, 45), cycle_time=7500, range_optimal=7200)
    eve_src_ship_id = client.alloc_item_id()
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=40)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 1))
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(6), approx(6), approx(6), approx(6)]
    assert api_module_proj_dmg_stats.volley == [approx(45), approx(45), approx(45), approx(45)]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(6), approx(6), approx(6), approx(6)]
    assert api_module_nonproj_dmg_stats.volley == [approx(45), approx(45), approx(45), approx(45)]


def test_tgt_not_loaded(client, consts):
    # Smartbombs do not rely on any target attributes, so work even vs non-loaded targets
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_smartbomb(
        client=client, basic_info=eve_basic_info, dmgs=(45, 45, 45, 45), cycle_time=7500, range_optimal=7200)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550)
    eve_tgt_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 1))
    # Verification
    api_module_proj_dmg_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_proj_dmg_stats.dps == [approx(6), approx(6), approx(6), approx(6)]
    assert api_module_proj_dmg_stats.volley == [approx(45), approx(45), approx(45), approx(45)]
    api_module_nonproj_dmg_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_module_nonproj_dmg_stats.dps == [approx(6), approx(6), approx(6), approx(6)]
    assert api_module_nonproj_dmg_stats.volley == [approx(45), approx(45), approx(45), approx(45)]
