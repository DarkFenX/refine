from tests import approx


def test_root_add_fw_item_remove_fw_item(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_fw_effect.remove()
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_ship.remove()
    api_ship.update(status_code=404)
    api_fit.remove()


def test_root_add_item_fw_remove_state_item_fw(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_fw_effect.change_fw_effect(state=False)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_ship.remove()
    api_ship.update(status_code=404)
    api_fw_effect.remove()
    api_fit.remove()


def test_root_add_fw_item_state_remove_fit(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id, state=False)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_fw_effect.change_fw_effect(state=True)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_fit.remove()
    api_ship.update(status_code=404)


def test_child_add_fw_item_remove_fw_item(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_fw_effect.remove()
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_drone.remove()
    api_drone.update(status_code=404)
    api_fit.remove()


def test_child_add_item_fw_remove_state_item_fw(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_fw_effect.change_fw_effect(state=False)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_drone.remove()
    api_drone.update(status_code=404)
    api_fw_effect.remove()
    api_fit.remove()


def test_child_add_fw_item_state_remove_fit(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.weather_darkness,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id, state=False)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_fw_effect.change_fw_effect(state=True)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_fit.remove()
    api_drone.update(status_code=404)
