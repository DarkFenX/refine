from pytest import approx


def test_target_untarget(client, consts):
    # When effect is active and not default, it shouldn't apply anything to target
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_attr1.id,
        affectee_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_ship(attrs={eve_attr2.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(-2)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(-2)
    api_module.change_mod(rm_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_affector_state_change(client, consts):
    # When effect is active and not default, it shouldn't apply anything to target
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_attr1.id,
        affectee_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 3}, eff_ids=[eve_effect.id])
    eve_ship = client.mk_eve_ship(attrs={eve_attr2.id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.online)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_module.change_mod(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(-2)
    api_module.change_mod(state=consts.ApiState.active)
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(-2)
    api_module.change_mod(state=consts.ApiState.online)
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(-2)
