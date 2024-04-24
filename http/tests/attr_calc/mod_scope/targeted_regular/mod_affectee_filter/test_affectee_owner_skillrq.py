from pytest import approx


def test_affected_state_change_via_ship(client, consts):
    # There are seem to be no such effects defined in EVE, but if my memory serves me properly, this
    # is how guidance disruptors used to work during dgmexpressions table days. Later that table was
    # removed and the effect was reworked to have strength reduction in falloff, and all associated
    # modifier info was removed. We support this scenario nevertheless.
    eve_skill = client.mk_eve_item()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.tgt,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: 80}, srqs={eve_skill.id: 1})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.online)
    api_fit2.set_char(type_id=eve_char.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module.change_mod(add_tgts=[(api_ship.id, None)])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)
    api_module.change_mod(state=consts.ApiState.active)
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(96)
    api_module.change_mod(state=consts.ApiState.online)
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)


def test_affected_state_change_via_struct(client, consts):
    eve_skill = client.mk_eve_item()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.tgt,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: 80}, srqs={eve_skill.id: 1})
    eve_char = client.mk_eve_item()
    eve_struct = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.online)
    api_fit2.set_char(type_id=eve_char.id)
    api_struct = api_fit2.set_struct(type_id=eve_struct.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module.change_mod(add_tgts=[(api_struct.id, None)])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)
    api_module.change_mod(state=consts.ApiState.active)
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(96)
    api_module.change_mod(state=consts.ApiState.online)
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)


def test_affected_targeting_via_ship(client, consts):
    eve_skill = client.mk_eve_item()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.tgt,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: 80}, srqs={eve_skill.id: 1})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_char(type_id=eve_char.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)
    api_module.change_mod(add_tgts=[(api_ship.id, None)])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(96)
    api_module.change_mod(rm_tgts=[api_ship.id])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)


def test_affected_targeting_via_struct(client, consts):
    eve_skill = client.mk_eve_item()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.tgt,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: 80}, srqs={eve_skill.id: 1})
    eve_char = client.mk_eve_item()
    eve_struct = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_char(type_id=eve_char.id)
    api_struct = api_fit2.set_struct(type_id=eve_struct.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)
    api_module.change_mod(add_tgts=[(api_struct.id, None)])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(96)
    api_module.change_mod(rm_tgts=[api_struct.id])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)


def test_affected_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_skill = client.mk_eve_item()
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
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.tgt,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_mid_attr.id,
        tgt_attr_id=eve_tgt_attr.id)
    eve_mid_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mid_mod])
    eve_mid_item = client.mk_eve_item(
        attrs={eve_mid_attr.id: 20},
        eff_ids=[eve_mid_effect.id],
        defeff_id=eve_mid_effect.id)
    eve_tgt_item = client.mk_eve_item(attrs={eve_tgt_attr.id: 80}, srqs={eve_skill.id: 1})
    eve_ship_item = client.mk_eve_item()
    eve_char_item = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.set_ship(type_id=eve_ship_item.id)
    api_fit2.set_char(type_id=eve_char_item.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_item.id)
    api_tgt_item = api_fit2.add_drone(type_id=eve_tgt_item.id)
    api_mid_item = api_fit1.add_mod(type_id=eve_mid_item.id, state=consts.ApiState.active)
    api_mid_item.change_mod(add_tgts=[(api_ship.id, None)])
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(96)
    api_src_item = api_fit1.add_rig(type_id=eve_src_item.id)
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(104)
    api_src_item.remove()
    assert api_tgt_item.update().attrs[eve_tgt_attr.id].dogma == approx(96)


def test_unaffected_non_owner_modifiable(client, consts):
    # Check that items which are not marked as owner-modifiable do not receive modification
    eve_skill = client.mk_eve_item()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.tgt,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(attrs={eve_attr2.id: 80}, srqs={eve_skill.id: 1})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_char(type_id=eve_char.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    api_module.change_mod(add_tgts=[(api_ship.id, None)])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)


def test_unaffected_other_skillreq(client, consts):
    # Check that entities which don't have needed skill requirement are not affected
    eve_skill1 = client.mk_eve_item()
    eve_skill2 = client.mk_eve_item()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.tgt,
        srq=eve_skill1.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: 80}, srqs={eve_skill2.id: 1})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_char(type_id=eve_char.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module.change_mod(add_tgts=[(api_ship.id, None)])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)


def test_unaffected_targeted_child(client, consts):
    # When it's not ship/structure which is getting targeted, target item shouldn't be affected
    eve_skill = client.mk_eve_item()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.tgt,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: 80}, srqs={eve_skill.id: 1})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_char(type_id=eve_char.id)
    api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module.change_mod(add_tgts=[(api_drone.id, None)])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)


def test_unaffected_other_fit(client, consts):
    # Check that targeted modifications are not carried over to another fit
    eve_skill = client.mk_eve_item()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.tgt,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: 80}, srqs={eve_skill.id: 1})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit3 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_char(type_id=eve_char.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_fit2.add_drone(type_id=eve_drone.id)
    api_fit3.set_char(type_id=eve_char.id)
    api_fit3.set_ship(type_id=eve_ship.id)
    api_drone3 = api_fit3.add_drone(type_id=eve_drone.id)
    api_module.change_mod(add_tgts=[(api_ship.id, None)])
    assert api_drone3.update().attrs[eve_attr2.id].dogma == approx(80)


def test_unaffected_nontgt_domain_item(client, consts):
    # Targets shouldn't be affected by modifiers which do not have target domain
    eve_skill = client.mk_eve_item()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.item,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: 80}, srqs={eve_skill.id: 1})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_char(type_id=eve_char.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module.change_mod(add_tgts=[(api_ship.id, None)])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)


def test_unaffected_nontgt_domain_ship(client, consts):
    # Targets shouldn't be affected by modifiers which do not have target domain
    eve_skill = client.mk_eve_item()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.ship,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: 80}, srqs={eve_skill.id: 1})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_char(type_id=eve_char.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module.change_mod(add_tgts=[(api_ship.id, None)])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)


def test_replace_root(client, consts):
    # This behavior isn't defined in EVE, but we check how character presence influences
    # modifications with owner-skillreq filter. In our case it doesn't, because those are tracked
    # by fit ID
    eve_skill = client.mk_eve_item()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.tgt,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: 80}, srqs={eve_skill.id: 1})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_char = api_fit2.set_char(type_id=eve_char.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module.change_mod(add_tgts=[(api_ship.id, None)])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(96)
    api_char.remove()
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(96)
    api_fit2.set_char(type_id=eve_char.id)
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(96)


def test_replace_target(client, consts):
    eve_skill = client.mk_eve_item()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        dom=consts.EveModDom.tgt,
        srq=eve_skill.id,
        op=consts.EveModOp.post_percent,
        src_attr_id=eve_attr1.id,
        tgt_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_drone = client.mk_eve_item(attrs={eve_attr2.id: 80}, srqs={eve_skill.id: 1})
    eve_char = client.mk_eve_item()
    eve_ship = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_fit2.set_char(type_id=eve_char.id)
    api_ship1 = api_fit2.set_ship(type_id=eve_ship.id)
    api_drone = api_fit2.add_drone(type_id=eve_drone.id)
    api_module.change_mod(add_tgts=[(api_ship1.id, None)])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(96)
    api_ship1.remove()
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship.id)
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(80)
    api_module.change_mod(add_tgts=[(api_ship2.id, None)])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(96)
