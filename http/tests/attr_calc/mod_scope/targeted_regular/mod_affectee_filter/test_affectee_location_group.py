from pytest import approx


def test_affected_state_change_child_ship(client, consts):
    # Make sure items on ship location which pass group check are affected
    eve_grp = client.mk_eve_item_group()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.tgt,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_attr2.id: 80})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.online)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)
    api_module.change_mod(state=consts.ApiState.active)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(96)
    api_module.change_mod(state=consts.ApiState.online)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)


def test_affected_state_change_child_struct(client, consts):
    # Make sure items on structure location which pass group check are affected
    eve_grp = client.mk_eve_item_group()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.tgt,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_attr2.id: 80})
    eve_struct = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.online)
    api_struct = api_fit2.set_struct(type_id=eve_struct.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    api_module.change_mod(add_tgts=[api_struct.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)
    api_module.change_mod(state=consts.ApiState.active)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(96)
    api_module.change_mod(state=consts.ApiState.online)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)


def test_affected_targeting_child_ship(client, consts):
    # Make sure items on ship location which pass group check are affected
    eve_grp = client.mk_eve_item_group()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.tgt,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_attr2.id: 80})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(96)
    api_module.change_mod(rm_tgts=[api_ship.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)


def test_affected_targeting_child_struct(client, consts):
    # Make sure items on structure location which pass group check are affected
    eve_grp = client.mk_eve_item_group()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.tgt,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_attr2.id: 80})
    eve_struct = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_struct = api_fit2.set_struct(type_id=eve_struct.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)
    api_module.change_mod(add_tgts=[api_struct.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(96)
    api_module.change_mod(rm_tgts=[api_struct.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)


def test_affected_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_grp = client.mk_eve_item_group()
    eve_src_attr = client.mk_eve_attr()
    eve_mid_attr = client.mk_eve_attr()
    eve_tgt_attr = client.mk_eve_attr()
    eve_src_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_src_attr.id,
        tgt_attr_id=eve_mid_attr.id)
    eve_src_effect = client.mk_eve_effect(mod_info=[eve_src_mod])
    eve_src_item = client.mk_eve_item(attrs={eve_src_attr.id: 50}, eff_ids=[eve_src_effect.id])
    eve_mid_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.tgt,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_mid_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_mid_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mid_mod])
    eve_mid_item = client.mk_eve_item(
        attrs={eve_mid_attr.id: 20},
        eff_ids=[eve_mid_effect.id],
        defeff_id=eve_mid_effect.id)
    eve_tgt_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_tgt_attr.id: 80})
    eve_ship_item = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.set_ship(type_id=eve_ship_item.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_item.id)
    api_tgt_item = api_fit2.add_rig(type_id=eve_tgt_item.id)
    api_mid_item = api_fit1.add_mod(type_id=eve_mid_item.id, state=consts.ApiState.active)
    api_mid_item.change_mod(add_tgts=[api_ship.id])
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(96)
    api_src_item = api_fit1.add_rig(type_id=eve_src_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(104)
    api_src_item.remove()
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(96)


def test_unaffected_root(client, consts):
    # Ship shouldn't be affected
    eve_grp = client.mk_eve_item_group()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.tgt,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_attr2.id: 80})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(80)


def test_unaffected_char_child(client, consts):
    # On-character items shouldn't be affected
    eve_grp = client.mk_eve_item_group()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.tgt,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_implant = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_attr2.id: 80})
    eve_ship = client.mk_eve_item(grp_id=eve_grp.id)
    eve_char = client.mk_eve_item(grp_id=eve_grp.id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_char(type_id=eve_char.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_implant = api_fit2.add_implant(type_id=eve_implant.id)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_implant.update().attrs[eve_attr2.id].dogma == approx(80)


def test_unaffected_targeted_child(client, consts):
    # When it's not ship/structure which is getting targeted, target item shouldn't be affected
    eve_grp = client.mk_eve_item_group()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.tgt,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_attr2.id: 80})
    eve_ship = client.mk_eve_item(grp_id=eve_grp.id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module.change_mod(add_tgts=[api_drone.id])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)


def test_unaffected_via_child_target(client, consts):
    # Ship items shouldn't be affected when target is something which isn't ship (e.g. drone)
    eve_grp = client.mk_eve_item_group()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.tgt,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_attr2.id: 80})
    eve_drone = client.mk_eve_item()
    eve_ship = client.mk_eve_item(grp_id=eve_grp.id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module.change_mod(add_tgts=[api_drone.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)


def test_unaffected_other_group(client, consts):
    # Check that entities belonging to other item groups are not affected
    eve_grp1 = client.mk_eve_item_group()
    eve_grp2 = client.mk_eve_item_group()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.tgt,
        grp=eve_grp1.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp2.id, attrs={eve_attr2.id: 80})
    eve_ship = client.mk_eve_item(grp_id=eve_grp1.id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)


def test_unaffected_other_fit(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.tgt,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_attr2.id: 80})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit3 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship.id)
    api_fit3.set_ship(type_id=eve_ship.id)
    api_rig = api_fit3.add_rig(type_id=eve_rig.id)
    api_module.change_mod(add_tgts=[api_ship2.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)


def test_unaffected_nontgt_domain_item(client, consts):
    # Targets shouldn't be affected by modifiers which do not have target domain
    eve_grp = client.mk_eve_item_group()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.item,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_attr2.id: 80})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)


def test_unaffected_nontgt_domain_ship(client, consts):
    # Targets shouldn't be affected by modifiers which do not have target domain
    eve_grp = client.mk_eve_item_group()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.ship,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_attr2.id: 80})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)


def test_replace_target(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.tgt,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_attr2.id: 80})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship1 = api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    api_module.change_mod(add_tgts=[api_ship1.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(96)
    api_ship1.remove()
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship.id)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)
    api_module.change_mod(add_tgts=[api_ship2.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(96)
