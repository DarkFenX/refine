from tests import approx


def test_affected_via_ship(client, consts):
    # There are seem to be no such effects defined in EVE, but if my memory serves me properly, this
    # is how guidance disruptors used to work during dgmexpressions table days. Later that table was
    # removed and the effect was reworked to have strength reduction in falloff, and all associated
    # modifier info was removed. We support this scenario nevertheless.
    eve_skill_id = client.mk_eve_item()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(attrs={eve_attr2_id: 80}, srqs={eve_skill_id: 1})
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2.set_character(type_id=eve_char_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(96)


def test_affected_via_struct(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(attrs={eve_attr2_id: 80}, srqs={eve_skill_id: 1})
    eve_char_id = client.mk_eve_item()
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2.set_character(type_id=eve_char_id)
    api_struct = api_fit2.set_ship(type_id=eve_struct_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module.change_module(add_projs=[api_struct.id])
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(96)


def test_unaffected_non_owner_modifiable(client, consts):
    # Check that items which are not marked as owner-modifiable do not receive modification
    eve_skill_id = client.mk_eve_item()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(attrs={eve_attr2_id: 80}, srqs={eve_skill_id: 1})
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2.set_character(type_id=eve_char_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(80)


def test_unaffected_other_skillreq(client, consts):
    # Check that entities which don't have needed skill requirement are not affected
    eve_skill1_id = client.mk_eve_item()
    eve_skill2_id = client.mk_eve_item()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill1_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(attrs={eve_attr2_id: 80}, srqs={eve_skill2_id: 1})
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2.set_character(type_id=eve_char_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(80)


def test_unaffected_targeted_child(client, consts):
    # When it's not ship/structure which is getting targeted, target item shouldn't be affected
    eve_skill_id = client.mk_eve_item()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(attrs={eve_attr2_id: 80}, srqs={eve_skill_id: 1})
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2.set_character(type_id=eve_char_id)
    api_fit2.set_ship(type_id=eve_ship_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module.change_module(add_projs=[api_drone.id])
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(80)


def test_unaffected_other_fit(client, consts):
    # Check that targeted modifications are not carried over to another fit
    eve_skill_id = client.mk_eve_item()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(attrs={eve_attr2_id: 80}, srqs={eve_skill_id: 1})
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.set_character(type_id=eve_char_id)
    api_fit1.set_ship(type_id=eve_ship_id)
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_drone1 = api_fit1.add_drone(type_id=eve_drone_id)
    api_fit2 = api_sol.create_fit()
    api_fit2.set_character(type_id=eve_char_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_fit2.add_drone(type_id=eve_drone_id)
    api_fit3 = api_sol.create_fit()
    api_fit3.set_character(type_id=eve_char_id)
    api_fit3.set_ship(type_id=eve_ship_id)
    api_drone3 = api_fit3.add_drone(type_id=eve_drone_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_drone1.update().attrs[eve_attr2_id].dogma == approx(80)
    assert api_drone3.update().attrs[eve_attr2_id].dogma == approx(80)


def test_unaffected_nontgt_location_item(client, consts):
    # Targets shouldn't be affected by modifiers which do not have target location
    eve_skill_id = client.mk_eve_item()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.item,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(attrs={eve_attr2_id: 80}, srqs={eve_skill_id: 1})
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2.set_character(type_id=eve_char_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(80)


def test_unaffected_nontgt_location_ship(client, consts):
    # Targets shouldn't be affected by modifiers which do not have target location
    eve_skill_id = client.mk_eve_item()
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.ship,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(attrs={eve_attr2_id: 80}, srqs={eve_skill_id: 1})
    eve_char_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2.set_character(type_id=eve_char_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_drone = api_fit2.add_drone(type_id=eve_drone_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_drone.update().attrs[eve_attr2_id].dogma == approx(80)
