from tests import approx


def test_affected_root_ship(client, consts):
    # Make sure ships are affected by targeted effects
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 3}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr2_id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2_id].dogma == approx(1)


def test_affected_root_struct(client, consts):
    # Make sure structures are affected by targeted effects
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 3}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_struct_id = client.mk_eve_struct(attrs={eve_attr2_id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_struct = api_fit2.set_ship(type_id=eve_struct_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module.change_mod(add_projs=[api_struct.id])
    assert api_struct.update().attrs[eve_attr2_id].dogma == approx(1)


def test_affected_child(client, consts):
    # Make sure child targetable items are affected by targeted effects
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 3}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_item(attrs={eve_attr2_id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module.change_mod(add_projs=[api_drone.id])
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(1)


def test_unaffected_root(client, consts):
    # Character shouldn't be affected
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 3}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_char_id = client.mk_eve_item(attrs={eve_attr2_id: -2})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_char = api_fit2.set_char(type_id=eve_char_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_char.update().attrs[eve_attr2_id].dogma == approx(-2)


def test_unaffected_child_via_root(client, consts):
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 3}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_item(attrs={eve_attr2_id: -2})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(-2)


def test_unaffected_root_via_child(client, consts):
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 3}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr2_id: -2})
    eve_drone_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module.change_mod(add_projs=[api_drone.id])
    assert api_ship.update().attrs[eve_attr2_id].dogma == approx(-2)


def test_unaffected_root_other_fit(client, consts):
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 3}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr2_id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit3 = api_sol.create_fit()
    api_ship2 = api_fit2.set_ship(type_id=eve_ship_id)
    api_ship3 = api_fit3.set_ship(type_id=eve_ship_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module.change_mod(add_projs=[api_ship2.id])
    assert api_ship3.update().attrs[eve_attr2_id].dogma == approx(-2)


def test_unaffected_child_other_fit(client, consts):
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 3}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_item(attrs={eve_attr2_id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit3 = api_sol.create_fit()
    api_drone2 = api_fit2.add_drone(type_id=eve_drone_id)
    api_drone3 = api_fit3.add_drone(type_id=eve_drone_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module.change_mod(add_projs=[api_drone2.id])
    assert api_drone3.update().attrs[eve_attr2_id].dogma == approx(-2)


def test_unaffected_nontgt_location_item(client, consts):
    # Targets shouldn't be affected by modifiers which do not have target location
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 3}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr2_id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2_id].dogma == approx(-2)


def test_unaffected_nontgt_location_ship(client, consts):
    # Targets shouldn't be affected by modifiers which do not have target location
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 3}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr2_id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2_id].dogma == approx(-2)
