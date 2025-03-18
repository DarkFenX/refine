from tests import approx


def test_target_untarget(client, consts):
    # When effect is active and not default, it shouldn't apply anything to target
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 3}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr2_id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    assert api_ship.update().attrs[eve_attr2_id].dogma == approx(-2)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2_id].dogma == approx(-2)
    api_module.change_module(rm_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2_id].dogma == approx(-2)


def test_affector_state_change(client, consts):
    # When effect is active and not default, it shouldn't apply anything to target
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 3}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr2_id: -2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_module.change_module(add_projs=[api_ship.id])
    assert api_ship.update().attrs[eve_attr2_id].dogma == approx(-2)
    api_module.change_module(state=consts.ApiModuleState.active)
    assert api_ship.update().attrs[eve_attr2_id].dogma == approx(-2)
    api_module.change_module(state=consts.ApiModuleState.online)
    assert api_ship.update().attrs[eve_attr2_id].dogma == approx(-2)
