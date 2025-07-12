from tests import Muta, approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions


def test_ship_modified(client, consts):
    eve_shield_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_em_dmg_resonance)
    eve_shield_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_therm_dmg_resonance)
    eve_shield_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_kin_dmg_resonance)
    eve_shield_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_expl_dmg_resonance)
    eve_armor_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_em_dmg_resonance)
    eve_armor_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_therm_dmg_resonance)
    eve_armor_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_kin_dmg_resonance)
    eve_armor_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_expl_dmg_resonance)
    eve_hull_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.em_dmg_resonance)
    eve_hull_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.therm_dmg_resonance)
    eve_hull_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.kin_dmg_resonance)
    eve_hull_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.expl_dmg_resonance)
    eve_shield_mod_attr_id = client.mk_eve_attr()
    eve_armor_mod_attr_id = client.mk_eve_attr()
    eve_hull_em_mod_attr_id = client.mk_eve_attr()
    eve_hull_therm_mod_attr_id = client.mk_eve_attr()
    eve_hull_kin_mod_attr_id = client.mk_eve_attr()
    eve_hull_expl_mod_attr_id = client.mk_eve_attr()
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_shield_em_attr_id: 1,
        eve_shield_therm_attr_id: 0.8,
        eve_shield_kin_attr_id: 0.6,
        eve_shield_expl_attr_id: 0.4,
        eve_armor_em_attr_id: 0.5,
        eve_armor_therm_attr_id: 0.65,
        eve_armor_kin_attr_id: 0.75,
        eve_armor_expl_attr_id: 0.7,
        eve_hull_em_attr_id: 0.67,
        eve_hull_therm_attr_id: 0.67,
        eve_hull_kin_attr_id: 0.67,
        eve_hull_expl_attr_id: 0.67})
    eve_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.item,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.pre_mul,
            affector_attr_id=eve_affector_attr_id,
            affectee_attr_id=eve_affectee_attr_id)
        for eve_affector_attr_id, eve_affectee_attr_id in (
            (eve_shield_mod_attr_id, eve_shield_em_attr_id),
            (eve_shield_mod_attr_id, eve_shield_therm_attr_id),
            (eve_shield_mod_attr_id, eve_shield_kin_attr_id),
            (eve_shield_mod_attr_id, eve_shield_expl_attr_id),
            (eve_armor_mod_attr_id, eve_armor_em_attr_id),
            (eve_armor_mod_attr_id, eve_armor_therm_attr_id),
            (eve_armor_mod_attr_id, eve_armor_kin_attr_id),
            (eve_armor_mod_attr_id, eve_armor_expl_attr_id),
            (eve_hull_em_mod_attr_id, eve_hull_em_attr_id),
            (eve_hull_therm_mod_attr_id, eve_hull_therm_attr_id),
            (eve_hull_kin_mod_attr_id, eve_hull_kin_attr_id),
            (eve_hull_expl_mod_attr_id, eve_hull_expl_attr_id))]
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=eve_mods)
    eve_base_module_id = client.mk_eve_item(
        attrs={
            eve_shield_mod_attr_id: 0.875,
            eve_armor_mod_attr_id: 0.85,
            eve_hull_em_mod_attr_id: 0.6,
            eve_hull_therm_mod_attr_id: 0.6,
            eve_hull_kin_mod_attr_id: 0.6,
            eve_hull_expl_mod_attr_id: 0.6},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_mutated_module_id = client.mk_eve_item(eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={
            eve_hull_em_mod_attr_id: (0.9, 1.05),
            eve_hull_therm_mod_attr_id: (0.9, 1.05),
            eve_hull_kin_mod_attr_id: (0.9, 1.05),
            eve_hull_expl_mod_attr_id: (0.9, 1.05)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(resists=True))
    assert api_fit_stats.resists.shield == (approx(0), approx(0.2), approx(0.4), approx(0.6))
    assert api_fit_stats.resists.armor == (approx(0.5), approx(0.35), approx(0.25), approx(0.3))
    assert api_fit_stats.resists.hull == (approx(0.33), approx(0.33), approx(0.33), approx(0.33))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(resists=True))
    assert api_ship_stats.resists.shield == (approx(0), approx(0.2), approx(0.4), approx(0.6))
    assert api_ship_stats.resists.armor == (approx(0.5), approx(0.35), approx(0.25), approx(0.3))
    assert api_ship_stats.resists.hull == (approx(0.33), approx(0.33), approx(0.33), approx(0.33))
    # Action
    api_module = api_fit.add_module(type_id=eve_base_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(resists=True))
    assert api_fit_stats.resists.shield == (approx(0.125), approx(0.3), approx(0.475), approx(0.65))
    assert api_fit_stats.resists.armor == (approx(0.575), approx(0.4475), approx(0.3625), approx(0.405))
    assert api_fit_stats.resists.hull == (approx(0.598), approx(0.598), approx(0.598), approx(0.598))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(resists=True))
    assert api_ship_stats.resists.shield == (approx(0.125), approx(0.3), approx(0.475), approx(0.65))
    assert api_ship_stats.resists.armor == (approx(0.575), approx(0.4475), approx(0.3625), approx(0.405))
    assert api_ship_stats.resists.hull == (approx(0.598), approx(0.598), approx(0.598), approx(0.598))
    # Action
    api_module.change_module(mutation=(eve_mutator_id, {
        eve_hull_em_mod_attr_id: Muta.roll_to_api(val=0.22),
        eve_hull_therm_mod_attr_id: Muta.roll_to_api(val=0.87),
        eve_hull_kin_mod_attr_id: Muta.roll_to_api(val=0.64),
        eve_hull_expl_mod_attr_id: Muta.roll_to_api(val=0.43)}))
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(resists=True))
    assert api_fit_stats.resists.shield == (approx(0.125), approx(0.3), approx(0.475), approx(0.65))
    assert api_fit_stats.resists.armor == (approx(0.575), approx(0.4475), approx(0.3625), approx(0.405))
    assert api_fit_stats.resists.hull == (approx(0.624934), approx(0.585739), approx(0.599608), approx(0.612271))
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(resists=True))
    assert api_ship_stats.resists.shield == (approx(0.125), approx(0.3), approx(0.475), approx(0.65))
    assert api_ship_stats.resists.armor == (approx(0.575), approx(0.4475), approx(0.3625), approx(0.405))
    assert api_ship_stats.resists.hull == (approx(0.624934), approx(0.585739), approx(0.599608), approx(0.612271))


