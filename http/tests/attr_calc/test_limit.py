from tests import approx


def test_default_max(client, consts):
    # Check that max limit is applied properly when item doesn't have base value of limiter
    # attribute
    eve_limiter_attr_id = client.mk_eve_attr(def_val=5)
    eve_limitee_attr_id = client.mk_eve_attr(max_attr_id=eve_limiter_attr_id)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_limitee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(
        attrs={eve_limitee_attr_id: 3, eve_affector_attr_id: 6},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item_id)
    # Verification - should be 3 * 6 = 18 without limit, but 5 with
    api_item.update()
    assert api_item.attrs[eve_limitee_attr_id].dogma == approx(5)
    api_mod = api_item.mods.find_by_affector_attr(
        affectee_attr_id=eve_limitee_attr_id, affector_attr_id=eve_limiter_attr_id).one()
    assert api_mod.op == consts.ApiModOp.max_limit
    assert api_mod.initial_val == approx(5)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(5)
    assert api_mod.affectors.one().item_id == api_item.id
    assert api_mod.affectors.one().attr_id == eve_limiter_attr_id


def test_default_min(client, consts):
    # Check that min limit is applied properly when item doesn't have base value of limiter
    # attribute
    eve_limiter_attr_id = client.mk_eve_attr(def_val=20)
    eve_limitee_attr_id = client.mk_eve_attr(min_attr_id=eve_limiter_attr_id)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_limitee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(
        attrs={eve_limitee_attr_id: 3, eve_affector_attr_id: 6},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item_id)
    # Verification - should be 3 * 6 = 18 without limit, but 20 with
    api_item.update()
    assert api_item.attrs[eve_limitee_attr_id].dogma == approx(20)
    api_mod = api_item.mods.find_by_affector_attr(
        affectee_attr_id=eve_limitee_attr_id, affector_attr_id=eve_limiter_attr_id).one()
    assert api_mod.op == consts.ApiModOp.min_limit
    assert api_mod.initial_val == approx(20)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(20)
    assert api_mod.affectors.one().item_id == api_item.id
    assert api_mod.affectors.one().attr_id == eve_limiter_attr_id


def test_unmodified_max(client, consts):
    # Check that max limit is applied properly when item defines its value
    eve_limiter_attr_id = client.mk_eve_attr(def_val=5)
    eve_limitee_attr_id = client.mk_eve_attr(max_attr_id=eve_limiter_attr_id)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_limitee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(
        attrs={eve_limiter_attr_id: 2, eve_limitee_attr_id: 3, eve_affector_attr_id: 6},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item_id)
    # Verification - should be 3 * 6 = 18 without limit, but 2 with
    api_item.update()
    assert api_item.attrs[eve_limitee_attr_id].dogma == approx(2)
    api_mod = api_item.mods.find_by_affector_attr(
        affectee_attr_id=eve_limitee_attr_id, affector_attr_id=eve_limiter_attr_id).one()
    assert api_mod.op == consts.ApiModOp.max_limit
    assert api_mod.initial_val == approx(2)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(2)
    assert api_mod.affectors.one().item_id == api_item.id
    assert api_mod.affectors.one().attr_id == eve_limiter_attr_id


def test_unmodified_min(client, consts):
    # Check that min limit is applied properly when item defines its value
    eve_limiter_attr_id = client.mk_eve_attr(def_val=20)
    eve_limitee_attr_id = client.mk_eve_attr(min_attr_id=eve_limiter_attr_id)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_limitee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(
        attrs={eve_limiter_attr_id: 25, eve_limitee_attr_id: 3, eve_affector_attr_id: 6},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item_id)
    # Verification - should be 3 * 6 = 18 without limit, but 25 with
    api_item.update()
    assert api_item.attrs[eve_limitee_attr_id].dogma == approx(25)
    api_mod = api_item.mods.find_by_affector_attr(
        affectee_attr_id=eve_limitee_attr_id, affector_attr_id=eve_limiter_attr_id).one()
    assert api_mod.op == consts.ApiModOp.min_limit
    assert api_mod.initial_val == approx(25)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(25)
    assert api_mod.affectors.one().item_id == api_item.id
    assert api_mod.affectors.one().attr_id == eve_limiter_attr_id


def test_modified(client, consts):
    # Check that limit is applied properly when item defines its value, and it's modified further
    eve_limiter_attr_id = client.mk_eve_attr()
    eve_limitee_attr_id = client.mk_eve_attr(max_attr_id=eve_limiter_attr_id)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_limitee_attr_id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_limiter_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod1, eve_mod2])
    eve_item_id = client.mk_eve_item(
        attrs={eve_limiter_attr_id: 0.1, eve_limitee_attr_id: 3, eve_affector_attr_id: 6},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item_id)
    # Verification - should be 3 * 6 = 18 without limit, but 0.1 * 6 = 0.6 with
    api_item.update()
    assert api_item.attrs[eve_limitee_attr_id].dogma == approx(0.6)
    api_mod = api_item.mods.find_by_affector_attr(
        affectee_attr_id=eve_limitee_attr_id, affector_attr_id=eve_limiter_attr_id).one()
    assert api_mod.op == consts.ApiModOp.max_limit
    assert api_mod.initial_val == approx(0.6)
    assert api_mod.stacking_mult is None
    assert api_mod.applied_val == approx(0.6)
    assert api_mod.affectors.one().item_id == api_item.id
    assert api_mod.affectors.one().attr_id == eve_limiter_attr_id


