from tests import Muta, approx
from tests.fw.api import StatsFitOptions


def test_modified(client, consts):
    eve_shield_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_em_dmg_resonance)
    eve_shield_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_therm_dmg_resonance)
    eve_shield_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_kin_dmg_resonance)
    eve_shield_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_expl_dmg_resonance)
    eve_armor_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_em_dmg_resonance)
    eve_armor_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_therm_dmg_resonance)
    eve_armor_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_kin_dmg_resonance)
    eve_armor_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_expl_dmg_resonance)
    eve_struct_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.em_dmg_resonance)
    eve_struct_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.therm_dmg_resonance)
    eve_struct_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.kin_dmg_resonance)
    eve_struct_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.expl_dmg_resonance)
    eve_shield_mod_attr_id = client.mk_eve_attr()
    eve_armor_mod_attr_id = client.mk_eve_attr()
    eve_struct_em_mod_attr_id = client.mk_eve_attr()
    eve_struct_therm_mod_attr_id = client.mk_eve_attr()
    eve_struct_kin_mod_attr_id = client.mk_eve_attr()
    eve_struct_expl_mod_attr_id = client.mk_eve_attr()
    eve_ship_id = client.mk_eve_ship(attrs={
        eve_shield_em_attr_id: 1,
        eve_shield_therm_attr_id: 0.8,
        eve_shield_kin_attr_id: 0.6,
        eve_shield_expl_attr_id: 0.4,
        eve_armor_em_attr_id: 0.5,
        eve_armor_therm_attr_id: 0.65,
        eve_armor_kin_attr_id: 0.75,
        eve_armor_expl_attr_id: 0.7,
        eve_struct_em_attr_id: 0.67,
        eve_struct_therm_attr_id: 0.67,
        eve_struct_kin_attr_id: 0.67,
        eve_struct_expl_attr_id: 0.67})
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
            (eve_struct_em_mod_attr_id, eve_struct_em_attr_id),
            (eve_struct_therm_mod_attr_id, eve_struct_therm_attr_id),
            (eve_struct_kin_mod_attr_id, eve_struct_kin_attr_id),
            (eve_struct_expl_mod_attr_id, eve_struct_expl_attr_id))]
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=eve_mods)
    eve_base_module_id = client.mk_eve_item(
        attrs={
            eve_shield_mod_attr_id: 0.875,
            eve_armor_mod_attr_id: 0.85,
            eve_struct_em_mod_attr_id: 0.6,
            eve_struct_therm_mod_attr_id: 0.6,
            eve_struct_kin_mod_attr_id: 0.6,
            eve_struct_expl_mod_attr_id: 0.6},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_mutated_module_id = client.mk_eve_item(eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={
            eve_struct_em_mod_attr_id: (0.9, 1.05),
            eve_struct_therm_mod_attr_id: (0.9, 1.05),
            eve_struct_kin_mod_attr_id: (0.9, 1.05),
            eve_struct_expl_mod_attr_id: (0.9, 1.05)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(resists=True))
    assert api_stats.resists.shield == (approx(0), approx(0.2), approx(0.4), approx(0.6))
    assert api_stats.resists.armor == (approx(0.5), approx(0.35), approx(0.25), approx(0.3))
    assert api_stats.resists.hull == (approx(0.33), approx(0.33), approx(0.33), approx(0.33))
    # Action
    api_module = api_fit.add_module(type_id=eve_base_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(resists=True))
    assert api_stats.resists.shield == (approx(0.125), approx(0.3), approx(0.475), approx(0.65))
    assert api_stats.resists.armor == (approx(0.575), approx(0.4475), approx(0.3625), approx(0.405))
    assert api_stats.resists.hull == (approx(0.598), approx(0.598), approx(0.598), approx(0.598))
    # Action
    api_module.change_module(mutation=(eve_mutator_id, {
        eve_struct_em_mod_attr_id: Muta.roll_to_api(val=0.22),
        eve_struct_therm_mod_attr_id: Muta.roll_to_api(val=0.87),
        eve_struct_kin_mod_attr_id: Muta.roll_to_api(val=0.64),
        eve_struct_expl_mod_attr_id: Muta.roll_to_api(val=0.43)}))
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(resists=True))
    assert api_stats.resists.shield == (approx(0.125), approx(0.3), approx(0.475), approx(0.65))
    assert api_stats.resists.armor == (approx(0.575), approx(0.4475), approx(0.3625), approx(0.405))
    assert api_stats.resists.hull == (approx(0.624934), approx(0.585739), approx(0.599608), approx(0.612271))


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
    api_stats = api_fit.get_stats(options=StatsFitOptions(resists=True))
    assert api_stats.resists is None


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
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsFitOptions(resists=True))
    assert api_stats.resists is None
