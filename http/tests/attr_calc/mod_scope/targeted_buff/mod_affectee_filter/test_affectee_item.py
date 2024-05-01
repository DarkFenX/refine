from pytest import approx


def test_affected_state_change_root_ship(client, consts):
    # Make sure ship is affected by targeted buffs
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_tgt_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_src_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.online)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(200)
    api_module.change_mod(state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(90)
    api_module.change_mod(state=consts.ApiState.online)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(200)


def test_affected_state_change_root_struct(client, consts):
    # Make sure structure is affected by targeted buffs
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_tgt_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_src_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_struct = client.mk_eve_item(attrs={eve_tgt_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_struct = api_fit2.set_struct(type_id=eve_struct.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.online)
    api_module.change_mod(add_tgts=[api_struct.id])
    assert api_struct.update().attrs[eve_tgt_attr.id].dogma == approx(200)
    api_module.change_mod(state=consts.ApiState.active)
    assert api_struct.update().attrs[eve_tgt_attr.id].dogma == approx(90)
    api_module.change_mod(state=consts.ApiState.online)
    assert api_struct.update().attrs[eve_tgt_attr.id].dogma == approx(200)


def test_affected_state_change_child(client, consts):
    # Make sure child owner-modifiable items are affected by targeted buffs
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_tgt_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_src_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_tgt_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.online)
    api_module.change_mod(add_tgts=[api_drone.id])
    assert api_drone.update().attrs[eve_tgt_attr.id].dogma == approx(200)
    api_module.change_mod(state=consts.ApiState.active)
    assert api_drone.update().attrs[eve_tgt_attr.id].dogma == approx(90)
    api_module.change_mod(state=consts.ApiState.online)
    assert api_drone.update().attrs[eve_tgt_attr.id].dogma == approx(200)


def test_affected_targeting_root_ship(client, consts):
    # Make sure ship is affected by targeted buffs
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_tgt_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_src_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(200)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(90)
    api_module.change_mod(rm_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(200)


def test_affected_targeting_root_struct(client, consts):
    # Make sure structure is affected by targeted buffs
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_tgt_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_src_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_struct = client.mk_eve_item(attrs={eve_tgt_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_struct = api_fit2.set_struct(type_id=eve_struct.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    assert api_struct.update().attrs[eve_tgt_attr.id].dogma == approx(200)
    api_module.change_mod(add_tgts=[api_struct.id])
    assert api_struct.update().attrs[eve_tgt_attr.id].dogma == approx(90)
    api_module.change_mod(rm_tgts=[api_struct.id])
    assert api_struct.update().attrs[eve_tgt_attr.id].dogma == approx(200)


def test_affected_targeting_child(client, consts):
    # Make sure child owner-modifiable items are affected by targeted buffs
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_tgt_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_src_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_tgt_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    assert api_drone.update().attrs[eve_tgt_attr.id].dogma == approx(200)
    api_module.change_mod(add_tgts=[api_drone.id])
    assert api_drone.update().attrs[eve_tgt_attr.id].dogma == approx(90)
    api_module.change_mod(rm_tgts=[api_drone.id])
    assert api_drone.update().attrs[eve_tgt_attr.id].dogma == approx(200)


def test_affected_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_src_attr = client.mk_eve_attr()
    eve_mid_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_tgt_attr = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_mid_attr.id)
    eve_src_effect = client.mk_eve_effect(mod_info=[eve_src_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 50}, eff_ids=[eve_src_effect.id])
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_mid_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_mid_item = client.mk_eve_item(
        attrs={eve_mid_attr.id: -55},
        eff_ids=[eve_mid_effect.id],
        defeff_id=eve_mid_effect.id)
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 200})
    eve_ship_item = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.set_ship(type_id=eve_ship_item.id)
    api_tgt_item = api_fit2.set_ship(type_id=eve_tgt_item.id)
    api_mid_item = api_fit1.add_mod(type_id=eve_mid_item.id, state=consts.ApiState.active)
    api_mid_item.change_mod(add_tgts=[api_tgt_item.id])
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(90)
    api_src_item = api_fit1.add_rig(type_id=eve_src_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(35)
    api_src_item.remove()
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(90)


def test_unaffected_non_buff_modifiable_root(client, consts):
    # Character can't be affected via ship since it's not buff-modifiable
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_tgt_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_src_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_char = client.mk_eve_item(attrs={eve_tgt_attr.id: 200})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_char = api_fit2.set_char(type_id=eve_char.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_char.update().attrs[eve_tgt_attr.id].dogma == approx(200)


def test_unaffected_child_via_root(client, consts):
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_tgt_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_src_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_tgt_attr.id: 200})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.set_ship(type_id=eve_drone.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_drone.update().attrs[eve_tgt_attr.id].dogma == approx(200)


def test_unaffected_root_via_child(client, consts):
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_tgt_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_src_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 200})
    eve_drone = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.set_ship(type_id=eve_drone.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_tgts=[api_drone.id])
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(200)


def test_unaffected_root_other_fit(client, consts):
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_tgt_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_src_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit3 = api_sol.create_fit()
    api_ship2 = api_fit2.set_ship(type_id=eve_ship.id)
    api_ship3 = api_fit3.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_tgts=[api_ship2.id])
    assert api_ship3.update().attrs[eve_tgt_attr.id].dogma == approx(200)


def test_unaffected_child_other_fit(client, consts):
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_tgt_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_src_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_tgt_attr.id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit3 = api_sol.create_fit()
    api_drone2 = api_fit2.add_drone(type_id=eve_drone.id)
    api_drone3 = api_fit3.add_drone(type_id=eve_drone.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_tgts=[api_drone2.id])
    assert api_drone3.update().attrs[eve_tgt_attr.id].dogma == approx(200)


def test_replace_target(client, consts):
    eve_src_attr = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_tgt_attr = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module = client.mk_eve_item(attrs={eve_src_attr.id: -55}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship1 = client.mk_eve_item(attrs={eve_tgt_attr.id: 200})
    eve_ship2 = client.mk_eve_item(attrs={eve_tgt_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship1 = api_fit2.set_ship(type_id=eve_ship1.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_tgts=[api_ship1.id])
    assert api_ship1.update().attrs[eve_tgt_attr.id].dogma == approx(90)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship2.id)
    assert api_ship2.update().attrs[eve_tgt_attr.id].dogma == approx(100)
    api_module.change_mod(add_tgts=[api_ship2.id])
    assert api_ship2.update().attrs[eve_tgt_attr.id].dogma == approx(45)
