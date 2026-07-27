from fw import approx
from fw.api import FitStatsOptions


def test_debuff_rr(client, consts):
    eve_affectee_attr_id = client.mk_eve_attr(stackable=True)
    client.mk_eve_buff(
        id_=consts.EveBuff.remote_repair_impedance,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.debuff_lance, cat_id=consts.EveEffCat.active)
    eve_affector_module_id = client.mk_eve_item(eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_fit1 = api_sol.create_fit()
    api_affector_module1 = api_affector_fit1.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affector_module1.change_module(add_proj_item_ids=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].modified == approx(0.5)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_str == approx(-50)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_str == approx(-50)
    assert api_mod.affectors.one().item_id == api_affector_module1.id
    assert api_mod.affectors.one().attr_id is None
    # Action
    api_affector_fit2 = api_sol.create_fit()
    api_affector_module2 = api_affector_fit2.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affector_module2.change_module(add_proj_item_ids=[api_affectee_ship.id])
    # Verification - no stacking, lances are applied via debuff
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].modified == approx(0.5)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_str == approx(-50)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_str == approx(-50)
    assert api_mod.affectors.one().item_id in {api_affector_module1.id, api_affector_module2.id}
    assert api_mod.affectors.one().attr_id is None


def test_debuff_warp(client, consts):
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_scramble_status, stackable=True)
    client.mk_eve_buff(
        id_=consts.EveBuff.warp_penalty,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.debuff_lance, cat_id=consts.EveEffCat.active)
    eve_affector_module_id = client.mk_eve_item(eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_fit1 = api_sol.create_fit()
    api_affector_module1 = api_affector_fit1.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affector_module1.change_module(add_proj_item_ids=[api_affectee_ship.id])
    # Verification
    api_affectee_fit_stats = api_affectee_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True))
    assert api_affectee_fit_stats.can_warp.one() is False
    assert api_affectee_fit_stats.can_jump_drive.one() is False
    assert api_affectee_fit_stats.can_dock_citadel.one() is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].modified == approx(100)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.mod_add
    assert api_mod.initial_str == approx(100)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_str == approx(100)
    assert api_mod.affectors.one().item_id == api_affector_module1.id
    assert api_mod.affectors.one().attr_id is None
    # Action
    api_affector_fit2 = api_sol.create_fit()
    api_affector_module2 = api_affector_fit2.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affector_module2.change_module(add_proj_item_ids=[api_affectee_ship.id])
    # Verification - no stacking, lances are applied via debuff
    api_affectee_fit_stats = api_affectee_fit.get_stats(
        options=FitStatsOptions(can_warp=True, can_jump_drive=True, can_dock_citadel=True))
    assert api_affectee_fit_stats.can_warp.one() is False
    assert api_affectee_fit_stats.can_jump_drive.one() is False
    assert api_affectee_fit_stats.can_dock_citadel.one() is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].modified == approx(100)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.mod_add
    assert api_mod.initial_str == approx(100)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_str == approx(100)
    assert api_mod.affectors.one().item_id in {api_affector_module1.id, api_affector_module2.id}
    assert api_mod.affectors.one().attr_id is None


