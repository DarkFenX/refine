from tests import approx


def test_add_with_change_without_remove(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
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
        attrs={eve_affector_attr_id: -60, eve_optimal_attr_id: 10000, eve_falloff_attr_id: 5000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_affectee_drone_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 500}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_affectee_drone_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id, coordinates=(15000, 0, 0))
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone_id)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(350)
    api_affectee_ship.change_ship(coordinates=(0, 0, 0))
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_affector_module.change_module(rm_projs=[api_affectee_ship.id])
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(500)


def test_add_without_change_with_remove(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
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
        attrs={eve_affector_attr_id: -60, eve_optimal_attr_id: 10000, eve_falloff_attr_id: 5000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    eve_affectee_fighter_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 500}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affectee_fighter = api_affectee_fit.add_drone(type_id=eve_affectee_fighter_id)
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(200)
    api_affectee_ship.change_ship(coordinates=(15000, 0, 0))
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(350)
    api_affector_module.change_module(rm_projs=[api_affectee_ship.id])
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(500)


def test_add_with_change_with_remove(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
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
        attrs={eve_affector_attr_id: -60, eve_optimal_attr_id: 10000, eve_falloff_attr_id: 5000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_struct_id = client.mk_eve_struct()
    eve_affectee_drone_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 500}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_struct_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_affector_module_id,
        state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_struct = api_affectee_fit.set_ship(type_id=eve_struct_id, coordinates=(15000, 0, 0))
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone_id)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(500)
    api_affector_module.change_module(add_projs=[api_affectee_struct.id])
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(350)
    api_affectee_struct.change_ship(coordinates=(20000, 0, 0))
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(481.25)
    api_affector_module.change_module(rm_projs=[api_affectee_struct.id])
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(500)


def test_value_change_optimal(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_boost_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
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
    eve_ship_id = client.mk_eve_ship()
    eve_affectee_drone_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 500}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_ship_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id, coordinates=(15000, 0, 0))
    api_affectee_drone = api_affectee_fit.add_drone(type_id=eve_affectee_drone_id, coordinates=(0, 0, 0))
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(350)
    api_affector_rig = api_affector_fit.add_rig(type_id=eve_affector_rig_id)
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(247.731075)
    api_affector_rig.remove()
    assert api_affectee_drone.update().attrs[eve_affectee_attr_id].dogma == approx(350)


def test_value_change_falloff(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_boost_attr_id = client.mk_eve_attr()
    eve_module_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.tgt,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.tgt_normal1,
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
    eve_affector_ship_id = client.mk_eve_ship()
    eve_affectee_struct_id = client.mk_eve_struct()
    eve_affectee_fighter_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 500}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_affector_fit = api_sol.create_fit()
    api_affector_fit.set_ship(type_id=eve_affector_ship_id, coordinates=(0, 0, 0))
    api_affector_module = api_affector_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_struct = api_affectee_fit.set_ship(type_id=eve_affectee_struct_id, coordinates=(15000, 0, 0))
    api_affectee_fighter = api_affectee_fit.add_fighter(type_id=eve_affectee_fighter_id)
    api_affector_module.change_module(add_projs=[api_affectee_struct.id])
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(350)
    api_affector_rig = api_affector_fit.add_rig(type_id=eve_affector_rig_id)
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(279.539826)
    api_affector_rig.remove()
    assert api_affectee_fighter.update().attrs[eve_affectee_attr_id].dogma == approx(350)
