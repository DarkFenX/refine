from fw import approx


def test_root_add_pe_item_proj_remove_state_proj_fit(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_proj_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_proj_effect.change_proj_effect(state=False)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(rm_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_fit.remove()
    api_ship.update(status_code=404)
    api_proj_effect.remove()


def test_root_add_item_pe_proj_state_remove_pe_item(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_proj_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id, state=False)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(state=True)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_proj_effect.remove()
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_ship.remove()
    api_ship.update(status_code=404)
    api_fit.remove()


def test_root_add_item_pe_proj_remove_item(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_proj_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_ship.remove()
    api_ship.update(status_code=404)
    api_proj_effect.remove()
    api_fit.remove()


def test_child_add_pe_item_proj_remove_state_proj_fit(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_proj_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(add_projs=[api_drone.id])
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_proj_effect.change_proj_effect(state=False)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(rm_projs=[api_drone.id])
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_fit.remove()
    api_drone.update(status_code=404)
    api_proj_effect.remove()


def test_child_add_item_pe_proj_state_remove_pe_item(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_proj_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id, state=False)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(add_projs=[api_drone.id])
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(state=True)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_proj_effect.remove()
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_drone.remove()
    api_drone.update(status_code=404)
    api_fit.remove()


def test_child_add_item_pe_proj_remove_item(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_proj_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(add_projs=[api_drone.id])
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    api_drone.remove()
    api_drone.update(status_code=404)
    api_proj_effect.remove()
    api_fit.remove()
