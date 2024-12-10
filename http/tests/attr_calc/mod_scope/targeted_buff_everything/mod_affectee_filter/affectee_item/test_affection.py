from tests import approx


def test_affected_root_ship(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(attrs={eve_affector_attr_id: -55}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(90)


def test_affected_child(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(attrs={eve_affector_attr_id: -55}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_drone.id])
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(90)


def test_unaffected_root_struct(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(attrs={eve_affector_attr_id: -55}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_struct_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_struct = api_fit2.set_ship(type_id=eve_struct_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_struct.id])
    assert api_struct.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_unaffected_root_char(client, consts):
    # Character can't be affected via ship since it's not buff-modifiable
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(attrs={eve_affector_attr_id: -55}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_char_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_char = api_fit2.set_char(type_id=eve_char_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_char.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_unaffected_child_via_root(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(attrs={eve_affector_attr_id: -55}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_unaffected_root_via_child(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(attrs={eve_affector_attr_id: -55}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_drone_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_drone.id])
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_unaffected_root_other_fit(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(attrs={eve_affector_attr_id: -55}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit3 = api_sol.create_fit()
    api_ship2 = api_fit2.set_ship(type_id=eve_ship_id)
    api_ship3 = api_fit3.set_ship(type_id=eve_ship_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_ship2.id])
    assert api_ship3.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_unaffected_child_other_fit(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(attrs={eve_affector_attr_id: -55}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit3 = api_sol.create_fit()
    api_drone2 = api_fit2.add_drone(type_id=eve_drone_id)
    api_drone3 = api_fit3.add_drone(type_id=eve_drone_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_drone2.id])
    assert api_drone3.update().attrs[eve_affectee_attr_id].dogma == approx(200)
