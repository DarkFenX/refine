from tests import approx


def test_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_affector_attr = client.mk_eve_attr()
    eve_middle_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    eve_affector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_middle_attr.id)
    eve_affector_effect = client.mk_eve_effect(mod_info=[eve_affector_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 50}, eff_ids=[eve_affector_effect.id])
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_middle_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_middle_item = client.mk_eve_item(
        attrs={eve_middle_attr.id: -55},
        eff_ids=[eve_middle_effect.id],
        defeff_id=eve_middle_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 200})
    eve_affector_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_affector_ship.id)
    api_affectee_item = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_middle_item = api_affector_fit.add_mod(type_id=eve_middle_item.id, state=consts.ApiState.active)
    api_middle_item.change_mod(add_projs=[api_affectee_item.id])
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(90)
    api_affector_item = api_affector_fit.add_rig(type_id=eve_affector_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(35)
    api_affector_item.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(90)


def test_replace_proj_ship_to_struct(client, consts):
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_affector_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 200})
    eve_struct = client.mk_eve_struct(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_module = api_affector_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(90)
    api_struct = api_affectee_fit.set_ship(type_id=eve_struct.id)
    assert api_struct.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_module.change_mod(add_projs=[api_struct.id])
    assert api_struct.update().attrs[eve_affectee_attr.id].dogma == approx(45)
