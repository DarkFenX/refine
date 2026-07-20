from fw.api import ItemStatsOptions


def test_restrictions(client, consts):
    eve_cloak_t1_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.cloaking_prototype,
        cat_id=consts.EveEffCat.active)
    eve_cloak_t2_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.cloaking,
        cat_id=consts.EveEffCat.active)
    eve_cloak_covops_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.cloaking_warp_safe,
        cat_id=consts.EveEffCat.active)
    eve_cloak_t1_id = client.mk_eve_item(eff_ids=[eve_cloak_t1_effect_id], defeff_id=eve_cloak_t1_effect_id)
    eve_cloak_t2_id = client.mk_eve_item(eff_ids=[eve_cloak_t2_effect_id], defeff_id=eve_cloak_t2_effect_id)
    eve_cloak_covops_id = client.mk_eve_item(eff_ids=[eve_cloak_covops_effect_id], defeff_id=eve_cloak_covops_effect_id)
    eve_fighter_id = client.mk_eve_fighter()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_cloak = api_fit.add_module(type_id=eve_cloak_t1_id, state=consts.ApiModuleState.active)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is False
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_fighter_stats.can_warp.one() is True
    # Action
    api_cloak.change_module(type_id=eve_cloak_t2_id)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is False
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_fighter_stats.can_warp.one() is True
    # Action
    api_cloak.change_module(type_id=eve_cloak_covops_id)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    assert api_ship_stats.can_warp.one() is True
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is False
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_fighter_stats.can_warp.one() is True
