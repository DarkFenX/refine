from fw import approx


def test_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_middle_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_affector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_middle_attr_id)
    eve_affector_effect_id = client.mk_eve_effect(mod_info=[eve_affector_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 50}, eff_ids=[eve_affector_effect_id])
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_srq_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, skill_id=eve_skill_id)])
    eve_middle_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_middle_item_id = client.mk_eve_item(
        attrs={eve_middle_attr_id: -55},
        eff_ids=[eve_middle_effect_id],
        defeff_id=eve_middle_effect_id)
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.set_ship(type_id=eve_ship_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_affectee_item = api_fit2.add_rig(type_id=eve_affectee_item_id)
    api_middle_item = api_fit1.add_module(type_id=eve_middle_item_id, state=consts.ApiModuleState.active)
    api_middle_item.change_module(add_projs=[api_ship.id])
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    api_affector_item = api_fit1.add_rig(type_id=eve_affector_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(35)
    api_affector_item.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(90)
