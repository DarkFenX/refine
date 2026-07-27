"""
This module contains tests for various mobility restrictions imposed by various item (either when
used by a fit, or applied onto certain items).
"""

from fw.api import ItemStatsOptions, ValOptions


def make_cloak(*, client, consts):
    eve_cloak_effect_id = client.mk_eve_effect(id_=consts.EveEffect.cloaking, cat_id=consts.EveEffCat.active)
    return client.mk_eve_item(eff_ids=[eve_cloak_effect_id], defeff_id=eve_cloak_effect_id)


def make_assistive_module(*, client, consts):
    eve_range_attr_id = client.mk_eve_attr()
    eve_assist_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        is_assistance=True,
        range_attr_id=eve_range_attr_id)
    return client.mk_eve_item(
        attrs={eve_range_attr_id: 10000},
        eff_ids=[eve_assist_effect_id],
        defeff_id=eve_assist_effect_id)


def make_offensive_module(*, client, consts):
    eve_mod_src_attr_id = client.mk_eve_attr()
    eve_mod_tgt_attr_id = client.mk_eve_attr()
    eve_range_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_src_attr_id,
        affectee_attr_id=eve_mod_tgt_attr_id)
    eve_offense_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        is_offensive=True,
        range_attr_id=eve_range_attr_id,
        mod_info=[eve_mod])
    return client.mk_eve_item(
        attrs={eve_mod_src_attr_id: -50, eve_range_attr_id: 10000},
        eff_ids=[eve_offense_effect_id],
        defeff_id=eve_offense_effect_id)


def run_dd_test(*, client, consts, dd_effect_id: int, is_targeted: bool = False):
    eve_warp_scram_attr_id = client.mk_eve_attr(id_=consts.EveAttr.siege_mod_warp_status)
    eve_warp_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_cloak_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_cloaking)
    eve_tether_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_tethering)
    eve_docking_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_docking)
    eve_drive_jump_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_drive_jumping)
    client.mk_eve_attr(id_=consts.EveAttr.disallow_assistance)
    client.mk_eve_attr(id_=consts.EveAttr.disallow_offensive_modifiers)
    client.mk_eve_buff(
        id_=consts.EveBuff.warp_penalty,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_warp_status_attr_id)])
    client.mk_eve_buff(
        id_=consts.EveBuff.disallow_dock_jump,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[
            client.mk_eve_buff_mod(attr_id=eve_docking_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_drive_jump_attr_id)])
    client.mk_eve_buff(
        id_=consts.EveBuff.disallow_tether,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tether_attr_id)])
    client.mk_eve_buff(
        id_=consts.EveBuff.disallow_cloak,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_cloak_attr_id)])
    eve_dd_effect_id = client.mk_eve_effect(
        id_=dd_effect_id,
        cat_id=consts.EveEffCat.target if is_targeted else consts.EveEffCat.active,
        is_offensive=True)
    eve_dd_id = client.mk_eve_item(
        eff_ids=[eve_dd_effect_id],
        defeff_id=eve_dd_effect_id,
        attrs={eve_warp_scram_attr_id: 100, eve_tether_attr_id: 1, eve_docking_attr_id: 1})
    eve_ship_id = client.mk_eve_ship()
    eve_cloak_id = make_cloak(client=client, consts=consts)
    eve_assist_id = make_assistive_module(client=client, consts=consts)
    eve_offense_id = make_offensive_module(client=client, consts=consts)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_dd_id, state=consts.ApiModuleState.active)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(
        can_warp=True,
        can_jump_gate=True,
        can_jump_wormhole=True,
        can_jump_drive=True,
        can_dock_station=True,
        can_dock_citadel=True,
        can_tether=True))
    api_ship.update()
    assert api_ship_stats.can_warp.one() is False
    assert api_ship_stats.can_jump_gate.one() is False
    assert api_ship_stats.can_jump_wormhole.one() is True
    assert api_ship_stats.can_jump_drive.one() is False
    assert api_ship_stats.can_dock_station.one() is False
    assert api_ship_stats.can_dock_citadel.one() is False
    assert api_ship_stats.can_tether.one() is False
    # Action
    api_fit.add_module(type_id=eve_cloak_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_fit.validate(options=ValOptions(cloaking_blocked=True)).passed is False
    # Action
    api_proj_fit = api_sol.create_fit()
    api_proj_fit.add_module(type_id=eve_assist_id, state=consts.ApiModuleState.active, proj_item_ids=[api_ship.id])
    api_proj_fit.add_module(type_id=eve_offense_id, state=consts.ApiModuleState.active, proj_item_ids=[api_ship.id])
    # Verification
    assert api_proj_fit.validate(options=ValOptions(assist_immunity=True, offense_immunity=True)).passed is True


def test_dd_direct_amarr(client, consts):
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.super_weapon_amarr, is_targeted=True)


def test_dd_direct_caldari(client, consts):
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.super_weapon_caldari, is_targeted=True)


def test_dd_direct_gallente(client, consts):
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.super_weapon_gallente, is_targeted=True)


def test_dd_direct_minmatar(client, consts):
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.super_weapon_minmatar, is_targeted=True)


def test_dd_lance(client, consts):
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.doomsday_beam_dot)


def test_dd_reaper(client, consts):
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.doomsday_slash)


def test_dd_bosonic(client, consts):
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.doomsday_cone_dot)


def test_dd_gtfo(client, consts):
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.doomsday_hog)


def test_dd_debuff_lance(client, consts):
    run_dd_test(client=client, consts=consts, dd_effect_id=consts.EveEffect.debuff_lance)
