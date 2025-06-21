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


def test_local_aar_charge(client, consts):
    eve_shield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_armor_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_structure_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_rep_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_rep_effect_id = client.mk_eve_effect(id_=consts.EveEffect.fueled_armor_repair)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_shield_attr_id: 3000, eve_armor_attr_id: 2000, eve_structure_attr_id: 1000})
    eve_rep_item_id = client.mk_eve_item(
        attrs={
            eve_rep_mult_attr_id: 3,
            eve_rep_amount_attr_id: 100,
            eve_capacity_attr_id: 2.3,
            eve_charge_rate_attr_id: 1},
        eff_ids=[eve_rep_effect_id])
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
