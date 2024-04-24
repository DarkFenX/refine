from pytest import approx


def test_affected_parent_ship(client, consts):
    # Make sure ship is affected by fit-wide buffs
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
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect.id, state=False)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_fw_effect.change_fw_effect(state=True)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_fw_effect.change_fw_effect(state=False)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_affected_parent_struct(client, consts):
    # Make sure structure is affected by fit-wide buffs
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
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_struct = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_struct(type_id=eve_struct.id)
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect.id, state=False)
    assert api_struct.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_fw_effect.change_fw_effect(state=True)
    assert api_struct.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_fw_effect.change_fw_effect(state=False)
    assert api_struct.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_affected_child(client, consts):
    # Make sure drones are affected by fit-wide buffs
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
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone1 = api_fit.add_drone(type_id=eve_drone.id)
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect.id, state=False)
    api_drone2 = api_fit.add_drone(type_id=eve_drone.id)
    assert api_drone1.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    assert api_drone2.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    api_fw_effect.change_fw_effect(state=True)
    assert api_drone1.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    assert api_drone2.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_fw_effect.change_fw_effect(state=False)
    assert api_drone1.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    assert api_drone2.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


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
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_char = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_fw_effect.id)
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
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_fw_effect.id)
    api_ship = api_fit.set_ship(type_id=eve_tgt_item.id)
    api_rig = api_fit.add_rig(type_id=eve_tgt_item.id)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    assert api_rig.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_other_fw_effect(client, consts):
    # This is undefined behavior, and it's possible that it works differently in EVE, but in reefast
    # one fit-wide buff cannot affect another.
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
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_fw_effect.id)
    api_tgt_item = api_fit.add_fw_effect(type_id=eve_tgt_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_other_fit_parent(client, consts):
    # Check that fit-wide modifications are not carried over to another fit
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
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.add_fw_effect(type_id=eve_fw_effect.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    assert api_ship.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_unaffected_other_fit_child(client, consts):
    # Check that fit-wide modifications are not carried over to another fit
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
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_drone1 = api_fit1.add_drone(type_id=eve_drone.id)
    api_fit2.add_fw_effect(type_id=eve_fw_effect.id)
    api_drone2 = api_fit1.add_drone(type_id=eve_drone.id)
    assert api_drone1.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)
    assert api_drone2.update().attrs[eve_tgt_attr.id].dogma == approx(7.5)


def test_replace_parent(client, consts):
    # Make sure ship is affected by fit-wide buffs
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
    eve_fw_effect = client.mk_eve_item(
        attrs={eve_buff_type_attr.id: eve_buff.id, eve_buff_val_attr.id: 5},
        eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship1 = client.mk_eve_item(attrs={eve_tgt_attr.id: 7.5})
    eve_ship2 = client.mk_eve_item(attrs={eve_tgt_attr.id: 15})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship1 = api_fit.set_ship(type_id=eve_ship1.id)
    api_fit.add_fw_effect(type_id=eve_fw_effect.id)
    assert api_ship1.update().attrs[eve_tgt_attr.id].dogma == approx(37.5)
    api_ship2 = api_fit.set_ship(type_id=eve_ship2.id)
    assert api_ship2.update().attrs[eve_tgt_attr.id].dogma == approx(75)