def test_debuff_dock_jump(client, consts):
    eve_affectee_dock_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_docking, stackable=True)
    eve_affectee_jump_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_drive_jumping, stackable=True)
    client.mk_eve_buff(
        id_=consts.EveBuff.disallow_dock_jump,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[
            client.mk_eve_buff_mod(attr_id=eve_affectee_dock_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_affectee_jump_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.debuff_lance, cat_id=consts.EveEffCat.active)
    eve_affector_module_id = client.mk_eve_item(eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_dock_attr_id: 0, eve_affectee_jump_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_fit1 = api_sol.create_fit()
    api_affector_module1 = api_affector_fit1.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affector_module1.change_module(add_proj_item_ids=[api_affectee_ship.id])
    # Verification
    api_affectee_fit_stats = api_affectee_fit.get_stats(
        options=FitStatsOptions(
            can_dock_station=True,
            can_dock_citadel=True,
            can_jump_gate=True,
            can_jump_wormhole=True,
            can_jump_drive=True))
    assert api_affectee_fit_stats.can_dock_station.one() is False
    assert api_affectee_fit_stats.can_dock_citadel.one() is False
    assert api_affectee_fit_stats.can_jump_gate.one() is False
    assert api_affectee_fit_stats.can_jump_wormhole.one() is True
    assert api_affectee_fit_stats.can_jump_drive.one() is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_dock_attr_id].modified == approx(1)
    assert api_affectee_ship.attrs[eve_affectee_jump_attr_id].modified == approx(1)
    api_dock_mod = api_affectee_ship.mods[eve_affectee_dock_attr_id].one()
    assert api_dock_mod.op == consts.ApiModOp.mod_add
    assert api_dock_mod.initial_str == approx(1)
    assert api_dock_mod.stacking_mult is None
    assert api_dock_mod.initial_str == approx(1)
    assert api_dock_mod.affectors.one().item_id == api_affector_module1.id
    assert api_dock_mod.affectors.one().attr_id is None
    api_jump_mod = api_affectee_ship.mods[eve_affectee_jump_attr_id].one()
    assert api_jump_mod.op == consts.ApiModOp.mod_add
    assert api_jump_mod.initial_str == approx(1)
    assert api_jump_mod.stacking_mult is None
    assert api_jump_mod.initial_str == approx(1)
    assert api_jump_mod.affectors.one().item_id == api_affector_module1.id
    assert api_jump_mod.affectors.one().attr_id is None
    # Action
    api_affector_fit2 = api_sol.create_fit()
    api_affector_module2 = api_affector_fit2.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affector_module2.change_module(add_proj_item_ids=[api_affectee_ship.id])
    # Verification - no stacking, lances are applied via debuff
    api_affectee_fit_stats = api_affectee_fit.get_stats(
        options=FitStatsOptions(
            can_dock_station=True,
            can_dock_citadel=True,
            can_jump_gate=True,
            can_jump_wormhole=True,
            can_jump_drive=True))
    assert api_affectee_fit_stats.can_dock_station.one() is False
    assert api_affectee_fit_stats.can_dock_citadel.one() is False
    assert api_affectee_fit_stats.can_jump_gate.one() is False
    assert api_affectee_fit_stats.can_jump_wormhole.one() is True
    assert api_affectee_fit_stats.can_jump_drive.one() is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_dock_attr_id].modified == approx(1)
    assert api_affectee_ship.attrs[eve_affectee_jump_attr_id].modified == approx(1)
    api_dock_mod = api_affectee_ship.mods[eve_affectee_dock_attr_id].one()
    assert api_dock_mod.op == consts.ApiModOp.mod_add
    assert api_dock_mod.initial_str == approx(1)
    assert api_dock_mod.stacking_mult is None
    assert api_dock_mod.initial_str == approx(1)
    assert api_dock_mod.affectors.one().item_id in {api_affector_module1.id, api_affector_module2.id}
    assert api_dock_mod.affectors.one().attr_id is None
    api_jump_mod = api_affectee_ship.mods[eve_affectee_jump_attr_id].one()
    assert api_jump_mod.op == consts.ApiModOp.mod_add
    assert api_jump_mod.initial_str == approx(1)
    assert api_jump_mod.stacking_mult is None
    assert api_jump_mod.initial_str == approx(1)
    assert api_jump_mod.affectors.one().item_id in {api_affector_module1.id, api_affector_module2.id}
    assert api_jump_mod.affectors.one().attr_id is None


def test_debuff_tether(client, consts):
    eve_affectee_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_tethering, stackable=True)
    client.mk_eve_buff(
        id_=consts.EveBuff.disallow_tether,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.debuff_lance, cat_id=consts.EveEffCat.active)
    eve_affector_module_id = client.mk_eve_item(eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_fit1 = api_sol.create_fit()
    api_affector_module1 = api_affector_fit1.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affector_module1.change_module(add_proj_item_ids=[api_affectee_ship.id])
    # Verification
    api_affectee_fit_stats = api_affectee_fit.get_stats(options=FitStatsOptions(can_tether=True))
    assert api_affectee_fit_stats.can_tether.one() is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].modified == approx(1)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.mod_add
    assert api_mod.initial_str == approx(1)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_str == approx(1)
    assert api_mod.affectors.one().item_id == api_affector_module1.id
    assert api_mod.affectors.one().attr_id is None
    # Action
    api_affector_fit2 = api_sol.create_fit()
    api_affector_module2 = api_affector_fit2.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affector_module2.change_module(add_proj_item_ids=[api_affectee_ship.id])
    # Verification - no stacking, lances are applied via debuff
    api_affectee_fit_stats = api_affectee_fit.get_stats(options=FitStatsOptions(can_tether=True))
    assert api_affectee_fit_stats.can_tether.one() is False
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].modified == approx(1)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.mod_add
    assert api_mod.initial_str == approx(1)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_str == approx(1)
    assert api_mod.affectors.one().item_id in {api_affector_module1.id, api_affector_module2.id}
    assert api_mod.affectors.one().attr_id is None


