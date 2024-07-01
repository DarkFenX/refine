from pytest import approx


def test_add_afor_afee_proj_remove_state_proj_fit(client, consts):
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -55},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_module = client.mk_eve_item(attrs={eve_affectee_attr.id: 200})
    eve_affectee_struct = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_struct = api_affectee_fit.set_ship(type_id=eve_affectee_struct.id)
    api_affectee_module = api_affectee_fit.add_mod(type_id=eve_affectee_module.id)
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_module.change_mod(add_tgts=[api_affectee_struct.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(90)
    api_affector_module.change_mod(state=consts.ApiState.online)
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_module.change_mod(rm_tgts=[api_affectee_struct.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affectee_fit.remove()
    api_affectee_module.update(status_code=404)
    api_affector_fit.remove()


def test_add_afee_afor_proj_state_remove_afor_afee(client, consts):
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -55},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_module = client.mk_eve_item(attrs={eve_affectee_attr.id: 200})
    eve_affectee_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affectee_module = api_affectee_fit.add_mod(type_id=eve_affectee_module.id)
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.online)
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_module.change_mod(add_tgts=[api_affectee_ship.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_module.change_mod(state=consts.ApiState.active)
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(90)
    api_affector_module.remove()
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affectee_module.remove()
    api_affectee_module.update(status_code=404)
    api_affectee_fit.remove()
    api_affector_fit.remove()


def test_add_afee_afor_proj_remove_afee(client, consts):
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -55},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_module = client.mk_eve_item(attrs={eve_affectee_attr.id: 200})
    eve_affectee_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affectee_module = api_affectee_fit.add_mod(type_id=eve_affectee_module.id)
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_module.change_mod(add_tgts=[api_affectee_ship.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(90)
    api_affectee_module.remove()
    api_affectee_module.update(status_code=404)
    api_affectee_fit.remove()
    api_affector_fit.remove()
