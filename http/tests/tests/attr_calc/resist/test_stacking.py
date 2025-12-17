from fw import approx


def test_stacking(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=False)
    eve_resist_attr1_id = client.mk_eve_attr()
    eve_resist_attr2_id = client.mk_eve_attr()
    eve_module1_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module1_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr1_id,
        mod_info=[eve_module1_mod])
    eve_affector_module1_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -80},
        eff_ids=[eve_module1_effect_id],
        defeff_id=eve_module1_effect_id)
    eve_module2_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module2_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr2_id,
        mod_info=[eve_module2_mod])
    eve_affector_module2_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -30},
        eff_ids=[eve_module2_effect_id],
        defeff_id=eve_module2_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(
        attrs={eve_affectee_attr_id: 500, eve_resist_attr1_id: 0.15, eve_resist_attr2_id: 0.7})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module1 = api_affector_fit.add_module(
        type_id=eve_affector_module1_id,
        state=consts.ApiModuleState.active)
    api_affector_module1.change_module(add_projs=[api_affectee_ship.id])
    api_affector_module2 = api_affector_fit.add_module(
        type_id=eve_affector_module2_id,
        state=consts.ApiModuleState.active)
    api_affector_module2.change_module(add_projs=[api_affectee_ship.id])
    # Second module has stronger effect after resistance, and thus is penalized less. If it was the
    # other way around, the value would've been ~359.7
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(353.803713)
    api_mods = api_affectee_ship.mods[eve_affectee_attr_id]
    assert len(api_mods) == 2
    api_module1_mod = api_mods.find_by_affector_item(affector_item_id=api_affector_module1.id).one()
    assert api_module1_mod.op == consts.ApiModOp.post_percent
    assert api_module1_mod.initial_val == approx(-80)
    assert api_module1_mod.resist_mult == approx(0.15)
    assert api_module1_mod.stacking_mult == approx(consts.PenaltyStr.p2)
    assert api_module1_mod.applied_val == approx(-10.4294398)
    api_module2_mod = api_mods.find_by_affector_item(affector_item_id=api_affector_module2.id).one()
    assert api_module2_mod.op == consts.ApiModOp.post_percent
    assert api_module2_mod.initial_val == approx(-30)
    assert api_module2_mod.resist_mult == approx(0.7)
    assert api_module2_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_module2_mod.applied_val == approx(-21)