def test_drone(client, consts):
    eve_affectee_attr_id = client.mk_eve_attr(stackable=True)
    client.mk_eve_buff(
        id_=consts.EveBuff.remote_repair_impedance,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.debuff_lance, cat_id=consts.EveEffCat.active)
    eve_affector_module_id = client.mk_eve_item(eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_affectee_drone_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affector_module.change_module(add_proj_item_ids=[api_affectee_drone.id])
    # Verification
    api_affectee_drone.update()
    assert api_affectee_drone.attrs[eve_affectee_attr_id].modified == approx(0.5)
    api_mod = api_affectee_drone.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_str == approx(-50)
    assert api_mod.stacking_mult is None
    assert api_mod.initial_str == approx(-50)
    assert api_mod.affectors.one().item_id == api_affector_module.id
    assert api_mod.affectors.one().attr_id is None


def test_range_modified(client, consts):
    eve_affectee_attr_id = client.mk_eve_attr(stackable=True)
    eve_range_optimal_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_range)
    eve_range_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_dmg_radius)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_mod_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.remote_repair_impedance,
        aggr_mode=consts.EveBuffAggrMode.min,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_lance_effect_id = client.mk_eve_effect(id_=consts.EveEffect.debuff_lance, cat_id=consts.EveEffCat.active)
    eve_lance_id = client.mk_eve_item(
        attrs={eve_range_optimal_attr_id: 100000, eve_range_radius_attr_id: 2500},
        eff_ids=[eve_lance_effect_id],
        defeff_id=eve_lance_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 15000})
    eve_tgt_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1, eve_radius_attr_id: 8000})
    eve_optimal_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_range_optimal_attr_id)
    eve_optimal_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_optimal_mod])
    eve_optimal_rig = client.mk_eve_item(attrs={eve_mod_attr_id: -50}, eff_ids=[eve_optimal_effect_id])
    eve_radius_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_range_radius_attr_id)
    eve_radius_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_radius_mod])
    eve_radius_rig = client.mk_eve_item(attrs={eve_mod_attr_id: 100}, eff_ids=[eve_radius_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_lance = api_src_fit.add_module(type_id=eve_lance_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0))
    api_lance.change_module(add_proj_item_ids=[api_tgt_ship.id])
    # Verification - target within attacking ship radius still affected by the beam. Tested on
    # Singularity on 2026-07-26 by relative-warping Karura onto bookmark in the center of a frig and
    # DD'ing
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(0.5)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 125499, 0))
    # Verification - within surface-to-surface range of optimal + damage radius. Damage radius is
    # added to beam range, most likely like a semi-sphere. Tested on Singularity on 2026-07-26 by
    # putting target at ~102400 meters surface-to-surface range, and then moving target out of
    # damage range while DD is still firing
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(0.5)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 125501, 0))
    # Verification - slightly out range
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(1)
    # Action
    api_src_fit.add_rig(type_id=eve_radius_rig)
    # Verification - now in range thanks to increased beam radius
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(0.5)
    # Action
    api_src_fit.add_rig(type_id=eve_optimal_rig)
    # Verification - out of range due to decreased main/optimal range
    assert api_tgt_ship.update().attrs[eve_affectee_attr_id].modified == approx(1)
