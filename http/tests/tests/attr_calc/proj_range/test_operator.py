from tests import approx, check_no_field


def test_pre_assign(client, consts):
    # Assignment is not affected by range until mult is 0
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 100, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 500},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(100)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.pre_assign
    assert api_mod.initial_val == approx(100)
    assert api_mod.range_mult == approx(1.0)
    assert api_mod.applied_val == approx(100)
    # Action
    api_affectee_ship.change_ship(coordinates=(1500, 0, 0))
    # Verification - value is not getting reduced despite target being in falloff
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(100)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.pre_assign
    assert api_mod.initial_val == approx(100)
    assert api_mod.range_mult == approx(1.0)
    assert api_mod.applied_val == approx(100)
    # Action
    api_affectee_ship.change_ship(coordinates=(300000, 0, 0))
    # Verification - when multiplier reaches 0, assignment is not applied altogether
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(500)
    with check_no_field():
        api_affectee_ship.mods  # noqa: B018


def test_pre_mul(client, consts):
    # Check that actual modification is reduced, and not multiplier
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.pre_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 0.15, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id, coordinates=(11000, 0, 0))
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(287.5)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.pre_mul
    assert api_mod.initial_val == approx(0.15)
    assert api_mod.range_mult == approx(0.5)
    assert api_mod.applied_val == approx(0.575)


def test_pre_div(client, consts):
    # Check that actual modification is resisted, and not divisor multiplied by resistance
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.pre_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 6.66666666666667, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id, coordinates=(11000, 0, 0))
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(287.5)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.pre_div
    assert api_mod.initial_val == approx(6.66666666666667)
    assert api_mod.range_mult == approx(0.5)
    assert api_mod.applied_val == approx(1.7391304)


def test_add(client, consts):
    # There are no projected addition effects with falloff in EVE, but it seems sensible to reduce
    # those
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -100, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id, coordinates=(11000, 0, 0))
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(450)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.mod_add
    assert api_mod.initial_val == approx(-100)
    assert api_mod.range_mult == approx(0.5)
    assert api_mod.applied_val == approx(-50)


def test_sub(client, consts):
    # There are no projected subtraction effects with falloff in EVE, but it seems sensible to
    # reduce those
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_sub,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -100, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id, coordinates=(11000, 0, 0))
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(550)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.mod_sub
    assert api_mod.initial_val == approx(-100)
    assert api_mod.range_mult == approx(0.5)
    assert api_mod.applied_val == approx(-50)


def test_post_mul(client, consts):
    # Check that actual modification is reduced, and not multiplier
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 0.15, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id, coordinates=(11000, 0, 0))
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(287.5)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(0.15)
    assert api_mod.range_mult == approx(0.5)
    assert api_mod.applied_val == approx(0.575)


def test_post_div(client, consts):
    # Check that actual modification is resisted, and not divisor multiplied by resistance
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 6.66666666666667, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id, coordinates=(11000, 0, 0))
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(287.5)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_div
    assert api_mod.initial_val == approx(6.66666666666667)
    assert api_mod.range_mult == approx(0.5)
    assert api_mod.applied_val == approx(1.7391304)


def test_post_percent(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
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
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -85, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id, coordinates=(11000, 0, 0))
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(287.5)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-85)
    assert api_mod.range_mult == approx(0.5)
    assert api_mod.applied_val == approx(-42.5)


def test_post_assign(client, consts):
    # Assignment is not affected by range until mult is 0
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 100, eve_optimal_attr_id: 1000, eve_falloff_attr_id: 500},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(100)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_assign
    assert api_mod.initial_val == approx(100)
    assert api_mod.range_mult == approx(1.0)
    assert api_mod.applied_val == approx(100)
    # Action
    api_affectee_ship.change_ship(coordinates=(1500, 0, 0))
    # Verification - value is not getting reduced despite target being in falloff
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(100)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_assign
    assert api_mod.initial_val == approx(100)
    assert api_mod.range_mult == approx(1.0)
    assert api_mod.applied_val == approx(100)
    # Action
    api_affectee_ship.change_ship(coordinates=(300000, 0, 0))
    # Verification - when multiplier reaches 0, assignment is not applied altogether
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(500)
    with check_no_field():
        api_affectee_ship.mods  # noqa: B018
