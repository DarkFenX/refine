from tests import approx


def setup_root_test(*, client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        loc_grp_mods=[client.mk_eve_buff_mod(attr_id=eve_affectee_attr_id, group_id=eve_grp_id)])
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mod_titan_effect_generator,
        cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 30},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 200})
    eve_root_ship_id = client.mk_eve_ship()
    eve_root_struct_id = client.mk_eve_struct()
    eve_root_unknown_id = client.mk_eve_item()
    eve_root_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    return (
        eve_affectee_attr_id,
        eve_root_ship_id,
        eve_root_struct_id,
        eve_root_unknown_id,
        eve_root_not_loaded_id,
        api_fit2,
        api_module,
        api_rig)


def test_root_ship_to_struct_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _,
     _,
     api_fit2,
     api_module,
     api_rig) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_ship_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(260)
    # Action
    api_root.change_ship(type_id=eve_root_struct_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    # Action
    api_module.remove()
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_root_ship_to_unknown_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     eve_root_unknown_id,
     _,
     api_fit2,
     api_module,
     api_rig) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_ship_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(260)
    # Action
    api_root.change_ship(type_id=eve_root_unknown_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    # Action
    api_module.remove()
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_root_ship_to_not_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     _,
     eve_root_not_loaded_id,
     api_fit2,
     api_module,
     api_rig) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_ship_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(260)
    # Action
    api_root.change_ship(type_id=eve_root_not_loaded_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    # Action
    api_module.remove()
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_root_struct_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _,
     _,
     api_fit2,
     api_module,
     api_rig) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_struct_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(260)
    # Action
    api_module.remove()
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_root_unknown_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     eve_root_unknown_id,
     _,
     api_fit2,
     api_module,
     api_rig) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_unknown_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(260)
    # Action
    api_module.remove()
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)


def test_root_not_loaded_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     _,
     eve_root_not_loaded_id,
     api_fit2,
     api_module,
     api_rig) = setup_root_test(client=client, consts=consts)
    api_root = api_fit2.set_ship(type_id=eve_root_not_loaded_id)
    api_module.change_module(add_projs=[api_root.id])
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    # Action
    api_root.change_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(260)
    # Action
    api_module.remove()
    # Verification
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
