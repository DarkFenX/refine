from fw import approx


def test_proj_onlist_to_onlist(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_ship1 = api_affectee_fit.set_ship(type_id=eve_ship1_id)
    api_module = api_affector_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module.change_module(add_projs=[api_ship1.id])
    assert api_ship1.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    api_ship2 = api_affectee_fit.set_ship(type_id=eve_ship2_id)
    assert api_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_module.change_module(add_projs=[api_ship2.id])
    assert api_ship2.update().attrs[eve_affectee_attr_id].dogma == approx(45)


def test_proj_onlist_to_offlist(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_struct_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_module = api_affector_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    api_struct = api_affectee_fit.set_ship(type_id=eve_struct_id)
    assert api_struct.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_module.change_module(add_projs=[api_struct.id])
    assert api_struct.update().attrs[eve_affectee_attr_id].dogma == approx(100)
