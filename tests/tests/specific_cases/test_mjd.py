from fw import approx
from fw.api import ItemStatsOptions, ValOptions


def test_module_mjd_stats(client, consts):
    # Regular MJDs block almost everything, except for tether
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_sig_radius_bonus_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus_percent)
    eve_mjd_effect_id = client.mk_eve_effect(id_=consts.EveEffect.micro_jump_drive, cat_id=consts.EveEffCat.active)
    eve_mjd_id = client.mk_eve_item(
        attrs={eve_sig_radius_bonus_attr_id: 150},
        eff_ids=[eve_mjd_effect_id],
        defeff_id=eve_mjd_effect_id)
    eve_fighter_id = client.mk_eve_fighter()
    eve_ship_id = client.mk_eve_ship(attrs={eve_sig_radius_attr_id: 165})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_mjd_id, state=consts.ApiModuleState.active)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification - MJD stops lots of things on ship itself, but does not prevent warps of fit
    # fighters
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        sig_radius=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    assert api_ship_stats.sig_radius.one() == approx(412.5)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is False
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is True
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_fighter_stats.can_warp.one() is True


def test_module_mjd_cloak(client, consts):
    eve_mjd_effect_id = client.mk_eve_effect(id_=consts.EveEffect.micro_jump_drive, cat_id=consts.EveEffCat.active)
    eve_mjd_id = client.mk_eve_item(eff_ids=[eve_mjd_effect_id], defeff_id=eve_mjd_effect_id)
    eve_cloak_effect_id = client.mk_eve_effect(id_=consts.EveEffect.cloaking, cat_id=consts.EveEffCat.active)
    eve_cloak_id = client.mk_eve_item(eff_ids=[eve_cloak_effect_id], defeff_id=eve_cloak_effect_id)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_mjd_id, state=consts.ApiModuleState.active)
    api_fit.add_module(type_id=eve_cloak_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False


def test_module_mjfg_stats(client, consts):
    # MJFGs block everything, tether is blocked due to them being an aggro module
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_sig_radius_bonus_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius_bonus_percent)
    eve_mjd_subcap_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.micro_jump_portal_drive,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_mjd_cap_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.micro_jump_portal_drive_capital,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_mjd_subcap_id = client.mk_eve_item(
        attrs={eve_sig_radius_bonus_attr_id: 150},
        eff_ids=[eve_mjd_subcap_effect_id],
        defeff_id=eve_mjd_subcap_effect_id)
    eve_mjd_cap_id = client.mk_eve_item(
        attrs={eve_sig_radius_bonus_attr_id: 150},
        eff_ids=[eve_mjd_cap_effect_id],
        defeff_id=eve_mjd_cap_effect_id)
    eve_fighter_id = client.mk_eve_fighter()
    eve_ship_id = client.mk_eve_ship(attrs={eve_sig_radius_attr_id: 165})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_mjd_subcap = api_fit.add_module(type_id=eve_mjd_subcap_id, state=consts.ApiModuleState.active)
    api_mjd_cap = api_fit.add_module(type_id=eve_mjd_cap_id, state=consts.ApiModuleState.online)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification - MJFGs stop lots of things on ship itself, but does not prevent warps of fit
    # fighters
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        sig_radius=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    assert api_ship_stats.sig_radius.one() == approx(412.5)
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
    api_mjd_subcap.change_module(state=consts.ApiModuleState.online)
    api_mjd_cap.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        sig_radius=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    assert api_ship_stats.sig_radius.one() == approx(412.5)
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is False
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(can_warp=True))
    assert api_fighter_stats.can_warp.one() is True


def test_module_mjfg_cloak(client, consts):
    eve_mjd_subcap_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.micro_jump_portal_drive,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_mjd_cap_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.micro_jump_portal_drive_capital,
        cat_id=consts.EveEffCat.active,
        is_offensive=True)
    eve_mjd_subcap_id = client.mk_eve_item(eff_ids=[eve_mjd_subcap_effect_id], defeff_id=eve_mjd_subcap_effect_id)
    eve_mjd_cap_id = client.mk_eve_item(eff_ids=[eve_mjd_cap_effect_id], defeff_id=eve_mjd_cap_effect_id)
    eve_cloak_effect_id = client.mk_eve_effect(id_=consts.EveEffect.cloaking, cat_id=consts.EveEffCat.active)
    eve_cloak_id = client.mk_eve_item(eff_ids=[eve_cloak_effect_id], defeff_id=eve_cloak_effect_id)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_mjd_subcap = api_fit.add_module(type_id=eve_mjd_subcap_id, state=consts.ApiModuleState.active)
    api_mjd_cap = api_fit.add_module(type_id=eve_mjd_cap_id, state=consts.ApiModuleState.online)
    api_fit.add_module(type_id=eve_cloak_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_mjd_subcap.change_module(state=consts.ApiModuleState.online)
    api_mjd_cap.change_module(state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False


def test_fighter_mjd_stats(client, consts):
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_sig_radius_bonus_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_mjd_sig_radius_bonus)
    eve_mjd_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_mjd, cat_id=consts.EveEffCat.active)
    eve_mjd_abil_id = client.mk_eve_abil(id_=consts.EveAbil.mjd)
    eve_fighter_id = client.mk_eve_fighter(
        attrs={eve_sig_radius_attr_id: 120, eve_sig_radius_bonus_attr_id: 150},
        eff_ids=[eve_mjd_effect_id],
        abils=[client.mk_eve_item_abil(id_=eve_mjd_abil_id)])
    eve_ship_id = client.mk_eve_ship(attrs={eve_sig_radius_attr_id: 18640})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_mjd_abil_id: True})
    # Verification - fighter MJD:
    # - does not affect parent ship
    # - does not prevent fighter from warping regardless of MJD direction (tested on 2026-06-12 on
    #   TQ using Standup Ametat I)
    # - does not blow fighter sig (tested on 2026-06-10 on TQ by bombing Standup Ametat I)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        sig_radius=True,
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    assert api_ship_stats.sig_radius.one() == approx(18640)
    assert api_ship_stats.can_warp.one() is True
    assert api_ship_stats.can_jump_gate.one() is True
    assert api_ship_stats.can_jump_wormhole.one() is True
    assert api_ship_stats.can_jump_drive.one() is True
    assert api_ship_stats.can_dock_station.one() is True
    assert api_ship_stats.can_dock_citadel.one() is True
    assert api_ship_stats.can_tether.one() is False
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(sig_radius=True, can_warp=True))
    assert api_fighter_stats.sig_radius.one() == approx(120)
    assert api_fighter_stats.can_warp.one() is True


def test_fighter_mjd_cloak(client, consts):
    eve_mjd_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_mjd, cat_id=consts.EveEffCat.active)
    eve_mjd_abil_id = client.mk_eve_abil(id_=consts.EveAbil.mjd)
    eve_fighter_id = client.mk_eve_fighter(
        eff_ids=[eve_mjd_effect_id],
        abils=[client.mk_eve_item_abil(id_=eve_mjd_abil_id)])
    eve_cloak_effect_id = client.mk_eve_effect(id_=consts.EveEffect.cloaking, cat_id=consts.EveEffCat.active)
    eve_cloak_id = client.mk_eve_item(eff_ids=[eve_cloak_effect_id], defeff_id=eve_cloak_effect_id)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_mjd_abil_id: True})
    api_fit.add_module(type_id=eve_cloak_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is True
