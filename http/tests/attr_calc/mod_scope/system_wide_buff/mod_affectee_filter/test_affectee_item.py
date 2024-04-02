from pytest import approx


def test_affected_ships(client, consts):
    # Make sure ship is affected by system-wide buffs
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit1 = api_ss.create_fit()
    api_ship1 = api_fit1.set_ship(type_id=eve_ship.id)
    eve_sw_effect = api_ss.add_sw_effect(type_id=eve_sw_effect.id)
    api_fit2 = api_ss.create_fit()
    api_ship2 = api_fit2.set_ship(type_id=eve_ship.id)
    assert api_ship1.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    assert api_ship2.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    eve_sw_effect.remove()
    assert api_ship1.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    assert api_ship2.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_affected_drones(client, consts):
    # Make sure drones are affected by system-wide buffs
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_fit1 = api_ss.create_fit()
    api_drone1 = api_fit1.add_drone(type_id=eve_ship.id)
    eve_sw_effect = api_ss.add_sw_effect(type_id=eve_sw_effect.id)
    api_fit2 = api_ss.create_fit()
    api_drone2 = api_fit1.add_drone(type_id=eve_ship.id)
    api_drone3 = api_fit2.add_drone(type_id=eve_ship.id)
    assert api_drone1.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    assert api_drone2.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    assert api_drone3.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    eve_sw_effect.remove()
    assert api_drone1.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    assert api_drone2.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    assert api_drone3.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_non_buff_modifiable_parent(client, consts):
    # Check that top-level entities which are not supposed to receive modification (e.g. characters)
    # do not receive it
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_char = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_ss.add_sw_effect(type_id=eve_sw_effect.id)
    api_fit = api_ss.create_fit()
    api_char = api_fit.set_char(type_id=eve_char.id)
    assert api_char.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_non_buff_modifiable_child(client, consts):
    # Check that non-top-level entities do not receive modification, even if they are part of domain
    # whose owner is getting it
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_tgt_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_tgt_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_ss = client.create_ss()
    api_ss.add_sw_effect(type_id=eve_sw_effect.id)
    api_fit = api_ss.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_tgt_item.id)
    api_rig = api_fit.add_rig(type_id=eve_tgt_item.id)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
