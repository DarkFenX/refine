from tests import approx
from tests.fw.api import StatsOptions


def test_buffer_modified(client, consts):
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 3000, eve_armor_attr_id: 2000, eve_structure_attr_id: 1000})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mods = [
        client.mk_eve_effect_mod(
            func=consts.EveModFunc.item,
            loc=consts.EveModLoc.ship,
            op=consts.EveModOp.post_percent,
            affector_attr_id=eve_mod_attr_id,
            affectee_attr_id=eve_layer_attr)
        for eve_layer_attr in (eve_shield_attr_id, eve_armor_attr_id, eve_structure_attr_id)]
    eve_mod_effect_id = client.mk_eve_effect(mod_info=eve_mods)
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 25}, eff_ids=[eve_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3750), 0, 0)
    assert api_stats.hp.armor == (approx(2500), 0, 0)
    assert api_stats.hp.structure == (approx(1250), 0, 0)
    # Action
    api_rig.remove()
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)


def test_local_asb_accuracy_and_charge_switch(client, consts):
    # Accuracy = cases like 2.3 / 0.1 = 22.999999999999996
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_bonus)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_rep_effect_id = client.mk_eve_effect(id_=consts.EveEffect.fueled_shield_boosting, cat_id=consts.EveEffCat.active)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 3000, eve_armor_attr_id: 2000, eve_structure_attr_id: 1000})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_amount_attr_id: 300,
            eve_capacity_attr_id: 2.3,
            eve_charge_rate_attr_id: 1},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_asb = api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), approx(6900), 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_asb.change_module(charge_type_id=None)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_asb.change_module(charge_type_id=eve_charge_item_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), approx(6900), 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_asb.remove()
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)


def test_local_asb_state_switch(client, consts):
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_bonus)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_rep_effect_id = client.mk_eve_effect(id_=consts.EveEffect.fueled_shield_boosting, cat_id=consts.EveEffCat.active)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 3000, eve_armor_attr_id: 2000, eve_structure_attr_id: 1000})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_amount_attr_id: 300,
            eve_capacity_attr_id: 3,
            eve_charge_rate_attr_id: 1},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_asb = api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), approx(900), 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_asb.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_asb.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), approx(900), 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_asb.remove()
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)


def test_local_asb_modified_and_rep_hp_limit(client, consts):
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_bonus)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_shield_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_shield_attr_id)
    eve_rep_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_rep_amount_attr_id)
    eve_rep_effect_id = client.mk_eve_effect(id_=consts.EveEffect.fueled_shield_boosting, cat_id=consts.EveEffCat.active)
    eve_shield_mod_effect_id = client.mk_eve_effect(mod_info=[eve_shield_mod])
    eve_rep_mod_effect_id = client.mk_eve_effect(mod_info=[eve_rep_mod])
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 1000, eve_armor_attr_id: 500, eve_structure_attr_id: 250})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_amount_attr_id: 1500,
            eve_capacity_attr_id: 112,
            eve_charge_rate_attr_id: 1},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 12})
    eve_hp_rig = client.mk_eve_item(attrs={eve_mod_attr_id: 70}, eff_ids=[eve_shield_mod_effect_id])
    eve_rep_rig = client.mk_eve_item(attrs={eve_mod_attr_id: 70}, eff_ids=[eve_rep_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification - reps are limited from 1500 / cycle to 1000 / cycle
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(1000), approx(9000), 0)
    assert api_stats.hp.armor == (approx(500), 0, 0)
    assert api_stats.hp.structure == (approx(250), 0, 0)
    # Action
    api_fit.add_rig(type_id=eve_hp_rig)
    # Verification - no limit now, with shield HP increased
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(1700), approx(13500), 0)
    assert api_stats.hp.armor == (approx(500), 0, 0)
    assert api_stats.hp.structure == (approx(250), 0, 0)
    # Action
    api_fit.add_rig(type_id=eve_rep_rig)
    # Verification - limited again from 2550 / cycle to 1700 / cycle
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(1700), approx(15300), 0)
    assert api_stats.hp.armor == (approx(500), 0, 0)
    assert api_stats.hp.structure == (approx(250), 0, 0)


def test_local_aar_accuracy_and_charge_switch(client, consts):
    # Accuracy = cases like 2.3 / 0.1 = 22.999999999999996
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_rep_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_rep_effect_id = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair, cat_id=consts.EveEffCat.active)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 3000, eve_armor_attr_id: 2000, eve_structure_attr_id: 1000})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_mult_attr_id: 3,
            eve_rep_amount_attr_id: 100,
            eve_capacity_attr_id: 2.3,
            eve_charge_rate_attr_id: 1},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_aar = api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), approx(6900), 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_aar.change_module(charge_type_id=None)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_aar.change_module(charge_type_id=eve_charge_item_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), approx(6900), 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_aar.remove()
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)


