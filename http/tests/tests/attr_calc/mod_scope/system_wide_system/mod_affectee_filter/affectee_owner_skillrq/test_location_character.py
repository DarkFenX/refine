from tests import approx


def test_affected_multiple(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    eve_char_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.set_character(type_id=eve_char_item_id)
    api_fit2.set_character(type_id=eve_char_item_id)
    api_affectee_item1 = api_fit1.add_drone(type_id=eve_affectee_item_id)
    api_affectee_item2 = api_fit2.add_drone(type_id=eve_affectee_item_id)
    api_sol.add_sw_effect(type_id=eve_affector_item_id)
    assert api_affectee_item1.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    assert api_affectee_item2.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def test_unaffected_non_owner_modifiable(client, consts):
    # Check that items which are not marked as owner-modifiable do not receive modification
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    eve_char_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_item_id)
    api_sol.add_sw_effect(type_id=eve_affector_item_id)
    api_affectee_item = api_fit.add_rig(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_unaffected_other_skillreq(client, consts):
    # Check that entities which don't have needed skill requirement are not affected
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill2_id: 1})
    eve_char_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_item_id)
    api_sol.add_sw_effect(type_id=eve_affector_item_id)
    api_affectee_item = api_fit.add_drone(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_replace_root(client, consts):
    # This behavior isn't defined in EVE, but we check how character presence influences
    # modifications with owner-skillreq filter. In our case it doesn't, because those are tracked
    # by fit ID
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    eve_char_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_sol.add_sw_effect(type_id=eve_affector_item_id)
    api_fit.set_character(type_id=eve_char_item_id)
    api_affectee_item = api_fit.add_drone(type_id=eve_affectee_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_fit.remove_character()
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    api_fit.set_character(type_id=eve_char_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def setup_switch_type_id_root_test(*, client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id])
    eve_affectee_item_id = client.mk_eve_drone(attrs={eve_affectee_attr_id: 100}, srqs={eve_skill_id: 1})
    eve_char_loaded_id = client.mk_eve_item()
    eve_char_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affectee_item = api_fit.add_drone(type_id=eve_affectee_item_id)
    api_sw_effect = api_sol.add_sw_effect(type_id=eve_affector_item_id)
    return eve_affectee_attr_id, eve_char_loaded_id, eve_char_not_loaded_id, api_fit, api_sw_effect, api_affectee_item


def test_switch_type_id_root_loaded_to_not_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_char_loaded_id,
     eve_char_not_loaded_id,
     api_fit,
     api_sw_effect,
     api_affectee_item) = setup_switch_type_id_root_test(client=client, consts=consts)
    api_character = api_fit.set_character(type_id=eve_char_loaded_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_character.change_character(type_id=eve_char_not_loaded_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_sw_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_switch_type_id_root_not_loaded_to_loaded_remove(client, consts):
    (eve_affectee_attr_id,
     eve_char_loaded_id,
     eve_char_not_loaded_id,
     api_fit,
     api_sw_effect,
     api_affectee_item) = setup_switch_type_id_root_test(client=client, consts=consts)
    api_character = api_fit.set_character(type_id=eve_char_not_loaded_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_character.change_character(type_id=eve_char_loaded_id)
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_sw_effect.remove()
    # Verification
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(100)
