from fw.api import ItemStatsOptions


def test_remote_effect(client, consts):
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_bubble, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        can_warp=True,
        can_tether=True,
        can_jump_drive=True,
        can_dock_citadel=True))
    assert api_tgt_ship_stats.can_warp is True
    assert api_tgt_ship_stats.can_tether is True
    assert api_tgt_ship_stats.can_jump_drive is True
    assert api_tgt_ship_stats.can_dock_citadel is True
    # Action
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(
        can_warp=True,
        can_tether=True,
        can_jump_drive=True,
        can_dock_citadel=True))
    assert api_tgt_ship_stats.can_warp is False
    assert api_tgt_ship_stats.can_tether is True
    assert api_tgt_ship_stats.can_jump_drive is False
    assert api_tgt_ship_stats.can_dock_citadel is True


def test_remote_range(client, consts):
    eve_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_range)
    eve_aoe_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_aoe_range)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_bubble, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_range_attr_id: 500000, eve_aoe_range_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 4400})
    eve_tgt_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 1500})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 511499, 0))
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_tgt_ship_stats.can_warp is False
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 511501, 0))
    # Verification
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_tgt_ship_stats.can_warp is True
