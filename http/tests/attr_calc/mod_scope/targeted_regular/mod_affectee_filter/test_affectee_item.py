from pytest import approx


def test_affected_root_ship(client, consts):
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
    api_module.change_mod(add_tgts=[(api_ship.id, None)])
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(1)
    api_module.remove()
    assert api_ship.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_affected_root_struct(client, consts):
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
    api_module.change_mod(add_tgts=[(api_struct.id, None)])
    assert api_struct.update().attrs[eve_attr2.id].dogma == approx(1)
    api_module.remove()
    assert api_struct.update().attrs[eve_attr2.id].dogma == approx(-2)


def test_affected_child(client, consts):
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
    api_module.change_mod(add_tgts=[(api_drone.id, None)])
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(1)
    api_module.remove()
    assert api_drone.update().attrs[eve_attr2.id].dogma == approx(-2)
