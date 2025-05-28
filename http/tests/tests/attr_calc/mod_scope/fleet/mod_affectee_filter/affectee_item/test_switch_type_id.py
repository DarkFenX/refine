from tests import approx, check_no_field


def setup_root_test(*, client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_root_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    eve_root_struct_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 7.5})
    eve_root_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    return eve_affectee_attr_id, eve_module_id, eve_root_ship_id, eve_root_struct_id, eve_root_not_loaded_id


def test_root_affected_to_unaffected_remove_self(client, consts):
    (eve_affectee_attr_id,
     eve_module_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _) = setup_root_test(client=client, consts=consts)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_root = api_fit.set_ship(type_id=eve_root_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_root.change_ship(type_id=eve_root_struct_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action
    api_module.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_root_affected_to_unaffected_remove_fleeted(client, consts):
    (eve_affectee_attr_id,
     eve_module_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _) = setup_root_test(client=client, consts=consts)
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_root = api_fit2.set_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_root.change_ship(type_id=eve_root_struct_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action
    api_module.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_root_affected_to_not_loaded_remove_self(client, consts):
    (eve_affectee_attr_id,
     eve_module_id,
     eve_root_ship_id,
     _,
     eve_root_not_loaded_id) = setup_root_test(client=client, consts=consts)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_root = api_fit.set_ship(type_id=eve_root_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_root.change_ship(type_id=eve_root_not_loaded_id)
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action
    api_module.remove()
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018


def test_root_affected_to_not_loaded_remove_fleeted(client, consts):
    (eve_affectee_attr_id,
     eve_module_id,
     eve_root_ship_id,
     _,
     eve_root_not_loaded_id) = setup_root_test(client=client, consts=consts)
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_root = api_fit2.set_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_root.change_ship(type_id=eve_root_not_loaded_id)
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action
    api_module.remove()
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018


def test_root_unaffected_to_affected_remove_self(client, consts):
    (eve_affectee_attr_id,
     eve_module_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _) = setup_root_test(client=client, consts=consts)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_root = api_fit.set_ship(type_id=eve_root_struct_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_module.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_root_unaffected_to_affected_remove_fleeted(client, consts):
    (eve_affectee_attr_id,
     eve_module_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _) = setup_root_test(client=client, consts=consts)
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_root = api_fit2.set_ship(type_id=eve_root_struct_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_module.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_root_not_loaded_to_affected_remove_self(client, consts):
    (eve_affectee_attr_id,
     eve_module_id,
     eve_root_ship_id,
     _,
     eve_root_not_loaded_id) = setup_root_test(client=client, consts=consts)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_root = api_fit.set_ship(type_id=eve_root_not_loaded_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_module.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_root_not_loaded_to_affected_remove_fleeted(client, consts):
    (eve_affectee_attr_id,
     eve_module_id,
     eve_root_ship_id,
     _,
     eve_root_not_loaded_id) = setup_root_test(client=client, consts=consts)
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_root = api_fit2.set_ship(type_id=eve_root_not_loaded_id)
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_module.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)


def test_affectee_self(client, consts):
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 5})
    eve_ship3_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
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


def test_affectee_fleeted(client, consts):
    # Make sure ship is affected by fleet buffs
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_bonus_warfare_link_armor,
        cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 5},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 5})
    eve_ship3_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship1_id)
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
