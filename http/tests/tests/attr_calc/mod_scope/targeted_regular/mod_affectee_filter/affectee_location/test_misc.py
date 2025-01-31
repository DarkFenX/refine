from tests import approx


def test_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_affector_attr_id = client.mk_eve_attr()
    eve_middle_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_affector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_middle_attr_id)
    eve_affector_effect_id = client.mk_eve_effect(mod_info=[eve_affector_mod])
    eve_affector_item_id = client.mk_eve_item(attrs={eve_affector_attr_id: 50}, eff_ids=[eve_affector_effect_id])
    eve_middle_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_middle_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_middle_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_middle_mod])
    eve_middle_item_id = client.mk_eve_item(
        attrs={eve_middle_attr_id: 20},
        eff_ids=[eve_middle_effect_id],
        defeff_id=eve_middle_effect_id)
    eve_affectee_item_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 80})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.set_ship(type_id=eve_ship_id)
    api_ship = api_fit2.set_ship(type_id=eve_ship_id)
    api_affectee_item = api_fit2.add_rig(type_id=eve_affectee_item_id)
    api_middle_item = api_fit1.add_mod(type_id=eve_middle_item_id, state=consts.ApiState.active)
    api_middle_item.change_mod(add_projs=[api_ship.id])
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(96)
    api_affector_item = api_fit1.add_rig(type_id=eve_affector_item_id)
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(104)
    api_affector_item.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr_id].dogma == approx(96)


def test_replace_proj(client, consts):
    eve_attr1_id = client.mk_eve_attr()
    eve_attr2_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1_id,
        affectee_attr_id=eve_attr2_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(attrs={eve_attr1_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_rig_id = client.mk_eve_item(attrs={eve_attr2_id: 80})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module_id, state=consts.ApiState.active)
    api_ship1 = api_fit2.set_ship(type_id=eve_ship_id)
    api_rig = api_fit2.add_rig(type_id=eve_rig_id)
    api_module.change_mod(add_projs=[api_ship1.id])
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(96)
    api_ship1.remove()
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(80)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship_id)
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(80)
    api_module.change_mod(add_projs=[api_ship2.id])
    assert api_rig.update().attrs[eve_attr2_id].dogma == approx(96)
