from fw import approx, check_no_field


def test_affected(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_item_id)
    api_fit.add_fw_effect(type_id=eve_affector_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def test_unaffected_other_location(client, consts):
    # Make sure "top" entities described by other locations are not affected
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_affector_item_id)
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_unaffected_child(client, consts):
    # Check that items (in this case rig) are not affected if they belong to location even if
    # its "owner" (in this case ship) is affected
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fw_effect(type_id=eve_affector_item_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_unaffected_other_fit(client, consts):
    # Check that fit-wide modifications are not carried over to another fit
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.add_fw_effect(type_id=eve_affector_item_id)
    api_affectee_item = api_fit2.set_ship(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_replace_root(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item1_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    eve_affectee_item2_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item1 = api_fit.set_ship(type_id=eve_affectee_item1_id)
    api_fit.add_fw_effect(type_id=eve_affector_item_id)
    assert api_affectee_item1.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_affectee_item2 = api_fit.set_ship(type_id=eve_affectee_item2_id)
    assert api_affectee_item2.update().attrs[eve_affectee_attr_id].dogma == approx(60)


def setup_switch_type_id_test(*, client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 100})
    eve_affectee_struct_id = client.mk_eve_struct(attrs={eve_affectee_attr_id: 50})
    eve_affectee_unknown_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 25})
    eve_affectee_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_affector_item_id)
    return (
        eve_affectee_attr_id,
        eve_affectee_ship_id,
        eve_affectee_struct_id,
        eve_affectee_unknown_id,
        eve_affectee_not_loaded_id,
        api_fit,
        api_fw_effect)


def test_switch_type_id_ship_to_struct_remove(client, consts):
    (eve_affectee_attr_id,
     eve_affectee_ship_id,
     eve_affectee_struct_id,
     _,
     _,
     api_fit,
     api_fw_effect) = setup_switch_type_id_test(client=client, consts=consts)
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_ship_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_affectee_item.change_ship(type_id=eve_affectee_struct_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(50)
    # Action
    api_fw_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(50)


def test_switch_type_id_ship_to_unknown_remove(client, consts):
    (eve_affectee_attr_id,
     eve_affectee_ship_id,
     _,
     eve_affectee_unknown_id,
     _,
     api_fit,
     api_fw_effect) = setup_switch_type_id_test(client=client, consts=consts)
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_ship_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_affectee_item.change_ship(type_id=eve_affectee_unknown_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(25)
    # Action
    api_fw_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(25)


def test_switch_type_id_ship_to_not_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_affectee_ship_id,
     _,
     _,
     eve_affectee_not_loaded_id,
     api_fit,
     api_fw_effect) = setup_switch_type_id_test(client=client, consts=consts)
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_ship_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_affectee_item.change_ship(type_id=eve_affectee_not_loaded_id)
    # Verification
    api_affectee_item.update()
    with check_no_field():
        api_affectee_item.attrs  # noqa: B018
    # Action
    api_fw_effect.remove()
    # Verification
    api_affectee_item.update()
    with check_no_field():
        api_affectee_item.attrs  # noqa: B018


def test_switch_type_id_struct_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_affectee_ship_id,
     eve_affectee_struct_id,
     _,
     _,
     api_fit,
     api_fw_effect) = setup_switch_type_id_test(client=client, consts=consts)
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_struct_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(50)
    # Action
    api_affectee_item.change_ship(type_id=eve_affectee_ship_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_fw_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_unknown_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_affectee_ship_id,
     _,
     eve_affectee_unknown_id,
     _,
     api_fit,
     api_fw_effect) = setup_switch_type_id_test(client=client, consts=consts)
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_unknown_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(25)
    # Action
    api_affectee_item.change_ship(type_id=eve_affectee_ship_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_fw_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_not_loaded_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_affectee_ship_id,
     _,
     _,
     eve_affectee_not_loaded_id,
     api_fit,
     api_fw_effect) = setup_switch_type_id_test(client=client, consts=consts)
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_not_loaded_id)
    # Verification
    api_affectee_item.update()
    with check_no_field():
        api_affectee_item.attrs  # noqa: B018
    # Action
    api_affectee_item.change_ship(type_id=eve_affectee_ship_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_fw_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_src_to_struct(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_affector_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_affectee_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(datas=[eve_d1, eve_d2], cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_fw_effect_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id])
    eve_root_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_ship(datas=[eve_d1], id_=eve_root_id, attrs={eve_affectee_attr_id: 100})
    client.mk_eve_struct(datas=[eve_d2], id_=eve_root_id, attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_root = api_fit.set_ship(type_id=eve_root_id)
    api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_root.update().attrs[eve_affectee_attr_id].dogma == approx(120)
