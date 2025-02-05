from tests import approx


def test_add_with_change_without_remove(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -60, eve_optimal_attr_id: 10000, eve_falloff_attr_id: 5000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship()
    eve_affectee_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_rig = api_affectee_fit.add_rig(type_id=eve_affectee_rig_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module_id, state=consts.ApiModuleState.active)
    assert api_affectee_rig.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 15000)])
    assert api_affectee_rig.update().attrs[eve_affectee_attr_id].dogma == approx(350)
    api_affector_module.change_mod(change_projs=[(api_affectee_ship.id, None)])
    assert api_affectee_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_affector_module.change_mod(rm_projs=[api_affectee_ship.id])
    assert api_affectee_rig.update().attrs[eve_affectee_attr_id].dogma == approx(500)


def test_add_without_change_with_remove(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -60, eve_optimal_attr_id: 10000, eve_falloff_attr_id: 5000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_struct_id = client.mk_eve_struct()
    eve_affectee_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_struct = api_affectee_fit.set_ship(type_id=eve_affectee_struct_id)
    api_affectee_rig = api_affectee_fit.add_rig(type_id=eve_affectee_rig_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module_id, state=consts.ApiModuleState.active)
    assert api_affectee_rig.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    api_affector_module.change_mod(add_projs=[(api_affectee_struct.id, None)])
    assert api_affectee_rig.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_affector_module.change_mod(change_projs=[(api_affectee_struct.id, 15000)])
    assert api_affectee_rig.update().attrs[eve_affectee_attr_id].dogma == approx(350)
    api_affector_module.change_mod(rm_projs=[api_affectee_struct.id])
    assert api_affectee_rig.update().attrs[eve_affectee_attr_id].dogma == approx(500)


def test_add_with_change_with_remove(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_mod])
    eve_affector_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -60, eve_optimal_attr_id: 10000, eve_falloff_attr_id: 5000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_affectee_ship_id = client.mk_eve_ship()
    eve_affectee_rig_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_affectee_ship_id)
    api_affectee_rig = api_affectee_fit.add_rig(type_id=eve_affectee_rig_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_affector_module_id, state=consts.ApiModuleState.active)
    assert api_affectee_rig.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 15000)])
    assert api_affectee_rig.update().attrs[eve_affectee_attr_id].dogma == approx(350)
    api_affector_module.change_mod(change_projs=[(api_affectee_ship.id, 20000)])
    assert api_affectee_rig.update().attrs[eve_affectee_attr_id].dogma == approx(481.25)
    api_affector_module.change_mod(rm_projs=[api_affectee_ship.id])
    assert api_affectee_rig.update().attrs[eve_affectee_attr_id].dogma == approx(500)


def test_value_change_optimal(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_boost_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_module_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -60, eve_optimal_attr_id: 10000, eve_falloff_attr_id: 5000},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr_id,
        affectee_attr_id=eve_optimal_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_affector_rig_id = client.mk_eve_item(attrs={eve_boost_attr_id: 25}, eff_ids=[eve_rig_effect_id])
    eve_affector_ship_id = client.mk_eve_ship()
    eve_affectee_struct_id = client.mk_eve_struct()
    eve_affectee_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_affector_ship_id)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_struct = api_affectee_fit.set_ship(type_id=eve_affectee_struct_id)
    api_affectee_module = api_affectee_fit.add_mod(type_id=eve_affectee_module_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_affector_module.change_mod(add_projs=[(api_affectee_struct.id, 15000)])
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].dogma == approx(350)
    api_affector_rig = api_affector_fit.add_rig(type_id=eve_affector_rig_id)
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].dogma == approx(247.731075)
    api_affector_rig.remove()
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].dogma == approx(350)


def test_value_change_falloff(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_boost_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc_grp,
        loc=consts.EveModLoc.tgt,
        grp=eve_grp_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id,
        mod_info=[eve_module_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: -60, eve_optimal_attr_id: 10000, eve_falloff_attr_id: 5000},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_rig_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_boost_attr_id,
        affectee_attr_id=eve_falloff_attr_id)
    eve_rig_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_rig_mod])
    eve_affector_rig_id = client.mk_eve_item(attrs={eve_boost_attr_id: 50}, eff_ids=[eve_rig_effect_id])
    eve_ship_id = client.mk_eve_ship()
    eve_affectee_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_affectee_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_ship_id)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affectee_module = api_affectee_fit.add_mod(type_id=eve_affectee_module_id)
    api_affector_module = api_affector_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 15000)])
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].dogma == approx(350)
    api_affector_rig = api_affector_fit.add_rig(type_id=eve_affector_rig_id)
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].dogma == approx(279.539826)
    api_affector_rig.remove()
    assert api_affectee_module.update().attrs[eve_affectee_attr_id].dogma == approx(350)
