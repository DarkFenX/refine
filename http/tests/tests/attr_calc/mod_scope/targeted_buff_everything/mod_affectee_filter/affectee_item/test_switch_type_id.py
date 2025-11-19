from tests import approx, check_no_field


def setup_root_test(*, client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_root_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 200})
    eve_root_struct_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 100})
    eve_root_unknown_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 50})
    eve_root_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    return (
        eve_affectee_attr_id,
        eve_root_ship_id,
        eve_root_struct_id,
        eve_root_unknown_id,
        eve_root_not_loaded_id,
        api_fit2,
        api_module)


def test_root_ship_to_struct_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _,
     _,
     api_fit2,
     api_module) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_ship_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    # Action
    api_root.change_ship(type_id=eve_root_struct_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_module.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_root_ship_to_unknown_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     eve_root_unknown_id,
     _,
     api_fit2,
     api_module) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_ship_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    # Action
    api_root.change_ship(type_id=eve_root_unknown_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(50)
    # Action
    api_module.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(50)


def test_root_ship_to_not_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     _,
     eve_root_not_loaded_id,
     api_fit2,
     api_module) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_ship_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(90)
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


def test_root_struct_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _,
     _,
     api_fit2,
     api_module) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_struct_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    # Action
    api_module.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_root_unknown_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     eve_root_unknown_id,
     _,
     api_fit2,
     api_module) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_unknown_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(50)
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    # Action
    api_module.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_root_not_loaded_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     _,
     eve_root_not_loaded_id,
     api_fit2,
     api_module) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_not_loaded_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    # Action
    api_module.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_affectee_child_drone(client, consts):
    eve_affector_attr_id = client.mk_eve_attr(id_=consts.EveAttr.speed_factor)
    eve_affectee_attr_id = client.mk_eve_attr()
    client.mk_eve_buff(
        id_=consts.EveBuff.stasis_webification_burst,
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_web, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -55},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_drone1_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 200})
    eve_drone2_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 100})
    eve_drone3_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_drone = api_fit2.add_drone(type_id=eve_drone1_id)
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module.change_module(add_projs=[api_drone.id])
    # Verification
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(90)
    # Action
    api_drone.change_drone(type_id=eve_drone2_id)
    # Verification
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(45)
    # Action
    api_drone.change_drone(type_id=eve_drone3_id)
    # Verification
    api_drone.update()
    with check_no_field():
        api_drone.attrs  # noqa: B018
    # Action
    api_drone.change_drone(type_id=eve_drone1_id)
    # Verification
    assert api_drone.update().attrs[eve_affectee_attr_id].dogma == approx(90)
