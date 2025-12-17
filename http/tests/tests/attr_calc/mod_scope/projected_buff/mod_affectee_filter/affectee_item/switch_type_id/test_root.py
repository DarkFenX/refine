from fw import approx, check_no_field


def setup_root_test(*, client, consts):
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_loaded_onlist_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 7.5})
    eve_loaded_offlist_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 5})
    eve_unloaded_onlist_id = client.alloc_item_id()
    eve_unloaded_offlist_id = client.alloc_item_id()
    eve_item_list_id = client.mk_eve_item_list(inc_type_ids=[eve_loaded_onlist_id, eve_unloaded_onlist_id])
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_mul,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id)])
    eve_proj_effect_id = client.mk_eve_item()
    client.mk_eve_space_comp(type_id=eve_proj_effect_id, sw_buffs=({eve_buff_id: 5}, eve_item_list_id))
    client.create_sources()
    api_sol = client.create_sol()
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_fit = api_sol.create_fit()
    return (
        eve_affectee_attr_id,
        eve_loaded_onlist_id,
        eve_loaded_offlist_id,
        eve_unloaded_onlist_id,
        eve_unloaded_offlist_id,
        api_fit,
        api_proj_effect)


def test_loaded_onlist_to_loaded_offlist_remove_pe_remove_root(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     eve_loaded_offlist_id,
     _,
     _,
     api_fit,
     api_proj_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_loaded_onlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_root.change_ship(type_id=eve_loaded_offlist_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(5)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(5)
    # Action & verification
    api_root.remove()


def test_loaded_onlist_to_loaded_offlist_remove_root_remove_pe(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     eve_loaded_offlist_id,
     _,
     _,
     api_fit,
     api_proj_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_loaded_onlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_root.change_ship(type_id=eve_loaded_offlist_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(5)
    # Action & verification
    api_root.remove()
    api_proj_effect.remove()


def test_loaded_onlist_to_unloaded_onlist_remove_pe_remove_root(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     eve_unloaded_onlist_id,
     _,
     api_fit,
     api_proj_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_loaded_onlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_root.change_ship(type_id=eve_unloaded_onlist_id)
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action
    api_proj_effect.remove()
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action & verification
    api_root.remove()


def test_loaded_onlist_to_unloaded_onlist_remove_root_remove_pe(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     eve_unloaded_onlist_id,
     _,
     api_fit,
     api_proj_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_loaded_onlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_root.change_ship(type_id=eve_unloaded_onlist_id)
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action & verification
    api_root.remove()
    api_proj_effect.remove()


def test_loaded_onlist_to_unloaded_offlist_remove_pe_remove_root(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     _,
     eve_unloaded_offlist_id,
     api_fit,
     api_proj_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_loaded_onlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_root.change_ship(type_id=eve_unloaded_offlist_id)
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action
    api_proj_effect.remove()
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action & verification
    api_root.remove()


def test_loaded_onlist_to_unloaded_offlist_remove_root_remove_pe(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     _,
     eve_unloaded_offlist_id,
     api_fit,
     api_proj_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_loaded_onlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_root.change_ship(type_id=eve_unloaded_offlist_id)
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action & verification
    api_root.remove()
    api_proj_effect.remove()


def test_loaded_offlist_to_loaded_onlist_remove_pe_remove_root(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     eve_loaded_offlist_id,
     _,
     _,
     api_fit,
     api_proj_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_loaded_offlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(5)
    # Action
    api_root.change_ship(type_id=eve_loaded_onlist_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action & verification
    api_root.remove()


def test_loaded_offlist_to_loaded_onlist_remove_root_remove_pe(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     eve_loaded_offlist_id,
     _,
     _,
     api_fit,
     api_proj_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_loaded_offlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(5)
    # Action
    api_root.change_ship(type_id=eve_loaded_onlist_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action & verification
    api_root.remove()
    api_proj_effect.remove()


def test_unloaded_onlist_to_loaded_onlist_remove_pe_remove_root(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     eve_unloaded_onlist_id,
     _,
     api_fit,
     api_proj_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_unloaded_onlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action
    api_root.change_ship(type_id=eve_loaded_onlist_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action & verification
    api_root.remove()


def test_unloaded_onlist_to_loaded_onlist_remove_root_remove_pe(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     eve_unloaded_onlist_id,
     _,
     api_fit,
     api_proj_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_unloaded_onlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action
    api_root.change_ship(type_id=eve_loaded_onlist_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action & verification
    api_root.remove()
    api_proj_effect.remove()


def test_unloaded_offlist_to_loaded_onlist_pe_remove_root(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     _,
     eve_unloaded_offlist_id,
     api_fit,
     api_proj_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_unloaded_offlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action
    api_root.change_ship(type_id=eve_loaded_onlist_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action & verification
    api_root.remove()


def test_unloaded_offlist_to_loaded_onlist_remove_root_remove_pe(client, consts):
    (eve_affectee_attr_id,
     eve_loaded_onlist_id,
     _,
     _,
     eve_unloaded_offlist_id,
     api_fit,
     api_proj_effect) = setup_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_unloaded_offlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_root.id])
    # Verification
    api_root.update()
    with check_no_field():
        api_root.attrs  # noqa: B018
    # Action
    api_root.change_ship(type_id=eve_loaded_onlist_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action & verification
    api_root.remove()
    api_proj_effect.remove()
