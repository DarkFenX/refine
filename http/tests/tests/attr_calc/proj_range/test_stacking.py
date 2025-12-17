from fw import approx


def test_stacking(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr(stackable=False)
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module1_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -80, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affector_module2_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -30, eve_optimal_attr_id: 12000, eve_falloff_attr_id: 2000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit1 = api_sol.create_fit()
    api_affector_fit1.set_ship(type_id=eve_ship_id, coordinates=(18000, 0, 0))
    api_affector_module1 = api_affector_fit1.add_module(
        type_id=eve_affector_module1_id,
        state=consts.ApiModuleState.active)
    api_affector_fit2 = api_sol.create_fit()
    api_affector_fit2.set_ship(type_id=eve_ship_id, coordinates=(13000, 0, 0))
    api_affector_module2 = api_affector_fit2.add_module(
        type_id=eve_affector_module2_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_module1.change_module(add_projs=[api_affectee_ship.id])
    api_affector_module2.change_module(add_projs=[api_affectee_ship.id])
    # Second module has stronger effect after range factored in, and thus is penalized less. If it
    # was the other way around, the value would've been ~348.2
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(338.797742)
    api_mods = api_affectee_ship.mods[eve_affectee_attr_id]
    assert len(api_mods) == 2
    api_module1_mod = api_mods.find_by_affector_item(affector_item_id=api_affector_module1.id).one()
    assert api_module1_mod.op == consts.ApiModOp.post_percent
    assert api_module1_mod.initial_val == approx(-80)
    assert api_module1_mod.range_mult == approx(0.1349035)
    assert api_module1_mod.stacking_mult == approx(consts.PenaltyStr.p2)
    assert api_module1_mod.applied_val == approx(-9.3797882)
    api_module2_mod = api_mods.find_by_affector_item(affector_item_id=api_affector_module2.id).one()
    assert api_module2_mod.op == consts.ApiModOp.post_percent
    assert api_module2_mod.initial_val == approx(-30)
    assert api_module2_mod.range_mult == approx(0.8408964)
    assert api_module2_mod.stacking_mult == approx(consts.PenaltyStr.p1)
    assert api_module2_mod.applied_val == approx(-25.2268925)
