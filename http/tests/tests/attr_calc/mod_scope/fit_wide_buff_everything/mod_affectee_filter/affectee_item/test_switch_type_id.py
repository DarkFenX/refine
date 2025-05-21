from tests import approx, check_no_field


def test_root_to_unaffected(client, consts):
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
    eve_root1_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    eve_root2_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_root = api_fit.set_ship(type_id=eve_root1_id)
    api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_root.change_ship(type_id=eve_root2_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action
    api_root.change_ship(type_id=eve_root1_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)


def test_affectee_root_ship(client, consts):
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
    eve_ship1_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 5})
    eve_ship3_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(25)
    # Action
    api_ship.change_ship(type_id=eve_ship3_id)
    # Verification
    api_ship.update()
    with check_no_field():
        api_ship.attrs  # noqa: B018
    # Action
    api_ship.change_ship(type_id=eve_ship1_id)
    # Verification
    assert api_ship.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)


def test_affectee_child_drone(client, consts):
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
    eve_drone1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 7.5})
    eve_drone2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5})
    eve_drone3_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone1_id)
    api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_drone.change_drone(type_id=eve_drone2_id)
    # Verification
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(25)
    # Action
    api_drone.change_drone(type_id=eve_drone3_id)
    # Verification
    api_drone.update()
    with check_no_field():
        api_drone.attrs  # noqa: B018
    # Action
    api_drone.change_drone(type_id=eve_drone1_id)
    # Verification
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
