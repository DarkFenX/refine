from fw import approx, check_no_field


def setup_test(*, client, consts):
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_loaded_onlist_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 200})
    eve_loaded_offlist_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 100})
    eve_unloaded_onlist_id = client.alloc_item_id()
    eve_unloaded_offlist_id = client.alloc_item_id()
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_loaded_onlist_id, eve_unloaded_onlist_id])
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_module_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_module_id, sw_buffs=({eve_buff_id: -55}, eve_item_list_id))
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    return (
        eve_affectee_attr_id,
        eve_loaded_onlist_id,
        eve_loaded_offlist_id,
        eve_unloaded_onlist_id,
        eve_unloaded_offlist_id,
        api_fit2,
        api_module)


def test_loaded_onlist_to_loaded_offlist_remove(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     eve_loaded_offlist_id,
     _,
     _,
     api_fit2,
     api_module) = setup_test(client=client, consts=consts)
    api_child = api_fit2.add_drone(type_id=eve_loaded_onlist_id)
    api_module.change_module(add_projs=[api_child.id])
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].modified == approx(90)
    # Action
    api_child.change_drone(type_id=eve_loaded_offlist_id)
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].modified == approx(100)
    # Action
    api_module.remove()
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].modified == approx(100)


def test_loaded_onlist_to_unloaded_onlist_remove(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     eve_unloaded_onlist_id,
     _,
     api_fit2,
     api_module) = setup_test(client=client, consts=consts)
    api_child = api_fit2.add_drone(type_id=eve_loaded_onlist_id)
    api_module.change_module(add_projs=[api_child.id])
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].modified == approx(90)
    # Action
    api_child.change_drone(type_id=eve_unloaded_onlist_id)
    # Verification
    api_child.update()
    with check_no_field():
        api_child.attrs  # noqa: B018
    # Action
    api_module.remove()
    # Verification
    api_child.update()
    with check_no_field():
        api_child.attrs  # noqa: B018


def test_loaded_onlist_to_unloaded_offlist_remove(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     _,
     eve_unloaded_offlist_id,
     api_fit2,
     api_module) = setup_test(client=client, consts=consts)
    api_child = api_fit2.add_drone(type_id=eve_loaded_onlist_id)
    api_module.change_module(add_projs=[api_child.id])
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].modified == approx(90)
    # Action
    api_child.change_drone(type_id=eve_unloaded_offlist_id)
    # Verification
    api_child.update()
    with check_no_field():
        api_child.attrs  # noqa: B018
    # Action
    api_module.remove()
    # Verification
    api_child.update()
    with check_no_field():
        api_child.attrs  # noqa: B018


def test_loaded_offlist_to_loaded_onlist_remove(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     eve_loaded_offlist_id,
     _,
     _,
     api_fit2,
     api_module) = setup_test(client=client, consts=consts)
    api_child = api_fit2.add_drone(type_id=eve_loaded_offlist_id)
    api_module.change_module(add_projs=[api_child.id])
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].modified == approx(100)
    # Action
    api_child.change_drone(type_id=eve_loaded_onlist_id)
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].modified == approx(90)
    # Action
    api_module.remove()
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].modified == approx(200)


def test_unloaded_onlist_to_loaded_onlist_remove(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     eve_unloaded_onlist_id,
     _,
     api_fit2,
     api_module) = setup_test(client=client, consts=consts)
    api_child = api_fit2.add_drone(type_id=eve_unloaded_onlist_id)
    api_module.change_module(add_projs=[api_child.id])
    # Verification
    with check_no_field():
        api_child.attrs  # noqa: B018
    # Action
    api_child.change_drone(type_id=eve_loaded_onlist_id)
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].modified == approx(90)
    # Action
    api_module.remove()
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].modified == approx(200)


def test_unloaded_offlist_to_loaded_onlist_remove(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     _,
     eve_unloaded_offlist_id,
     api_fit2,
     api_module) = setup_test(client=client, consts=consts)
    api_child = api_fit2.add_drone(type_id=eve_unloaded_offlist_id)
    api_module.change_module(add_projs=[api_child.id])
    # Verification
    api_child.update()
    with check_no_field():
        api_child.attrs  # noqa: B018
    # Action
    api_child.change_drone(type_id=eve_loaded_onlist_id)
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].modified == approx(90)
    # Action
    api_module.remove()
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].modified == approx(200)