def test_update(client, consts):
    # Make sure that when value of limiter attribute changes, values which depend on it are updated
    eve_limiter_attr_id = client.mk_eve_attr()
    eve_limitee_attr_id = client.mk_eve_attr(max_attr_id=eve_limiter_attr_id)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_limitee_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_limitee_attr_id)
    eve_limitee_effect_id = client.mk_eve_effect(mod_info=[eve_limitee_mod])
    eve_limitee_item_id = client.mk_eve_item(
        attrs={eve_limiter_attr_id: 2, eve_limitee_attr_id: 3, eve_affector_attr_id: 6},
        eff_ids=[eve_limitee_effect_id])
    eve_limiter_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_limiter_attr_id)
    eve_limiter_effect_id = client.mk_eve_effect(mod_info=[eve_limiter_mod])
    eve_limiter_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 3.5}, eff_ids=[eve_limiter_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_limitee_item = api_fit.add_rig(type_id=eve_limitee_item_id)
    # Verification - request limitee attribute value before adding limiter item, to make sure
    # limitee attribute value is calculated
    api_limitee_item.update()
    assert api_limitee_item.attrs[eve_limitee_attr_id].dogma == approx(2)
    assert api_limitee_item.mods.find_by_affector_attr(
        affectee_attr_id=eve_limitee_attr_id, affector_attr_id=eve_limiter_attr_id).one().applied_val == approx(2)
    # Action
    api_limiter_item = api_fit.add_implant(type_id=eve_limiter_item_id)
    # Verification - here, limiter attribute should be multiplied by 3.5 (2 * 3.5 = 7), which is
    # still below unlimited value of limitee attribute (18)
    api_limitee_item.update()
    assert api_limitee_item.attrs[eve_limitee_attr_id].dogma == approx(7)
    assert api_limitee_item.mods.find_by_affector_attr(
        affectee_attr_id=eve_limitee_attr_id, affector_attr_id=eve_limiter_attr_id).one().applied_val == approx(7)
    # Action
    api_limiter_item.remove()
    # Verification - should revert back to base value after change of limiter attribute
    api_limitee_item.update()
    assert api_limitee_item.attrs[eve_limitee_attr_id].dogma == approx(2)
    assert api_limitee_item.mods.find_by_affector_attr(
        affectee_attr_id=eve_limitee_attr_id, affector_attr_id=eve_limiter_attr_id).one().applied_val == approx(2)


def test_unlimited(client, consts):
    # Check which modification data is exposed when calculated value is within limit
    eve_limiter_attr_id = client.mk_eve_attr(def_val=5)
    eve_limitee_attr_id = client.mk_eve_attr(max_attr_id=eve_limiter_attr_id)
    eve_affector_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_limitee_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(
        attrs={eve_limiter_attr_id: 20, eve_limitee_attr_id: 3, eve_affector_attr_id: 6},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item_id)
    # Verification - should be 3 * 6 = 18 without max limit, and max limit is higher - 20, so 18 is
    # exposed
    api_item.update()
    assert api_item.attrs[eve_limitee_attr_id].dogma == approx(18)
    assert len(api_item.mods.find_by_affector_attr(
        affectee_attr_id=eve_limitee_attr_id, affector_attr_id=eve_limiter_attr_id)) == 0


def test_src_switch(client):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_limiter_attr_id = eve_d1.mk_attr(def_val=5).id
    eve_limitee_attr_id = eve_d1.mk_attr(max_attr_id=eve_limiter_attr_id).id
    eve_d2.mk_attr(id_=eve_limiter_attr_id, def_val=5)
    eve_d2.mk_attr(id_=eve_limitee_attr_id)
    eve_item_id = eve_d1.mk_item(attrs={eve_limiter_attr_id: 2, eve_limitee_attr_id: 3}).id
    eve_d2.mk_item(attrs={eve_limiter_attr_id: 2, eve_limitee_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item_id)
    # Verification - limited on first source, because max attr ID is defined
    api_item.update()
    assert api_item.attrs[eve_limitee_attr_id].dogma == approx(2)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification - not limited on second source, since it doesn't specify max attr ID
    api_item.update()
    assert api_item.attrs[eve_limitee_attr_id].dogma == approx(3)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification - switch back and check that it's limited again
    api_item.update()
    assert api_item.attrs[eve_limitee_attr_id].dogma == approx(2)