def test_no_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.shield_em_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.shield_therm_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.shield_kin_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.shield_expl_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_em_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_therm_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_kin_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_expl_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.em_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.therm_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.kin_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.expl_dmg_resonance)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(resists=True))
    assert api_fit_stats.resists is None


def test_not_loaded_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.shield_em_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.shield_therm_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.shield_kin_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.shield_expl_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_em_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_therm_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_kin_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.armor_expl_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.em_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.therm_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.kin_dmg_resonance)
    client.mk_eve_attr(id_=consts.EveAttr.expl_dmg_resonance)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(resists=True))
    assert api_fit_stats.resists is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(resists=True))
    assert api_ship_stats.resists is None


def test_drone_modified(client, consts):
    eve_max_attr_id = client.mk_eve_attr(def_val=1)
    eve_shield_em_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.shield_em_dmg_resonance,
        max_attr_id=eve_max_attr_id)
    eve_shield_therm_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.shield_therm_dmg_resonance,
        max_attr_id=eve_max_attr_id)
    eve_shield_kin_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.shield_kin_dmg_resonance,
        max_attr_id=eve_max_attr_id)
    eve_shield_expl_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.shield_expl_dmg_resonance,
        max_attr_id=eve_max_attr_id)
    eve_armor_em_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_em_dmg_resonance,
        max_attr_id=eve_max_attr_id)
    eve_armor_therm_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_therm_dmg_resonance,
        max_attr_id=eve_max_attr_id)
    eve_armor_kin_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_kin_dmg_resonance,
        max_attr_id=eve_max_attr_id)
    eve_armor_expl_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_expl_dmg_resonance,
        max_attr_id=eve_max_attr_id)
    eve_hull_em_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.em_dmg_resonance,
        max_attr_id=eve_max_attr_id)
    eve_hull_therm_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.therm_dmg_resonance,
        max_attr_id=eve_max_attr_id)
    eve_hull_kin_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.kin_dmg_resonance,
        max_attr_id=eve_max_attr_id)
    eve_hull_expl_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.expl_dmg_resonance,
        max_attr_id=eve_max_attr_id)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[
            client.mk_eve_buff_mod(attr_id=eve_shield_therm_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_armor_therm_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_hull_therm_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 70},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_ship(attrs={
        eve_shield_em_attr_id: 1,
        eve_shield_therm_attr_id: 0.8,
        eve_shield_kin_attr_id: 0.6,
        eve_shield_expl_attr_id: 0.5,
        eve_armor_em_attr_id: 0.5,
        eve_armor_therm_attr_id: 0.55,
        eve_armor_kin_attr_id: 0.75,
        eve_armor_expl_attr_id: 0.9,
        eve_hull_em_attr_id: 1,
        eve_hull_therm_attr_id: 1,
        eve_hull_kin_attr_id: 1,
        eve_hull_expl_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(resists=True))
    assert api_drone_stats.resists.shield == (approx(0), approx(0.2), approx(0.4), approx(0.5))
    assert api_drone_stats.resists.armor == (approx(0.5), approx(0.45), approx(0.25), approx(0.1))
    assert api_drone_stats.resists.hull == (approx(0), approx(0), approx(0), approx(0))
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(resists=True))
    assert api_drone_stats.resists.shield == (approx(0), approx(0), approx(0.4), approx(0.5))
    assert api_drone_stats.resists.armor == (approx(0.5), approx(0.065), approx(0.25), approx(0.1))
    assert api_drone_stats.resists.hull == (approx(0), approx(0), approx(0), approx(0))
    # Action
    api_fw_effect.remove()
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(resists=True))
    assert api_drone_stats.resists.shield == (approx(0), approx(0.2), approx(0.4), approx(0.5))
    assert api_drone_stats.resists.armor == (approx(0.5), approx(0.45), approx(0.25), approx(0.1))
    assert api_drone_stats.resists.hull == (approx(0), approx(0), approx(0), approx(0))


