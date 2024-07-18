from tests import approx


def test_resisted_value_change_root(client, consts):
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    eve_resist_attr = client.mk_eve_attr()
    eve_boost_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_aoe_web,
        cat_id=consts.EveEffCat.active,
        resist_attr_id=eve_resist_attr.id)
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -55},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr.id,
        affectee_attr_id=eve_resist_attr.id)
    eve_rig_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_affectee_rig = client.mk_eve_item(attrs={eve_boost_attr.id: -25}, eff_ids=[eve_rig_effect.id])
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 200, eve_resist_attr.id: 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(156)
    api_affectee_rig = api_affectee_fit.add_rig(type_id=eve_affectee_rig.id)
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(167)
    api_affectee_rig.remove()
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(156)


def test_resisted_value_change_child(client, consts):
    eve_skill = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    eve_resist_attr = client.mk_eve_attr()
    eve_boost_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_aoe_web,
        cat_id=consts.EveEffCat.active,
        resist_attr_id=eve_resist_attr.id)
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -55},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.char,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr.id,
        affectee_attr_id=eve_resist_attr.id)
    eve_rig_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_affectee_rig = client.mk_eve_item(attrs={eve_boost_attr.id: -25}, eff_ids=[eve_rig_effect.id])
    eve_affectee_drone = client.mk_eve_item(
        attrs={eve_affectee_attr.id: 200, eve_resist_attr.id: 0.4},
        srqs={eve_skill.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_drone.id])
    assert api_affectee_drone.update().attrs[eve_affectee_attr.id].dogma == approx(156)
    api_affectee_rig = api_affectee_fit.add_rig(type_id=eve_affectee_rig.id)
    assert api_affectee_drone.update().attrs[eve_affectee_attr.id].dogma == approx(167)
    api_affectee_rig.remove()
    assert api_affectee_drone.update().attrs[eve_affectee_attr.id].dogma == approx(156)
