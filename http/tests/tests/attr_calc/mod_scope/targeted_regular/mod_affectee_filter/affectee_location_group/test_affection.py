from tests import approx


def test_affected_child_ship(client, consts):
    # Make sure items on ship location which pass group check are affected
    eve_grp_id = client.mk_eve_item_group()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_attr2_id: 80})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(96)


def test_affected_child_struct(client, consts):
    # Make sure items on structure location which pass group check are affected
    eve_grp_id = client.mk_eve_item_group()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_attr2_id: 80})
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_struct = api_fit2.set_ship(type_id=eve_struct_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_struct.id])
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(96)


def test_affected_charge(client, consts):
    # Make sure items on structure location which pass group check are affected
    eve_grp_id = client.mk_eve_item_group()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module1_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item()
    eve_charge_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_attr2_id: 80})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module1 = api_fit1.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_module2 = api_fit2.add_module(type_id=eve_module2_id, charge_type_id=eve_charge_id)
    api_module1.change_module(add_projs=[api_ship.id])
    assert api_module2.charge.update().attrs[eve_attr2_id].dogma == approx(96)


def test_unaffected_root(client, consts):
    # Ship shouldn't be affected
    eve_grp_id = client.mk_eve_ship_group()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(grp_id=eve_grp_id, attrs={eve_attr2_id: 80})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2_id].dogma == approx(80)


def test_unaffected_char_child(client, consts):
    # On-character items shouldn't be affected
    eve_grp_id = client.mk_eve_ship_group()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_implant_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_attr2_id: 80})
    eve_ship_id = client.mk_eve_ship(grp_id=eve_grp_id)
    eve_char_id = client.mk_eve_item(grp_id=eve_grp_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2.set_character(type_id=eve_char_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_implant = api_fit2.add_implant(type_id=eve_implant_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_implant.update().attrs[eve_attr2_id].dogma == approx(80)


def test_unaffected_targeted_child(client, consts):
    # When it's not ship/structure which is getting targeted, target item shouldn't be affected
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_drone_grp_id = client.mk_eve_drone_group()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_ship_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_ship_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_drone_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_drone_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_ship_mod, eve_drone_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(grp_id=eve_drone_grp_id, attrs={eve_attr2_id: 80})
    eve_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2.set_ship(type_id=eve_ship_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module.change_module(add_projs=[api_drone.id])
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(80)


def test_unaffected_via_child_target(client, consts):
    # Ship items shouldn't be affected when target is something which isn't ship (e.g. drone)
    eve_grp_id = client.mk_eve_ship_group()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_attr2_id: 80})
    eve_drone_id = client.mk_eve_drone()
    eve_ship_id = client.mk_eve_ship(grp_id=eve_grp_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module.change_module(add_projs=[api_drone.id])
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(80)


def test_unaffected_other_group(client, consts):
    # Check that entities belonging to other item groups are not affected
    eve_grp1_id = client.mk_eve_ship_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_grp1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(grp_id=eve_grp2_id, attrs={eve_attr2_id: 80})
    eve_ship_id = client.mk_eve_ship(grp_id=eve_grp1_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(80)


def test_unaffected_other_fit(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_attr2_id: 80})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit3 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship_id)
    api_fit3.set_ship(type_id=eve_ship_id)
    api_rig = api_fit3.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_ship2.id])
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(80)


def test_unaffected_nontgt_location_item(client, consts):
    # Targets shouldn't be affected by modifiers which do not have target location
    eve_grp_id = client.mk_eve_item_group()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.item,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_attr2_id: 80})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(80)


def test_unaffected_nontgt_location_ship(client, consts):
    # Targets shouldn't be affected by modifiers which do not have target location
    eve_grp_id = client.mk_eve_item_group()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.ship,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_attr2_id: 80})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(80)
