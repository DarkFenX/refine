from tests import approx


def test_propagation(client, consts):
    # Check that changes to attribute value which is source of modification are propagated to target
    eve_grp = client.mk_eve_item_group()
    eve_affector_attr = client.mk_eve_attr()
    eve_middle_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_affector_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_middle_attr.id)
    eve_affector_effect = client.mk_eve_effect(mod_info=[eve_affector_mod])
    eve_affector_item = client.mk_eve_item(attrs={eve_affector_attr.id: 50}, eff_ids=[eve_affector_effect.id])
    eve_middle_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.tgt,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_middle_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_middle_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_middle_mod])
    eve_middle_item = client.mk_eve_item(
        attrs={eve_middle_attr.id: 20},
        eff_ids=[eve_middle_effect.id],
        defeff_id=eve_middle_effect.id)
    eve_affectee_item = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_affectee_attr.id: 80})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_fit1.set_ship(type_id=eve_ship.id)
    api_ship = api_fit2.set_ship(type_id=eve_ship.id)
    api_affectee_item = api_fit2.add_rig(type_id=eve_affectee_item.id)
    api_middle_item = api_fit1.add_mod(type_id=eve_middle_item.id, state=consts.ApiState.active)
    api_middle_item.change_mod(add_projs=[api_ship.id])
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(96)
    api_affector_item = api_fit1.add_rig(type_id=eve_affector_item.id)
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(104)
    api_affector_item.remove()
    assert api_affectee_item.update().attrs[eve_affectee_attr.id].dogma == approx(96)


def test_replace_target(client, consts):
    eve_grp = client.mk_eve_item_group()
    eve_attr1 = client.mk_eve_attr()
    eve_attr2 = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        dom=consts.EveModDom.tgt,
        grp=eve_grp.id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_attr1.id,
        affectee_attr_id=eve_attr2.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item(attrs={eve_attr1.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_rig = client.mk_eve_item(grp_id=eve_grp.id, attrs={eve_attr2.id: 80})
    eve_ship = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    api_module = api_fit1.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_ship1 = api_fit2.set_ship(type_id=eve_ship.id)
    api_rig = api_fit2.add_rig(type_id=eve_rig.id)
    api_module.change_mod(add_projs=[api_ship1.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(96)
    api_ship1.remove()
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)
    api_ship2 = api_fit2.set_ship(type_id=eve_ship.id)
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(80)
    api_module.change_mod(add_projs=[api_ship2.id])
    assert api_rig.update().attrs[eve_attr2.id].dogma == approx(96)
