from pytest import approx


def test_affected(client, consts):
    # There are no such effects in EVE, but we still assume that projected effect should be able to
    # affect itself
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(
        attrs={eve_affector_attr.id: 20, eve_affectee_attr.id: 100},
        eff_ids=[eve_effect.id])
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.create_fit()
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id, state=False)
    assert api_proj_effect.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    api_proj_effect.change_proj_effect(state=True)
    assert api_proj_effect.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    api_proj_effect.change_proj_effect(state=False)
    assert api_proj_effect.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_root(client, consts):
    # Check that direct item modification from projected effect doesn't affect fit top-level
    # entities
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_child(client, consts):
    # Check that direct item modification from projected effect doesn't affect fit child entities
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_rig = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_rig = api_fit.add_rig(type_id=eve_rig.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_projs=[api_ship.id])
    assert api_rig.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_unaffected_other_proj_effect(client, consts):
    # Check that self-modification does not touch other projected effects
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect_affector = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id])
    eve_proj_effect_affectee = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.create_fit()
    api_sol.add_proj_effect(type_id=eve_proj_effect_affector.id)
    api_proj_effect_affectee = api_sol.add_proj_effect(type_id=eve_proj_effect_affectee.id)
    assert api_proj_effect_affectee.update().attrs[eve_affectee_attr.id].dogma == approx(100)
