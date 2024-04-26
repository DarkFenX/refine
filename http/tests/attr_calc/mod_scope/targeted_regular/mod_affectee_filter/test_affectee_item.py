from pytest import approx


def test_affected_state_change_root_ship(client, consts):
    # Make sure ships are affected by targeted effects
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_attr2.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.online)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(-2)
    api_module.change_mod(state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(1)
    api_module.change_mod(state=consts.ApiState.online)
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_affected_state_change_root_struct(client, consts):
    # Make sure structures are affected by targeted effects
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_struct = client.mk_eve_item(attrs={eve_attr2.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_struct = api_fit2.set_struct(type_id=eve_struct.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.online)
    api_module.change_mod(add_tgts=[api_struct.id])
    assert api_struct.update().attrs[eve_attr2.id].dogma == approx(-2)
    api_module.change_mod(state=consts.ApiState.active)
    assert api_struct.update().attrs[eve_attr2.id].dogma == approx(1)
    api_module.change_mod(state=consts.ApiState.online)
    assert api_struct.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_affected_state_change_child(client, consts):
    # Make sure child targetable items are affected by targeted effects
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.online)
    api_module.change_mod(add_tgts=[api_drone.id])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(-2)
    api_module.change_mod(state=consts.ApiState.active)
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(1)
    api_module.change_mod(state=consts.ApiState.online)
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_affected_targeting_root_ship(client, consts):
    # Make sure ships are affected by targeted effects
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_attr2.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(-2)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(1)
    api_module.change_mod(rm_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_affected_targeting_root_struct(client, consts):
    # Make sure structures are affected by targeted effects
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_struct = client.mk_eve_item(attrs={eve_attr2.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_struct = api_fit2.set_struct(type_id=eve_struct.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    assert api_struct.update().attrs[eve_attr2.id].dogma == approx(-2)
    api_module.change_mod(add_tgts=[api_struct.id])
    assert api_struct.update().attrs[eve_attr2.id].dogma == approx(1)
    api_module.change_mod(rm_tgts=[api_struct.id])
    assert api_struct.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_affected_targeting_child(client, consts):
    # Make sure child targetable items are affected by targeted effects
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(-2)
    api_module.change_mod(add_tgts=[api_drone.id])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(1)
    api_module.change_mod(rm_tgts=[api_drone.id])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_affected_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
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
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_mid_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_mid_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mid_mod])
    eve_mid_item = client.mk_eve_item(
        attrs={eve_mid_attr.id: 3},
        eff_ids=[eve_mid_effect.id],
        defeff_id=eve_mid_effect.id)
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: -2})
    eve_ship_item = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.set_ship(type_id=eve_ship_item.id)
    api_tgt_item = api_fit2.set_ship(type_id=eve_tgt_item.id)
    api_mid_item = api_fit1.add_mod(type_id=eve_mid_item.id, state=consts.ApiState.active)
    api_mid_item.change_mod(add_tgts=[api_tgt_item.id])
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(1)
    api_src_item = api_fit1.add_rig(type_id=eve_src_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(2.5)
    api_src_item.remove()
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(1)


def test_unaffected_root(client, consts):
    # Character shouldn't be affected
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_char = client.mk_eve_item(attrs={eve_attr2.id: -2})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_char = api_fit2.set_char(type_id=eve_char.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_char.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_unaffected_child_via_root(client, consts):
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: -2})
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_unaffected_root_via_child(client, consts):
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_attr2.id: -2})
    eve_drone = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_tgts=[api_drone.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_unaffected_root_other_fit(client, consts):
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_attr2.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit3 = api_sol.create_fit()
    api_ship2 = api_fit2.set_ship(type_id=eve_ship.id)
    api_ship3 = api_fit3.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_tgts=[api_ship2.id])
    assert api_ship3.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_child_other_fit(client, consts):
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit3 = api_sol.create_fit()
    api_drone2 = api_fit2.add_drone(type_id=eve_drone.id)
    api_drone3 = api_fit3.add_drone(type_id=eve_drone.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_tgts=[api_drone2.id])
    assert api_drone3.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_unaffected_nontgt_domain_item(client, consts):
    # Targets shouldn't be affected by modifiers which do not have target domain
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_attr2.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_unaffected_nontgt_domain_ship(client, consts):
    # Targets shouldn't be affected by modifiers which do not have target domain
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship = client.mk_eve_item(attrs={eve_attr2.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_replace_target(client, consts):
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_ship1 = client.mk_eve_item(attrs={eve_attr2.id: -2})
    eve_ship2 = client.mk_eve_item(attrs={eve_attr2.id: -4})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_ship1 = api_fit2.set_ship(type_id=eve_ship1.id)
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_module.change_mod(add_tgts=[api_ship1.id])
    assert api_ship1.update().attrs[eve_attr2.id].dogma == approx(1)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship2.id)
    assert api_ship2.update().attrs[eve_attr2.id].dogma == approx(-4)
    api_module.change_mod(add_tgts=[api_ship2.id])
    assert api_ship2.update().attrs[eve_attr2.id].dogma == approx(-1)
