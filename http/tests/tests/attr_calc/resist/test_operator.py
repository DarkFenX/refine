from tests import approx


def test_pre_assign(client, consts):
    # Assignment is not affected by resistance
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.pre_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr_id,
        mod_info=[eve_module_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 400},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500, eve_resist_attr_id: 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module_id, state=consts.ApiModuleState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(400)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.pre_assign
    assert api_mod.initial_val == approx(400)
    assert api_mod.resist_mult is None
    assert api_mod.applied_val == approx(400)


def test_pre_mul(client, consts):
    # Check that actual modification is resisted, and not multiplier multiplied by resistance
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.pre_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr_id,
        mod_info=[eve_module_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 0.4},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500, eve_resist_attr_id: 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module_id, state=consts.ApiModuleState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(380)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.pre_mul
    assert api_mod.initial_val == approx(0.4)
    assert api_mod.resist_mult == approx(0.4)
    assert api_mod.applied_val == approx(0.76)


def test_pre_div(client, consts):
    # Check that actual modification is resisted, and not divisor multiplied by resistance
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.pre_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr_id,
        mod_info=[eve_module_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 2.5},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500, eve_resist_attr_id: 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module_id, state=consts.ApiModuleState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(380)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.pre_div
    assert api_mod.initial_val == approx(2.5)
    assert api_mod.resist_mult == approx(0.4)
    assert api_mod.applied_val == approx(1.3157895)


def test_add(client, consts):
    # There are no partially resisted addition effects in EVE, but it seems sensible to resist those
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr_id,
        mod_info=[eve_module_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 100},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500, eve_resist_attr_id: 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module_id, state=consts.ApiModuleState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(540)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.mod_add
    assert api_mod.initial_val == approx(100)
    assert api_mod.resist_mult == approx(0.4)
    assert api_mod.applied_val == approx(40)


def test_sub(client, consts):
    # There are no partially resisted subtraction effects in EVE, but it seems sensible to resist
    # those
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.mod_sub,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr_id,
        mod_info=[eve_module_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 100},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500, eve_resist_attr_id: 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module_id, state=consts.ApiModuleState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(460)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.mod_sub
    assert api_mod.initial_val == approx(100)
    assert api_mod.resist_mult == approx(0.4)
    assert api_mod.applied_val == approx(40)


def test_post_mul(client, consts):
    # Check that actual modification is resisted, and not multiplier multiplied by resistance
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr_id,
        mod_info=[eve_module_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 0.4},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500, eve_resist_attr_id: 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module_id, state=consts.ApiModuleState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(380)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_mul
    assert api_mod.initial_val == approx(0.4)
    assert api_mod.resist_mult == approx(0.4)
    assert api_mod.applied_val == approx(0.76)


def test_post_div(client, consts):
    # Check that actual modification is resisted, and not divisor multiplied by resistance
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_div,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr_id,
        mod_info=[eve_module_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 2.5},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500, eve_resist_attr_id: 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module_id, state=consts.ApiModuleState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(380)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_div
    assert api_mod.initial_val == approx(2.5)
    assert api_mod.resist_mult == approx(0.4)
    assert api_mod.applied_val == approx(1.3157895)


def test_post_percent(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr_id,
        mod_info=[eve_module_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -60},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500, eve_resist_attr_id: 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module_id, state=consts.ApiModuleState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(380)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_percent
    assert api_mod.initial_val == approx(-60)
    assert api_mod.resist_mult == approx(0.4)
    assert api_mod.applied_val == approx(-24)


def test_post_assign(client, consts):
    # Assignment is not affected by resistance
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        resist_attr_id=eve_resist_attr_id,
        mod_info=[eve_module_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 400},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 500, eve_resist_attr_id: 0.4})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module_id, state=consts.ApiModuleState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    api_affectee_ship.update()
    assert api_affectee_ship.attrs[eve_affectee_attr_id].dogma == approx(400)
    api_mod = api_affectee_ship.mods[eve_affectee_attr_id].one()
    assert api_mod.op == consts.ApiModOp.post_assign
    assert api_mod.initial_val == approx(400)
    assert api_mod.resist_mult is None
    assert api_mod.applied_val == approx(400)