def test_local_aar_charge_rate_rounding_and_state_switch(client, consts):
    # Rounding in this case means the way lib considers not-fully-charged-cycle
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_rep_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_rep_effect_id = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair, cat_id=consts.EveEffCat.active)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 3000, eve_armor_attr_id: 2000, eve_structure_attr_id: 1000})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_mult_attr_id: 3,
            eve_rep_amount_attr_id: 100,
            eve_capacity_attr_id: 15,
            eve_charge_rate_attr_id: 4},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_aar = api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification - count of cycles is floored (i.e. forced to reload on partially charged cycles)
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), approx(900), 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_aar.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_aar.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), approx(900), 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_aar.remove()
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)


def test_local_aar_modified_and_rep_hp_limit(client, consts):
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_rep_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_hp_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_armor_attr_id)
    eve_rep_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_rep_amount_attr_id)
    eve_rep_effect_id = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair, cat_id=consts.EveEffCat.active)
    eve_hp_mod_effect_id = client.mk_eve_effect(mod_info=[eve_hp_mod])
    eve_rep_mod_effect_id = client.mk_eve_effect(mod_info=[eve_rep_mod])
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 2000, eve_armor_attr_id: 1000, eve_structure_attr_id: 500})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_mult_attr_id: 3,
            eve_rep_amount_attr_id: 500,
            eve_capacity_attr_id: 0.64,
            eve_charge_rate_attr_id: 8},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 0.01})
    eve_hp_rig = client.mk_eve_item(attrs={eve_mod_attr_id: 70}, eff_ids=[eve_hp_mod_effect_id])
    eve_rep_rig = client.mk_eve_item(attrs={eve_mod_attr_id: 70}, eff_ids=[eve_rep_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    # Verification - reps are limited from 1500 / cycle to 1000 / cycle
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(2000), 0, 0)
    assert api_stats.hp.armor == (approx(1000), approx(8000), 0)
    assert api_stats.hp.structure == (approx(500), 0, 0)
    # Action
    api_fit.add_rig(type_id=eve_hp_rig)
    # Verification - no limit now, with armor HP increased
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(2000), 0, 0)
    assert api_stats.hp.armor == (approx(1700), approx(12000), 0)
    assert api_stats.hp.structure == (approx(500), 0, 0)
    # Action
    api_fit.add_rig(type_id=eve_rep_rig)
    # Verification - limited again from 2550 / cycle to 1700 / cycle
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(2000), 0, 0)
    assert api_stats.hp.armor == (approx(1700), approx(13600), 0)
    assert api_stats.hp.structure == (approx(500), 0, 0)


def test_remote_asb_accuracy_and_charge_switch(client, consts):
    # Accuracy = cases like 2.3 / 0.1 = 22.999999999999996
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_bonus)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_rep_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ship_module_rasb, cat_id=consts.EveEffCat.target)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 3000, eve_armor_attr_id: 2000, eve_structure_attr_id: 1000})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_amount_attr_id: 300,
            eve_capacity_attr_id: 2.3,
            eve_charge_rate_attr_id: 1},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_rasb = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_rasb.change_module(add_projs=[api_ship.id])
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, approx(6900))
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_rasb.change_module(charge_type_id=None)
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_rasb.change_module(charge_type_id=eve_charge_item_id)
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, approx(6900))
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_rasb.remove()
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)


def test_remote_asb_state_switch(client, consts):
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_bonus)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_rep_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ship_module_rasb, cat_id=consts.EveEffCat.target)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 3000, eve_armor_attr_id: 2000, eve_structure_attr_id: 1000})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_amount_attr_id: 300,
            eve_capacity_attr_id: 3,
            eve_charge_rate_attr_id: 1},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_rasb = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_rasb.change_module(add_projs=[api_ship.id])
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, approx(900))
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_rasb.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_rasb.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, approx(900))
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_rasb.remove()
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)


def test_remote_asb_resist_and_rep_hp_limit(client, consts):
    # Also check projection addition/removal
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_bonus)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_resist_attr_id = client.mk_eve_attr()
    eve_mod_attr_id = client.mk_eve_attr()
    eve_resist_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_resist_attr_id)
    eve_rep_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_module_rasb,
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr_id)
    eve_resist_mod_effect_id = client.mk_eve_effect(mod_info=[eve_resist_mod])
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 1000, eve_armor_attr_id: 500, eve_structure_attr_id: 250, eve_resist_attr_id: 1})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_amount_attr_id: 1500,
            eve_capacity_attr_id: 112,
            eve_charge_rate_attr_id: 1},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 12})
    eve_resist_rig = client.mk_eve_item(attrs={eve_mod_attr_id: -70}, eff_ids=[eve_resist_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_rasb = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(1000), 0, 0)
    assert api_stats.hp.armor == (approx(500), 0, 0)
    assert api_stats.hp.structure == (approx(250), 0, 0)
    # Action
    api_rasb.change_module(add_projs=[api_ship.id])
    # Verification - reps are limited from 1500 / cycle to 1000 / cycle
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(1000), 0, approx(9000))
    assert api_stats.hp.armor == (approx(500), 0, 0)
    assert api_stats.hp.structure == (approx(250), 0, 0)
    # Action
    api_tgt_fit.add_rig(type_id=eve_resist_rig)
    # Verification - reps are reduced by RR resistance, and are no longer limited by HP:
    # 1500 (not 1000) * 9 * 0.3 = 4050
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(1000), 0, approx(4050))
    assert api_stats.hp.armor == (approx(500), 0, 0)
    assert api_stats.hp.structure == (approx(250), 0, 0)
    # Action
    api_rasb.change_module(rm_projs=[api_ship.id])
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(1000), 0, 0)
    assert api_stats.hp.armor == (approx(500), 0, 0)
    assert api_stats.hp.structure == (approx(250), 0, 0)


