"""
This module contains tests for various mobility restrictions imposed by various item (either when
used by a fit, or applied onto certain items).
"""

from dataclasses import dataclass

from fw.api import ItemStatsOptions, ValOptions


@dataclass(kw_only=True)
class DdDebuffInfo:
    warp_scram_attr_id: int
    tether_attr_id: int
    docking_attr_id: int


def make_dd_self_debuffs(*, client, consts) -> DdDebuffInfo:
    eve_warp_scram_attr_id = client.mk_eve_attr(id_=consts.EveAttr.siege_mod_warp_status)
    eve_warp_status_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status)
    eve_cloak_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_cloaking)
    eve_tether_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_tethering)
    eve_docking_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_docking)
    eve_drive_jump_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_drive_jumping)
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
    return DdDebuffInfo(
        warp_scram_attr_id=eve_warp_scram_attr_id,
        tether_attr_id=eve_tether_attr_id,
        docking_attr_id=eve_docking_attr_id)


def make_cloak(*, client, consts):
    eve_cloak_effect_id = client.mk_eve_effect(id_=consts.EveEffect.cloaking, cat_id=consts.EveEffCat.active)
    eve_cloak_id = client.mk_eve_item(eff_ids=[eve_cloak_effect_id], defeff_id=eve_cloak_effect_id)
    return eve_cloak_id


def test_dd_direct_amarr(client, consts):
    eve_debuffs = make_dd_self_debuffs(client=client, consts=consts)
    eve_cloak_id = make_cloak(client=client, consts=consts)
    eve_dd_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.super_weapon_amarr,
        cat_id=consts.EveEffCat.target,
        is_offensive=True)
    eve_dd_id = client.mk_eve_item(
        eff_ids=[eve_dd_effect_id],
        defeff_id=eve_dd_effect_id,
        attrs={eve_debuffs.warp_scram_attr_id: 100, eve_debuffs.tether_attr_id: 1, eve_debuffs.docking_attr_id: 1})
    eve_ship_id = client.mk_eve_ship()
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
