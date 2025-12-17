from fw import approx, check_no_field


def test_affected(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_implant(type_id=eve_affector_item_id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def test_unaffected_other_location(client, consts):
    # Check that entities from other locations are not affected
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_fit.add_implant(type_id=eve_affector_item_id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_unaffected_other_fit(client, consts):
    # Check that local modifications are not carried over to another fit
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.set_ship(type_id=eve_ship_id)
    api_fit2.set_ship(type_id=eve_ship_id)
    api_fit1.add_implant(type_id=eve_affector_item_id)
    api_affectee_item = api_fit2.add_rig(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_affector_attr_id = client.mk_eve_attr()
    eve_middle_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_affector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_middle_attr_id)
    eve_affector_effect_id = client.mk_eve_effect(mod_info=[eve_affector_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 2}, eff_ids=[eve_affector_effect_id])
    eve_middle_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_middle_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_middle_effect_id = client.mk_eve_effect(mod_info=[eve_middle_mod])
    eve_middle_item_id = client.mk_eve_item(attrs={eve_middle_attr_id: 20}, eff_ids=[eve_middle_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_ship_id = client.mk_eve_ship()
    eve_char_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_item_id)
    api_fit.add_implant(type_id=eve_middle_item_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_affector_item = api_fit.add_rig(type_id=eve_affector_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(140)
    api_affector_item.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def test_replace_root(client, consts):
    # Modifiers which target items on ship location shouldn't apply when ship isn't set
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_affector_item_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_ship.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    api_fit.set_ship(type_id=eve_ship_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def setup_switch_type_id_root_test(*, client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_root_ship_id = client.mk_eve_ship()
    eve_root_struct_id = client.mk_eve_struct()
    eve_root_unknown_id = client.mk_eve_item()
    eve_root_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affector = api_fit.add_implant(type_id=eve_affector_item_id)
    api_affectee = api_fit.add_rig(type_id=eve_affectee_item_id)
    return (
        eve_affectee_attr_id,
        eve_root_ship_id,
        eve_root_struct_id,
        eve_root_unknown_id,
        eve_root_not_loaded_id,
        api_fit,
        api_affector,
        api_affectee)


def test_switch_type_id_root_ship_to_struct_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _,
     _,
     api_fit,
     api_affector,
     api_affectee) = setup_switch_type_id_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_root.change_ship(type_id=eve_root_struct_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_affector.remove()
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_root_ship_to_unknown_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     eve_root_unknown_id,
     _,
     api_fit,
     api_affector,
     api_affectee) = setup_switch_type_id_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_root.change_ship(type_id=eve_root_unknown_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_affector.remove()
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_root_ship_to_not_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     _,
     eve_root_not_loaded_id,
     api_fit,
     api_affector,
     api_affectee) = setup_switch_type_id_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_root.change_ship(type_id=eve_root_not_loaded_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_affector.remove()
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_root_struct_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     eve_root_struct_id,
     _,
     _,
     api_fit,
     api_affector,
     api_affectee) = setup_switch_type_id_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_root.change_ship(type_id=eve_root_struct_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_affector.remove()
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_root_unknown_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     eve_root_unknown_id,
     _,
     api_fit,
     api_affector,
     api_affectee) = setup_switch_type_id_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_root.change_ship(type_id=eve_root_unknown_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_affector.remove()
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_root_not_loaded_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_root_ship_id,
     _,
     _,
     eve_root_not_loaded_id,
     api_fit,
     api_affector,
     api_affectee) = setup_switch_type_id_root_test(client=client, consts=consts)
    api_root = api_fit.set_ship(type_id=eve_root_ship_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_root.change_ship(type_id=eve_root_not_loaded_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_affector.remove()
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_affectee(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_affectee_item2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 50})
    eve_affectee_item3_id = client.alloc_item_id()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_implant(type_id=eve_affector_item_id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item1_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_affectee_item.change_rig(type_id=eve_affectee_item2_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(60)
    # Action
    api_affectee_item.change_rig(type_id=eve_affectee_item3_id)
    # Verification
    api_affectee_item.update()
    with check_no_field():
        api_affectee_item.attrs  # noqa: B018
    # Action
    api_affectee_item.change_rig(type_id=eve_affectee_item1_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def test_switch_src_to_struct(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_affector_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_affectee_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(datas=[eve_d1, eve_d2], mod_info=[eve_mod])
    eve_affector_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id])
    eve_affectee_id = client.mk_eve_item(datas=[eve_d1, eve_d2], attrs={eve_affectee_attr_id: 100})
    eve_root_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_ship(datas=[eve_d1], id_=eve_root_id)
    client.mk_eve_struct(datas=[eve_d2], id_=eve_root_id)
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_affector_id)
    api_fit.set_ship(type_id=eve_root_id)
    api_affectee = api_fit.add_rig(type_id=eve_affectee_id)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_affectee.update().attrs[eve_affectee_attr_id].dogma == approx(120)