def test_remote_aar_accuracy_and_charge_switch(client, consts):
    # Accuracy = cases like 2.3 / 0.1 = 22.999999999999996
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_rep_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_rep_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ship_module_raar, cat_id=consts.EveEffCat.target)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 3000, eve_armor_attr_id: 2000, eve_structure_attr_id: 1000})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_mult_attr_id: 3,
            eve_rep_amount_attr_id: 100,
            eve_capacity_attr_id: 2.3,
            eve_charge_rate_attr_id: 1},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_raar = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_raar.change_module(add_projs=[api_ship.id])
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, approx(6900))
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_raar.change_module(charge_type_id=None)
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_raar.change_module(charge_type_id=eve_charge_item_id)
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, approx(6900))
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_raar.remove()
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)


def test_remote_aar_charge_rate_rounding_and_state_switch(client, consts):
    # Rounding in this case means the way lib considers not-fully-charged-cycle
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_rep_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_rep_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ship_module_raar, cat_id=consts.EveEffCat.target)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 3000, eve_armor_attr_id: 2000, eve_structure_attr_id: 1000})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_mult_attr_id: 3,
            eve_rep_amount_attr_id: 100,
            eve_capacity_attr_id: 15,
            eve_charge_rate_attr_id: 4},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_raar = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_raar.change_module(add_projs=[api_ship.id])
    # Verification - count of cycles is floored (i.e. forced to reload on partially charged cycles)
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, approx(900))
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_raar.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_raar.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, approx(900))
    assert api_stats.hp.structure == (approx(1000), 0, 0)
    # Action
    api_raar.remove()
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(3000), 0, 0)
    assert api_stats.hp.armor == (approx(2000), 0, 0)
    assert api_stats.hp.structure == (approx(1000), 0, 0)


def test_remote_aar_resist_and_rep_hp_limit(client, consts):
    # Also check projection addition/removal
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_rep_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_resist_attr_id = client.mk_eve_attr()
    eve_mod_attr_id = client.mk_eve_attr()
    eve_resist_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_resist_attr_id)
    eve_rep_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_module_raar,
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr_id)
    eve_resist_mod_effect_id = client.mk_eve_effect(mod_info=[eve_resist_mod])
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 2000, eve_armor_attr_id: 1000, eve_structure_attr_id: 500, eve_resist_attr_id: 1})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_mult_attr_id: 3,
            eve_rep_amount_attr_id: 500,
            eve_capacity_attr_id: 0.64,
            eve_charge_rate_attr_id: 8},
        eff_ids=[eve_rep_effect_id],
        defeff_id=eve_rep_effect_id)
    eve_charge_item_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 0.01})
    eve_resist_rig = client.mk_eve_item(attrs={eve_mod_attr_id: -70}, eff_ids=[eve_resist_mod_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_raar = api_src_fit.add_module(
        type_id=eve_rep_item_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_item_id)
    api_tgt_fit = api_sol.create_fit()
    api_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(2000), 0, 0)
    assert api_stats.hp.armor == (approx(1000), 0, 0)
    assert api_stats.hp.structure == (approx(500), 0, 0)
    # Action
    api_raar.change_module(add_projs=[api_ship.id])
    # Verification - reps are limited from 1500 / cycle to 1000 / cycle
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(2000), 0, 0)
    assert api_stats.hp.armor == (approx(1000), 0, approx(8000))
    assert api_stats.hp.structure == (approx(500), 0, 0)
    # Action
    api_tgt_fit.add_rig(type_id=eve_resist_rig)
    # Verification - reps are reduced by RR resistance, and are no longer limited by HP:
    # 1500 (not 1000) * 8 * 0.3 = 3600
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(2000), 0, 0)
    assert api_stats.hp.armor == (approx(1000), 0, approx(3600))
    assert api_stats.hp.structure == (approx(500), 0, 0)
    # Action
    api_raar.change_module(rm_projs=[api_ship.id])
    # Verification
    api_stats = api_tgt_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp.shield == (approx(2000), 0, 0)
    assert api_stats.hp.armor == (approx(1000), 0, 0)
    assert api_stats.hp.structure == (approx(500), 0, 0)


def test_no_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    client.mk_eve_attr(id_=consts.EveAttr.hp)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp is None


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(hp=True))
    assert api_stats.hp is None
