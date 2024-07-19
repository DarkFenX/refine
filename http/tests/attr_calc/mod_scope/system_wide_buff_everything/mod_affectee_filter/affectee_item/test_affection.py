from tests import approx


def test_affected_root_ship_multiple(client, consts):
    # Make sure ships are affected by system-wide buffs
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_ship1 = api_fit1.set_ship(type_id=eve_ship.id)
    api_sol.add_sw_effect(type_id=eve_sw_effect.id)
    api_fit2 = api_sol.create_fit()
    api_ship2 = api_fit2.set_ship(type_id=eve_ship.id)
    assert api_ship1.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    assert api_ship2.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)


def test_affected_child(client, consts):
    # Make sure drones are affected by system-wide buffs
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_drone1 = api_fit1.add_drone(type_id=eve_drone.id)
    api_sol.add_sw_effect(type_id=eve_sw_effect.id)
    api_fit2 = api_sol.create_fit()
    api_drone2 = api_fit1.add_drone(type_id=eve_drone.id)
    api_drone3 = api_fit2.add_drone(type_id=eve_drone.id)
    assert api_drone1.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    assert api_drone2.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    assert api_drone3.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)


def test_unaffected_root_struct(client, consts):
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_struct = client.mk_eve_struct(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct.id)
    api_sol.add_sw_effect(type_id=eve_sw_effect.id)
    assert api_struct.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)


def test_unaffected_root_char(client, consts):
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_char = client.mk_eve_item(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect.id)
    api_fit = api_sol.create_fit()
    api_char = api_fit.set_char(type_id=eve_char.id)
    assert api_char.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)


def test_unaffected_child_unbuffable(client, consts):
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_affectee_item = client.mk_eve_ship(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect.id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_affectee_item.id)
    api_rig = api_fit.add_rig(type_id=eve_affectee_item.id)
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(37.5)
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)


def test_unaffected_other_sw_effect(client, consts):
    # This is undefined behavior, and it's possible that it works differently in EVE, but in reefast
    # one system-wide buff cannot affect another.
    eve_buff_type_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr = client.mk_eve_attr()
    eve_buff = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr.id)])
    eve_effect = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_sw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect.id)
    api_affectee_item = api_sol.add_sw_effect(type_id=eve_affectee_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(7.5)
