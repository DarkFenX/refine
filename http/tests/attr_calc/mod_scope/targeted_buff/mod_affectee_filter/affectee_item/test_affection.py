from pytest import approx


def test_affected_root_ship(client, consts):
    # Make sure ship is affected by targeted buffs
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
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(90)


def test_affected_root_struct(client, consts):
    # Make sure structure is affected by targeted buffs
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_affector_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_struct = client.mk_eve_struct(attrs={eve_affectee_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_struct = api_fit2.set_ship(type_id=eve_struct.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_struct.id])
    assert api_struct.update().attrs[eve_affectee_attr.id].dogma == approx(90)


def test_affected_child(client, consts):
    # Make sure child owner-modifiable items are affected by targeted buffs
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_affector_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_affectee_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_drone.id])
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(90)


def test_unaffected_non_buff_modifiable_root(client, consts):
    # Character can't be affected via ship since it's not buff-modifiable
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_affector_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_char = client.mk_eve_item(attrs={eve_affectee_attr.id: 200})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_char = api_fit2.set_char(type_id=eve_char.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_char.update().attrs[eve_affectee_attr.id].dogma == approx(200)


def test_unaffected_child_via_root(client, consts):
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_affector_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_affectee_attr.id: 200})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_drone.update().attrs[eve_affectee_attr.id].dogma == approx(200)


def test_unaffected_root_via_child(client, consts):
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
    eve_drone = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_drone.id])
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(200)


def test_unaffected_root_other_fit(client, consts):
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
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit3 = api_sol.create_fit()
    api_ship2 = api_fit2.set_ship(type_id=eve_ship.id)
    api_ship3 = api_fit3.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_ship2.id])
    assert api_ship3.update().attrs[eve_affectee_attr.id].dogma == approx(200)


def test_unaffected_child_other_fit(client, consts):
    eve_affector_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_affector_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_affectee_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit3 = api_sol.create_fit()
    api_drone2 = api_fit2.add_drone(type_id=eve_drone.id)
    api_drone3 = api_fit3.add_drone(type_id=eve_drone.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_projs=[api_drone2.id])
    assert api_drone3.update().attrs[eve_affectee_attr.id].dogma == approx(200)
