from pytest import approx


def test_resisted_value_change(client, consts):
    eve_skill = client.mk_eve_item()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_resist_attr = client.mk_eve_attr()
    eve_boost_attr = client.mk_eve_attr()
    eve_fw_effect_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, skill_id=eve_skill.id)])
    eve_fw_effect_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active,
        resist_attr_id=eve_resist_attr.id)
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_fw_effect_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_fw_effect_effect.id], defeff_id=eve_fw_effect_effect.id)
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr.id,
        affectee_attr_id=eve_resist_attr.id)
    eve_rig_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_rig = client.mk_eve_item(attrs={eve_boost_attr.id: -25}, eff_ids=[eve_rig_effect.id])
    eve_struct = client.mk_eve_struct(attrs={eve_resist_attr.id: 0.4})
    eve_module = client.mk_eve_item(attrs={eve_affectee_attr.id: 7.5}, srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct.id)
    api_module = api_fit.add_mod(type_id=eve_module.id)
    api_fit.add_fw_effect(type_id=eve_fw_effect.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(19.5)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(16.5)
    api_rig.remove()
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(19.5)
