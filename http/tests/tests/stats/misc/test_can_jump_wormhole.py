from fw import check_no_field
from fw.api import FitStatsOptions, ItemStatsOptions


def test_module_state(client, consts):
    eve_mjd_effect_id = client.mk_eve_effect(id_=consts.EveEffect.micro_jump_drive, cat_id=consts.EveEffCat.active)
    eve_mjd_id = client.mk_eve_item(eff_ids=[eve_mjd_effect_id], defeff_id=eve_mjd_effect_id)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_mjd = api_fit.add_module(type_id=eve_mjd_id, state=consts.ApiModuleState.online)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_jump_wormhole=True))
    assert api_fit_stats.can_jump_wormhole.one() is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_jump_wormhole=True))
    assert api_ship_stats.can_jump_wormhole.one() is True
    # Action
    api_mjd.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_jump_wormhole=True))
    assert api_fit_stats.can_jump_wormhole.one() is False
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_jump_wormhole=True))
    assert api_ship_stats.can_jump_wormhole.one() is False
    # Action
    api_mjd.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_jump_wormhole=True))
    assert api_fit_stats.can_jump_wormhole.one() is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_jump_wormhole=True))
    assert api_ship_stats.can_jump_wormhole.one() is True


def test_type_list(client, consts):
    eve_ship1_id = client.mk_eve_ship()
    eve_ship2_id = client.mk_eve_ship()
    client.mk_eve_item_list(id_=consts.EveItemList.wormhole_jump_blacklist, inc_type_ids=[eve_ship1_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_jump_wormhole=True))
    assert api_fit_stats.can_jump_wormhole.one() is False
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_jump_wormhole=True))
    assert api_ship_stats.can_jump_wormhole.one() is False
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_jump_wormhole=True))
    assert api_fit_stats.can_jump_wormhole.one() is True
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_jump_wormhole=True))
    assert api_ship_stats.can_jump_wormhole.one() is True


def test_ship_absent(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_jump_wormhole=True))
    assert api_fit_stats.can_jump_wormhole is None


def test_ship_not_loaded(client):
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_jump_wormhole=True))
    assert api_fit_stats.can_jump_wormhole is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_jump_wormhole=True))
    assert api_ship_stats.can_jump_wormhole is None


def test_struct(client):
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_jump_wormhole=True))
    assert api_fit_stats.can_jump_wormhole is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_jump_wormhole=True))
    assert api_ship_stats.can_jump_wormhole is None


def test_incorrect_item_kind(client):
    eve_fighter_id = client.mk_eve_fighter()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_drone_stats = api_fighter.get_stats(options=ItemStatsOptions(can_jump_wormhole=True))
    assert api_drone_stats.can_jump_wormhole is None


def test_not_requested(client, consts):
    eve_mjd_effect_id = client.mk_eve_effect(id_=consts.EveEffect.micro_jump_drive, cat_id=consts.EveEffCat.active)
    eve_mjd_id = client.mk_eve_item(eff_ids=[eve_mjd_effect_id], defeff_id=eve_mjd_effect_id)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_mjd_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(can_jump_wormhole=False))
    with check_no_field():
        api_fit_stats.can_jump_wormhole  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(can_jump_wormhole=False))
    with check_no_field():
        api_ship_stats.can_jump_wormhole  # noqa: B018
