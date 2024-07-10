from pytest import approx


def test_pre_assign(client, consts):
    # Assignment is not affected by range
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id,
        mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: 100, eve_optimal_attr.id: 1000, eve_falloff_attr.id: 10000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 11000)])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(100)
    api_affectee_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_affectee_mod.op == consts.ApiModOp.pre_assign
    assert api_affectee_mod.initial_val == approx(100)
    assert api_affectee_mod.range_mult is None
    assert api_affectee_mod.applied_val == approx(100)


def test_pre_mul(client, consts):
    # Check that actual modification is reduced, and not multiplier
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.pre_mul,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id,
        mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: 0.15, eve_optimal_attr.id: 1000, eve_falloff_attr.id: 10000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 11000)])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(287.5)
    api_affectee_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_affectee_mod.op == consts.ApiModOp.pre_mul
    assert api_affectee_mod.initial_val == approx(0.15)
    assert api_affectee_mod.range_mult == approx(0.5)
    assert api_affectee_mod.applied_val == approx(0.575)


def test_pre_div(client, consts):
    # Check that actual modification is resisted, and not divisor multiplied by resistance
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.pre_div,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id,
        mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: 6.66666666666667, eve_optimal_attr.id: 1000, eve_falloff_attr.id: 10000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 11000)])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(287.5)
    api_affectee_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_affectee_mod.op == consts.ApiModOp.pre_div
    assert api_affectee_mod.initial_val == approx(6.66666666666667)
    assert api_affectee_mod.range_mult == approx(0.5)
    assert api_affectee_mod.applied_val == approx(1.7391304)


def test_add(client, consts):
    # There are no projected addition effects with falloff in EVE, but it seems sensible to reduce
    # those
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id,
        mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -100, eve_optimal_attr.id: 1000, eve_falloff_attr.id: 10000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 11000)])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(450)
    api_affectee_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_affectee_mod.op == consts.ApiModOp.mod_add
    assert api_affectee_mod.initial_val == approx(-100)
    assert api_affectee_mod.range_mult == approx(0.5)
    assert api_affectee_mod.applied_val == approx(-50)


def test_sub(client, consts):
    # There are no projected subtraction effects with falloff in EVE, but it seems sensible to
    # reduce those
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.mod_sub,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id,
        mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -100, eve_optimal_attr.id: 1000, eve_falloff_attr.id: 10000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 11000)])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(550)
    api_affectee_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_affectee_mod.op == consts.ApiModOp.mod_sub
    assert api_affectee_mod.initial_val == approx(-100)
    assert api_affectee_mod.range_mult == approx(0.5)
    assert api_affectee_mod.applied_val == approx(-50)


def test_post_mul(client, consts):
    # Check that actual modification is reduced, and not multiplier
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id,
        mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: 0.15, eve_optimal_attr.id: 1000, eve_falloff_attr.id: 10000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 11000)])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(287.5)
    api_affectee_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_mul
    assert api_affectee_mod.initial_val == approx(0.15)
    assert api_affectee_mod.range_mult == approx(0.5)
    assert api_affectee_mod.applied_val == approx(0.575)


def test_post_div(client, consts):
    # Check that actual modification is resisted, and not divisor multiplied by resistance
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_div,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id,
        mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: 6.66666666666667, eve_optimal_attr.id: 1000, eve_falloff_attr.id: 10000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 11000)])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(287.5)
    api_affectee_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_div
    assert api_affectee_mod.initial_val == approx(6.66666666666667)
    assert api_affectee_mod.range_mult == approx(0.5)
    assert api_affectee_mod.applied_val == approx(1.7391304)


def test_post_percent(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id,
        mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -85, eve_optimal_attr.id: 1000, eve_falloff_attr.id: 10000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 11000)])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(287.5)
    api_affectee_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_percent
    assert api_affectee_mod.initial_val == approx(-85)
    assert api_affectee_mod.range_mult == approx(0.5)
    assert api_affectee_mod.applied_val == approx(-42.5)


def test_post_assign(client, consts):
    # Assignment is not affected by range
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id,
        mod_info=[eve_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: 100, eve_optimal_attr.id: 1000, eve_falloff_attr.id: 10000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 11000)])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr.id].dogma == approx(100)
    api_affectee_mod = api_affectee_ship.mods[eve_affectee_attr.id].one()
    assert api_affectee_mod.op == consts.ApiModOp.post_assign
    assert api_affectee_mod.initial_val == approx(100)
    assert api_affectee_mod.range_mult is None
    assert api_affectee_mod.applied_val == approx(100)
