from tests import approx, check_no_field


def test_affected_via_ship(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_implant_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def test_affected_via_struct(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_implant_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_char_id = client.mk_eve_item()
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_proj_effect.change_proj_effect(add_projs=[api_struct.id])
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def test_unaffected_other_location(client, consts):
    # Check that entities from other locations are not affected
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_rig_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_rig.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_unaffected_other_fit(client, consts):
    # Check that projected modifications are not carried over to another fit
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_implant_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.set_character(type_id=eve_char_id)
    api_fit2.set_character(type_id=eve_char_id)
    api_implant = api_fit2.add_implant(type_id=eve_implant_id)
    api_ship = api_fit1.set_ship(type_id=eve_ship_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_replace_root(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_implant_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_fit.remove_character()
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_fit.set_character(type_id=eve_char_id)
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def setup_switch_type_id_char_test(*, client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_implant_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_char_loaded_id = client.mk_eve_item()
    eve_char_not_loaded_id = client.alloc_item_id()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item = api_fit.add_implant(type_id=eve_implant_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    return eve_affectee_attr_id, eve_char_loaded_id, eve_char_not_loaded_id, api_fit, api_proj_effect, api_affectee_item


def test_switch_type_id_char_loaded_to_not_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_char_loaded_id,
     eve_char_not_loaded_id,
     api_fit,
     api_proj_effect,
     api_affectee_item) = setup_switch_type_id_char_test(client=client, consts=consts)
    api_character = api_fit.set_character(type_id=eve_char_loaded_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_character.change_character(type_id=eve_char_not_loaded_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_char_not_loaded_to_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_char_loaded_id,
     eve_char_not_loaded_id,
     api_fit,
     api_proj_effect,
     api_affectee_item) = setup_switch_type_id_char_test(client=client, consts=consts)
    api_character = api_fit.set_character(type_id=eve_char_not_loaded_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_character.change_character(type_id=eve_char_loaded_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def setup_switch_type_id_tgt_test(*, client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_implant_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_char_id = client.mk_eve_item()
    eve_tgt_ship_id = client.mk_eve_ship()
    eve_tgt_struct_id = client.mk_eve_struct()
    eve_tgt_unknown_id = client.mk_eve_item()
    eve_tgt_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_affectee_item = api_fit.add_implant(type_id=eve_implant_id)
    return (
        eve_affectee_attr_id,
        eve_tgt_ship_id,
        eve_tgt_struct_id,
        eve_tgt_unknown_id,
        eve_tgt_not_loaded_id,
        api_fit,
        api_affectee_item,
        api_proj_effect)


def test_switch_type_id_tgt_ship_to_struct_remove(client, consts):
    (eve_affectee_attr_id,
     eve_tgt_ship_id,
     eve_tgt_struct_id,
     _,
     _,
     api_fit,
     api_affectee_item,
     api_proj_effect) = setup_switch_type_id_tgt_test(client=client, consts=consts)
    api_tgt = api_fit.set_ship(type_id=eve_tgt_ship_id)
    api_proj_effect.change_proj_effect(add_projs=[api_tgt.id])
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_tgt.change_ship(type_id=eve_tgt_struct_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_tgt_ship_to_unknown_remove(client, consts):
    (eve_affectee_attr_id,
     eve_tgt_ship_id,
     _,
     eve_tgt_unknown_id,
     _,
     api_fit,
     api_affectee_item,
     api_proj_effect) = setup_switch_type_id_tgt_test(client=client, consts=consts)
    api_tgt = api_fit.set_ship(type_id=eve_tgt_ship_id)
    api_proj_effect.change_proj_effect(add_projs=[api_tgt.id])
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_tgt.change_ship(type_id=eve_tgt_unknown_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_tgt_ship_to_not_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_tgt_ship_id,
     _,
     _,
     eve_tgt_not_loaded_id,
     api_fit,
     api_affectee_item,
     api_proj_effect) = setup_switch_type_id_tgt_test(client=client, consts=consts)
    api_tgt = api_fit.set_ship(type_id=eve_tgt_ship_id)
    api_proj_effect.change_proj_effect(add_projs=[api_tgt.id])
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_tgt.change_ship(type_id=eve_tgt_not_loaded_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_tgt_struct_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_tgt_ship_id,
     eve_tgt_struct_id,
     _,
     _,
     api_fit,
     api_affectee_item,
     api_proj_effect) = setup_switch_type_id_tgt_test(client=client, consts=consts)
    api_tgt = api_fit.set_ship(type_id=eve_tgt_struct_id)
    api_proj_effect.change_proj_effect(add_projs=[api_tgt.id])
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_tgt.change_ship(type_id=eve_tgt_ship_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_tgt_struct_to_not_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     _,
     eve_tgt_struct_id,
     _,
     eve_tgt_not_loaded_id,
     api_fit,
     api_affectee_item,
     api_proj_effect) = setup_switch_type_id_tgt_test(client=client, consts=consts)
    api_tgt = api_fit.set_ship(type_id=eve_tgt_struct_id)
    api_proj_effect.change_proj_effect(add_projs=[api_tgt.id])
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_tgt.change_ship(type_id=eve_tgt_not_loaded_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_tgt_unknown_to_ship_remove(client, consts):
    (eve_affectee_attr_id,
     eve_tgt_ship_id,
     _,
     eve_tgt_unknown_id,
     _,
     api_fit,
     api_affectee_item,
     api_proj_effect) = setup_switch_type_id_tgt_test(client=client, consts=consts)
    api_tgt = api_fit.set_ship(type_id=eve_tgt_unknown_id)
    api_proj_effect.change_proj_effect(add_projs=[api_tgt.id])
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_tgt.change_ship(type_id=eve_tgt_ship_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_proj_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_affectee(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_implant1_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    eve_implant2_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 50})
    eve_implant3_id = client.alloc_item_id()
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_implant = api_fit.add_implant(type_id=eve_implant1_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect_id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    # Verification
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_implant.change_implant(type_id=eve_implant2_id)
    # Verification
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(60)
    # Action
    api_implant.change_implant(type_id=eve_implant3_id)
    # Verification
    api_implant.update()
    with check_no_field():
        api_implant.attrs  # noqa: B018
    # Action
    api_implant.change_implant(type_id=eve_implant1_id)
    # Verification
    assert api_implant.update().attrs[eve_affectee_attr_id].dogma == approx(120)
