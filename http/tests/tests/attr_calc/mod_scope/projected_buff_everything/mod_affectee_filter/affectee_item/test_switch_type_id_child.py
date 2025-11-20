from tests import approx, check_no_field


def setup_child_test(*, client, consts):
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
    eve_onlist_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 7.5})
    eve_offlist_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 5})
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    return (
        eve_affectee_attr_id,
        eve_onlist_id,
        eve_offlist_id,
        eve_not_loaded_id,
        api_fit,
        api_proj_effect)


def test_child_onlist_to_offlist_remove_pe_remove_child(client, consts):
    (eve_affectee_attr_id,
     eve_onlist_id,
     eve_offlist_id,
     _,
     api_fit,
     api_proj_effect) = setup_child_test(client=client, consts=consts)
    api_child = api_fit.add_drone(type_id=eve_onlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_child.id])
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_child.change_drone(type_id=eve_offlist_id)
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(5)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(5)
    # Action & verification
    api_child.remove()


def test_child_onlist_to_offlist_remove_child_remove_pe(client, consts):
    (eve_affectee_attr_id,
     eve_onlist_id,
     eve_offlist_id,
     _,
     api_fit,
     api_proj_effect) = setup_child_test(client=client, consts=consts)
    api_child = api_fit.add_drone(type_id=eve_onlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_child.id])
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_child.change_drone(type_id=eve_offlist_id)
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(5)
    # Action & verification
    api_child.remove()
    api_proj_effect.remove()


def test_child_onlist_to_not_loaded_remove_pe_remove_child(client, consts):
    (eve_affectee_attr_id,
     eve_onlist_id,
     _,
     eve_not_loaded_id,
     api_fit,
     api_proj_effect) = setup_child_test(client=client, consts=consts)
    api_child = api_fit.add_drone(type_id=eve_onlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_child.id])
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_child.change_drone(type_id=eve_not_loaded_id)
    # Verification
    api_child.update()
    with check_no_field():
        api_child.attrs  # noqa: B018
    # Action
    api_proj_effect.remove()
    # Verification
    api_child.update()
    with check_no_field():
        api_child.attrs  # noqa: B018
    # Action & verification
    api_child.remove()


def test_child_onlist_to_not_loaded_remove_child_remove_pe(client, consts):
    (eve_affectee_attr_id,
     eve_onlist_id,
     _,
     eve_not_loaded_id,
     api_fit,
     api_proj_effect) = setup_child_test(client=client, consts=consts)
    api_child = api_fit.add_drone(type_id=eve_onlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_child.id])
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_child.change_drone(type_id=eve_not_loaded_id)
    # Verification
    api_child.update()
    with check_no_field():
        api_child.attrs  # noqa: B018
    # Action & verification
    api_child.remove()
    api_proj_effect.remove()


def test_child_offlist_to_onlist_remove_pe_remove_child(client, consts):
    (eve_affectee_attr_id,
     eve_onlist_id,
     eve_offlist_id,
     _,
     api_fit,
     api_proj_effect) = setup_child_test(client=client, consts=consts)
    api_child = api_fit.add_drone(type_id=eve_offlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_child.id])
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(5)
    # Action
    api_child.change_drone(type_id=eve_onlist_id)
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action & verification
    api_child.remove()


def test_child_offlist_to_onlist_remove_child_remove_pe(client, consts):
    (eve_affectee_attr_id,
     eve_onlist_id,
     eve_offlist_id,
     _,
     api_fit,
     api_proj_effect) = setup_child_test(client=client, consts=consts)
    api_child = api_fit.add_drone(type_id=eve_offlist_id)
    api_proj_effect.change_proj_effect(add_projs=[api_child.id])
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(5)
    # Action
    api_child.change_drone(type_id=eve_onlist_id)
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action & verification
    api_child.remove()
    api_proj_effect.remove()


def test_child_not_loaded_to_onlist_remove_pe_remove_child(client, consts):
    (eve_affectee_attr_id,
     eve_onlist_id,
     _,
     eve_not_loaded_id,
     api_fit,
     api_proj_effect) = setup_child_test(client=client, consts=consts)
    api_child = api_fit.add_drone(type_id=eve_not_loaded_id)
    api_proj_effect.change_proj_effect(add_projs=[api_child.id])
    # Verification
    api_child.update()
    with check_no_field():
        api_child.attrs  # noqa: B018
    # Action
    api_child.change_drone(type_id=eve_onlist_id)
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(7.5)
    # Action & verification
    api_child.remove()


def test_child_not_loaded_to_onlist_remove_child_remove_pe(client, consts):
    (eve_affectee_attr_id,
     eve_onlist_id,
     _,
     eve_not_loaded_id,
     api_fit,
     api_proj_effect) = setup_child_test(client=client, consts=consts)
    api_child = api_fit.add_drone(type_id=eve_not_loaded_id)
    api_proj_effect.change_proj_effect(add_projs=[api_child.id])
    # Verification
    api_child.update()
    with check_no_field():
        api_child.attrs  # noqa: B018
    # Action
    api_child.change_drone(type_id=eve_onlist_id)
    # Verification
    assert api_child.update().attrs[eve_affectee_attr_id].dogma == approx(37.5)
    # Action & verification
    api_child.remove()
    api_proj_effect.remove()
