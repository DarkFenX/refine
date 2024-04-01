from pytest import approx


def test_ships(client, consts):
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
    eve_sw_effect = api_ss.add_sw_effect(type_id=eve_sw_effect.id)
    api_fit1 = api_ss.create_fit()
    api_fit2 = api_ss.create_fit()
    api_ship1 = api_fit1.set_ship(type_id=eve_ship.id)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship.id)
    assert api_ship1.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    assert api_ship2.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    eve_sw_effect.remove()
    assert api_ship1.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    assert api_ship2.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_drones(client, consts):
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
    eve_sw_effect = api_ss.add_sw_effect(type_id=eve_sw_effect.id)
    api_fit1 = api_ss.create_fit()
    api_fit2 = api_ss.create_fit()
    api_drone1 = api_fit1.add_drone(type_id=eve_ship.id)
    api_drone2 = api_fit1.add_drone(type_id=eve_ship.id)
    api_drone3 = api_fit2.add_drone(type_id=eve_ship.id)
    assert api_drone1.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    assert api_drone2.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    assert api_drone3.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    eve_sw_effect.remove()
    assert api_drone1.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    assert api_drone2.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    assert api_drone3.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
