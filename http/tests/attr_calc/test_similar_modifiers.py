from tests import approx


def test_same_item_different_effects_attrs(client, consts):
    # Reflects currently real EVE scenario: 2 different skills affect 2 separate attributes on
    # capital ships, which, in turn, affect ship agility via 2 different on-ship effects
    eve_affector_attr1 = client.mk_eve_attr()
    eve_affector_attr2 = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr1.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect1 = client.mk_eve_effect(mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr2.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect2 = client.mk_eve_effect(mod_info=[eve_mod2])
    eve_ship = client.mk_eve_ship(
        attrs={eve_affector_attr1.id: 20, eve_affector_attr2.id: 20, eve_affectee_attr.id: 100},
        eff_ids=[eve_effect1.id, eve_effect2.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.set_ship(type_id=eve_ship.id)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr.id].dogma == approx(144)
    assert len(api_item.mods[eve_affectee_attr.id]) == 2
    api_mod1 = api_item.mods[eve_affectee_attr.id].find_by_affector_attr(affector_attr_id=eve_affector_attr1.id).one()
    assert api_mod1.op == consts.ApiModOp.post_percent
    assert api_mod1.initial_val == approx(20)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(20)
    api_mod2 = api_item.mods[eve_affectee_attr.id].find_by_affector_attr(affector_attr_id=eve_affector_attr2.id).one()
    assert api_mod2.op == consts.ApiModOp.post_percent
    assert api_mod2.initial_val == approx(20)
    assert api_mod2.stacking_mult is None
    assert api_mod2.applied_val == approx(20)


def test_same_item_different_effects_attrs_switching(client, consts):
    # This case is theoretical. We just check that our calculation core properly registers/
    # unregisters affectors, ignoring modifier keys which are needed for some tests in this module
    # to work. If it would not register properly and used the same keys as logic above it, after
    # disabling one of effects attr value would revert to its base value
    eve_affector_attr1 = client.mk_eve_attr()
    eve_affector_attr2 = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr1.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect1 = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr2.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect2 = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod2])
    eve_item = client.mk_eve_item(
        attrs={eve_affector_attr1.id: 20, eve_affector_attr2.id: 20, eve_affectee_attr.id: 100},
        eff_ids=[eve_effect1.id, eve_effect2.id],
        defeff_id=eve_effect2.id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_mod(type_id=eve_item.id, state=consts.ApiState.offline)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr.id].dogma == approx(120)
    api_mod = api_item.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(20)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(20)
    assert api_mod.affectors.one().attr_id == eve_affector_attr1.id
    # Action
    api_item.change_mod(state=consts.ApiState.active)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr.id].dogma == approx(144)
    assert len(api_item.mods[eve_affectee_attr.id]) == 2
    api_mod1 = api_item.mods[eve_affectee_attr.id].find_by_affector_attr(affector_attr_id=eve_affector_attr1.id).one()
    assert api_mod1.op == consts.ApiModOp.post_percent
    assert api_mod1.initial_val == approx(20)
    assert api_mod1.stacking_mult is None
    assert api_mod1.applied_val == approx(20)
    api_mod2 = api_item.mods[eve_affectee_attr.id].find_by_affector_attr(affector_attr_id=eve_affector_attr2.id).one()
    assert api_mod2.op == consts.ApiModOp.post_percent
    assert api_mod2.initial_val == approx(20)
    assert api_mod2.stacking_mult is None
    assert api_mod2.applied_val == approx(20)
    # Action
    api_item.change_mod(state=consts.ApiState.offline)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr.id].dogma == approx(120)
    api_mod = api_item.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(20)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(20)
    assert api_mod.affectors.one().attr_id == eve_affector_attr1.id


def test_same_item_attr_different_effects(client, consts):
    # Reflects currently real EVE scenario: capital hull repair systems have both capital and
    # sub-capital repair systems skills in their direct requirements. There are some items which
    # affect both (e.g. nanobot accelerator rigs). Despite having two different effects,
    # modification is applied only once in game
    eve_skill1 = client.mk_eve_item()
    eve_skill2 = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr(stackable=True)
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        dom=consts.EveModDom.ship,
        srq=eve_skill1.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect1 = client.mk_eve_effect(mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_srq,
        dom=consts.EveModDom.ship,
        srq=eve_skill2.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect2 = client.mk_eve_effect(mod_info=[eve_mod2])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect1.id, eve_effect2.id])
    eve_affectee_item = client.mk_eve_item(attrs={eve_affectee_attr.id: 100}, srqs={eve_skill1.id: 1, eve_skill2.id: 1})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship.id)
    api_fit.add_rig(type_id=eve_affector_item.id)
    api_item = api_fit.add_mod(type_id=eve_affectee_item.id, rack=consts.ApiRack.mid)
    # Verification
    api_item.update()
    assert api_item.attrs[eve_affectee_attr.id].dogma == approx(120)
    api_mod = api_item.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(20)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(20)
    assert api_mod.affectors.one().attr_id == eve_affector_attr.id


def test_same_item_attr_different_effects_switch(client, consts):
    # This case is theoretical. We just check that our calculation core properly registers/
    # unregisters affectors, ignoring modifier keys which are needed for some tests in this module
    # to work. If it would not register properly and used the same keys as logic above it, after
    # disabling one of effects attr value would revert to its base value
    eve_skill1 = client.mk_eve_item()
    eve_skill2 = client.mk_eve_item()
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr(stackable=True)
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect1 = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect2 = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod2])
    eve_affector_item = client.mk_eve_item(
        attrs={eve_affector_attr.id: 20},
        eff_ids=[eve_effect1.id, eve_effect2.id],
        defeff_id=eve_effect2.id)
    eve_affectee_item = client.mk_eve_ship(attrs={eve_affectee_attr.id: 100}, srqs={eve_skill1.id: 1, eve_skill2.id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_affector_item = api_fit.add_mod(type_id=eve_affector_item.id, state=consts.ApiState.offline)
    api_affectee_item = api_fit.set_ship(type_id=eve_affectee_item.id)
    # Verification
    api_affectee_item.update()
    assert api_affectee_item.attrs[eve_affectee_attr.id].dogma == approx(120)
    api_mod = api_affectee_item.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(20)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(20)
    assert api_mod.affectors.one().attr_id == eve_affector_attr.id
    # Action
    api_affector_item.change_mod(state=consts.ApiState.active)
    # Verification
    api_affectee_item.update()
    assert api_affectee_item.attrs[eve_affectee_attr.id].dogma == approx(120)
    api_mod = api_affectee_item.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(20)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(20)
    assert api_mod.affectors.one().attr_id == eve_affector_attr.id
    # Action
    api_affector_item.change_mod(state=consts.ApiState.offline)
    # Verification
    api_affectee_item.update()
    assert api_affectee_item.attrs[eve_affectee_attr.id].dogma == approx(120)
    api_mod = api_affectee_item.mods[eve_affectee_attr.id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(20)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(20)
    assert api_mod.affectors.one().attr_id == eve_affector_attr.id
