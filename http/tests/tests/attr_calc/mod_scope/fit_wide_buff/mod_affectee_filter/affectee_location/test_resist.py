from tests import approx


def test_resisted_value_change(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_boost_attr_id = client.mk_eve_attr()
    eve_fw_effect_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_fw_effect_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.buff_everything,
        cat_id=consts.EveEffCat.active,
        resist_attr_id=eve_resist_attr_id)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_fw_effect_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_fw_effect_effect_id], defeff_id=eve_fw_effect_effect_id)
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr_id,
        affectee_attr_id=eve_resist_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_boost_attr_id: -25}, eff_ids=[eve_rig_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_resist_attr_id: 0.4})
    eve_module_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id)
    api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(19.5)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(16.5)
    api_rig.remove()
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(19.5)