def test_fighter_modified(client, consts):
    eve_max_attr_id = client.mk_eve_attr(def_val=1)
    eve_shield_em_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.shield_em_dmg_resonance,
        def_val=1,
        max_attr_id=eve_max_attr_id)
    eve_shield_therm_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.shield_therm_dmg_resonance,
        def_val=1,
        max_attr_id=eve_max_attr_id)
    eve_shield_kin_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.shield_kin_dmg_resonance,
        def_val=1,
        max_attr_id=eve_max_attr_id)
    eve_shield_expl_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.shield_expl_dmg_resonance,
        def_val=1,
        max_attr_id=eve_max_attr_id)
    client.mk_eve_attr(id_=consts.EveAttr.armor_em_dmg_resonance, def_val=1, max_attr_id=eve_max_attr_id)
    eve_armor_therm_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_therm_dmg_resonance,
        def_val=1,
        max_attr_id=eve_max_attr_id)
    client.mk_eve_attr(id_=consts.EveAttr.armor_kin_dmg_resonance, def_val=1, max_attr_id=eve_max_attr_id)
    client.mk_eve_attr(id_=consts.EveAttr.armor_expl_dmg_resonance, def_val=1, max_attr_id=eve_max_attr_id)
    client.mk_eve_attr(id_=consts.EveAttr.em_dmg_resonance, def_val=1, max_attr_id=eve_max_attr_id)
    eve_hull_therm_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.therm_dmg_resonance,
        def_val=1,
        max_attr_id=eve_max_attr_id)
    client.mk_eve_attr(id_=consts.EveAttr.kin_dmg_resonance, def_val=1, max_attr_id=eve_max_attr_id)
    client.mk_eve_attr(id_=consts.EveAttr.expl_dmg_resonance, def_val=1, max_attr_id=eve_max_attr_id)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[
            client.mk_eve_buff_mod(attr_id=eve_shield_therm_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_armor_therm_attr_id),
            client.mk_eve_buff_mod(attr_id=eve_hull_therm_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 70},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_fighter_id = client.mk_eve_ship(attrs={
        eve_shield_em_attr_id: 0.7,
        eve_shield_therm_attr_id: 0.85,
        eve_shield_kin_attr_id: 1,
        eve_shield_expl_attr_id: 1,
        eve_max_count_attr_id: 9})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(resists=True))
    assert api_fighter_stats.resists.shield == (approx(0.3), approx(0.15), approx(0), approx(0))
    assert api_fighter_stats.resists.armor == (approx(0), approx(0), approx(0), approx(0))
    assert api_fighter_stats.resists.hull == (approx(0), approx(0), approx(0), approx(0))
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(resists=True))
    assert api_fighter_stats.resists.shield == (approx(0.3), approx(0), approx(0), approx(0))
    assert api_fighter_stats.resists.armor == (approx(0), approx(0), approx(0), approx(0))
    assert api_fighter_stats.resists.hull == (approx(0), approx(0), approx(0), approx(0))
    # Action
    api_fw_effect.remove()
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(resists=True))
    assert api_fighter_stats.resists.shield == (approx(0.3), approx(0.15), approx(0), approx(0))
    assert api_fighter_stats.resists.armor == (approx(0), approx(0), approx(0), approx(0))
    assert api_fighter_stats.resists.hull == (approx(0), approx(0), approx(0), approx(0))
