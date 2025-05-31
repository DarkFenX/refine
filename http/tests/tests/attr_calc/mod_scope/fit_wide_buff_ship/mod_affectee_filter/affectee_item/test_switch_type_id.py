from tests import approx, check_no_field


def setup_root_test(*, client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_titan_effect_generator,
        cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 30},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_root_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_root_struct_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 200})
    eve_root_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    return eve_affectee_attr_id, eve_root_ship_id, eve_root_struct_id, eve_root_not_loaded_id, api_fit, api_fw_effect


def test_root_affected_to_unaffected_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _,
     api_fit,
     api_fw_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(260)
    # Action
    api_root.change_ship(type_id=eve_root_struct_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    # Action
    api_fw_effect.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_root_affected_to_not_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     eve_root_not_loaded_id,
     api_fit,
     api_fw_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(260)
    # Action
    api_root.change_ship(type_id=eve_root_not_loaded_id)
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action
    api_fw_effect.remove()
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018


def test_root_unaffected_to_affected_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _,
     api_fit,
     api_fw_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_struct_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(260)
    # Action
    api_fw_effect.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_root_not_loaded_to_affected_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     eve_root_not_loaded_id,
     api_fit,
     api_fw_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_not_loaded_id)
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(260)
    # Action
    api_fw_effect.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(200)
