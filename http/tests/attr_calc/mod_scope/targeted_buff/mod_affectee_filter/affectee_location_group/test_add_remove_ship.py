from pytest import approx


def test_add_afor_afee_proj_remove_state_proj_fit(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_titan_effect_generator, cat_id=consts.EveEffCat.active)
    eve_affector_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 30},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_module = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 200})
    eve_affectee_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affectee_module = api_affectee_fit.add_mod(type_id=eve_affectee_module.id)
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(260)
    api_affector_module.change_mod(state=consts.ApiState.online)
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_module.change_mod(rm_projs=[api_affectee_ship.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affectee_fit.remove()
    api_affectee_module.update(status_code=404)
    api_affector_fit.remove()


def test_add_afee_afor_proj_state_remove_afor_afee(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_titan_effect_generator, cat_id=consts.EveEffCat.active)
    eve_affector_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 30},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_module = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 200})
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
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_module.change_mod(state=consts.ApiState.active)
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(260)
    api_affector_module.remove()
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affectee_module.remove()
    api_affectee_module.update(status_code=404)
    api_affectee_fit.remove()
    api_affector_fit.remove()


def test_add_afee_afor_proj_remove_afee(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id, group_id=eve_grp.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.mod_titan_effect_generator, cat_id=consts.EveEffCat.active)
    eve_affector_module = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 30},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_module = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 200})
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
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    assert api_affectee_module.update().attrs[eve_affectee_attr.id].dogma == approx(260)
    api_affectee_module.remove()
    api_affectee_module.update(status_code=404)
    api_affectee_fit.remove()
    api_affector_fit.remove()
