from pytest import approx


def test_add_with_change_without_remove(client, consts):
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
        attrs={eve_affector_attr.id: -60, eve_optimal_attr.id: 10000, eve_falloff_attr.id: 5000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_struct = client.mk_eve_struct(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_struct = api_affectee_fit.set_ship(type_id=eve_affectee_struct.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(500)
    api_affector_module.change_mod(add_projs=[(api_affectee_struct.id, 15000)])
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(350)
    api_affector_module.change_mod(change_projs=[(api_affectee_struct.id, None)])
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_module.change_mod(rm_projs=[api_affectee_struct.id])
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(500)


def test_add_without_change_with_remove(client, consts):
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
        attrs={eve_affector_attr.id: -60, eve_optimal_attr.id: 10000, eve_falloff_attr.id: 5000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(500)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, None)])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(200)
    api_affector_module.change_mod(change_projs=[(api_affectee_ship.id, 15000)])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(350)
    api_affector_module.change_mod(rm_projs=[api_affectee_ship.id])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(500)


def test_add_with_change_with_remove(client, consts):
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
        attrs={eve_affector_attr.id: -60, eve_optimal_attr.id: 10000, eve_falloff_attr.id: 5000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_affectee_struct = client.mk_eve_struct(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_struct = api_affectee_fit.set_ship(type_id=eve_affectee_struct.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(500)
    api_affector_module.change_mod(add_projs=[(api_affectee_struct.id, 15000)])
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(350)
    api_affector_module.change_mod(change_projs=[(api_affectee_struct.id, 20000)])
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(481.25)
    api_affector_module.change_mod(rm_projs=[api_affectee_struct.id])
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(500)


def test_value_change_optimal(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    eve_boost_attr = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_module_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id,
        mod_info=[eve_module_mod])
    eve_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -60, eve_optimal_attr.id: 10000, eve_falloff_attr.id: 5000},
        eff_ids=[eve_module_effect.id],
        defeff_id=eve_module_effect.id)
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr.id,
        affectee_attr_id=eve_optimal_attr.id)
    eve_rig_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_affector_rig = client.mk_eve_item(attrs={eve_boost_attr.id: 25}, eff_ids=[eve_rig_effect.id])
    eve_affector_ship = client.mk_eve_ship()
    eve_affectee_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_affector_ship.id)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 15000)])
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(350)
    api_affector_rig = api_affector_fit.add_rig(type_id=eve_affector_rig.id)
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(247.73108)
    api_affector_rig.remove()
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(350)


def test_value_change_falloff(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_optimal_attr = client.mk_eve_attr()
    eve_falloff_attr = client.mk_eve_attr()
    eve_boost_attr = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_module_effect = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr.id,
        falloff_attr_id=eve_falloff_attr.id,
        mod_info=[eve_module_mod])
    eve_affector_module = client.mk_eve_item(
        attrs={eve_affector_attr.id: -60, eve_optimal_attr.id: 10000, eve_falloff_attr.id: 5000},
        eff_ids=[eve_module_effect.id],
        defeff_id=eve_module_effect.id)
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        dom=consts.EveModDom.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr.id,
        affectee_attr_id=eve_falloff_attr.id)
    eve_rig_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_affector_rig = client.mk_eve_item(attrs={eve_boost_attr.id: 50}, eff_ids=[eve_rig_effect.id])
    eve_affector_ship = client.mk_eve_ship()
    eve_affectee_struct = client.mk_eve_struct(attrs={eve_affectee_attr.id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_affector_ship.id)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_struct = api_affectee_fit.set_ship(type_id=eve_affectee_struct.id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[(api_affectee_struct.id, 15000)])
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(350)
    api_affector_rig = api_affector_fit.add_rig(type_id=eve_affector_rig.id)
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(279.53983)
    api_affector_rig.remove()
    assert api_affectee_struct.update().attrs[eve_affectee_attr.id].dogma == approx(350)
