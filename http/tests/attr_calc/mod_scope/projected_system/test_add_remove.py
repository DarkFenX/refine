from pytest import approx


def test_project_unproject(client, consts):
    # Check that effects are applied/removed when projected effect is applied/unapplied
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_attr1.id,
        affectee_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_attr1.id: 5}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_item(attrs={eve_attr2.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(37.5)
    api_proj_effect.change_proj_effect(rm_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(7.5)


def test_affector_state_change(client, consts):
    # Check that effects are applied/removed when projected effect is enabled/disabled
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_attr1.id,
        affectee_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_attr1.id: 5}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_item(attrs={eve_attr2.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id, state=False)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(7.5)
    api_proj_effect.change_proj_effect(state=True)
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(37.5)
    api_proj_effect.change_proj_effect(state=False)
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(7.5)


def test_remove(client, consts):
    # Check that effects are removed when projected effect is removed
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_attr1.id,
        affectee_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_proj_effect = client.mk_eve_item(attrs={eve_attr1.id: 5}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_item(attrs={eve_attr2.id: 7.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship.id)
    api_proj_effect = api_sol.add_proj_effect(type_id=eve_proj_effect.id)
    api_proj_effect.change_proj_effect(add_tgts=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(37.5)
    api_proj_effect.remove()
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(7.5)
