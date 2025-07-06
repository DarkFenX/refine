"""
Some charges have active projectable effects (e.g. wubbles, HIC rays), here we check how their
effects are applied/removed in different circumstances.
"""

from tests import approx


def test_bundled_proj_unproj(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_charge_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
    # Action
    api_affector_module.change_module(rm_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)


def test_bundled_remove(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_charge_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
    # Action
    api_affector_module.remove()
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)


def test_charge_charge_uncharge(client, consts):
    eve_affector_attr1_id = client.mk_eve_attr()
    eve_affector_attr2_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr1_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect1_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod1])
    eve_charge1_id = client.mk_eve_item(
        attrs={eve_affector_attr1_id: 20},
        eff_ids=[eve_effect1_id],
        defeff_id=eve_effect1_id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr2_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod2])
    eve_charge2_id = client.mk_eve_item(
        attrs={eve_affector_attr2_id: 1.5},
        eff_ids=[eve_effect2_id],
        defeff_id=eve_effect2_id)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_affector_module.change_module(charge_type_id=eve_charge1_id)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
    # Action
    api_affector_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1500)
    # Action
    api_affector_module.change_module(charge_type_id=None)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)


def test_states(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_charge_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.online,
        charge_type_id=eve_charge_id)
    api_affector_charge = api_affector_module.charge
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_affector_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
    # Action
    api_affector_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_affector_charge.change_charge(state=True)
    # Verification - active charge state does not override too low module state
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_affector_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
    # Action
    api_affector_charge.change_charge(state=False)
    # Verification - disabled charge state stops effects, even if parent module is in high enough
    # state
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_affector_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)
    # Action
    api_affector_module.change_module(state=consts.ApiModuleState.active)
    # Verification - re-enabling module does not enable charge, since it was not enabled after
    # getting disabled
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1000)


def test_range(client, consts):
    # Check that module range change affects charge as well
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
    eve_charge_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20, eve_optimal_attr_id: 10000, eve_falloff_attr_id: 5000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_affector_module.change_module(add_projs=[(api_affectee_ship.id, 10000)])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
    # Action
    api_affector_module.change_module(change_projs=[(api_affectee_ship.id, 15000)])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1100)
    # Action
    api_affector_module.change_module(change_projs=[(api_affectee_ship.id, None)])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)


def test_switch_src(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    # The same affectee attr ID
    eve_affectee_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    # Different affector attrs ID
    eve_d1_affector_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d1], id_=eve_d1_affector_attr_id)
    eve_d2_affector_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d2], id_=eve_d2_affector_attr_id)
    # Different effect IDs
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_d1_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_d1_effect_id = client.alloc_effect_id(datas=[eve_d1, eve_d2])
    client.mk_eve_effect(datas=[eve_d1], id_=eve_d1_effect_id, cat_id=consts.EveEffCat.target, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.tgt,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_d2_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_d2_effect_id = client.alloc_effect_id(datas=[eve_d1, eve_d2])
    client.mk_eve_effect(datas=[eve_d2], id_=eve_d2_effect_id, cat_id=consts.EveEffCat.target, mod_info=[eve_mod2])
    # The same charge ID
    eve_charge_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_charge_id,
        attrs={eve_d1_affector_attr_id: 20},
        eff_ids=[eve_d1_effect_id],
        defeff_id=eve_d1_effect_id)
    client.mk_eve_item(
        datas=[eve_d2],
        id_=eve_charge_id,
        attrs={eve_d2_affector_attr_id: 1.5},
        eff_ids=[eve_d2_effect_id],
        defeff_id=eve_d2_effect_id)
    # The same module ID
    eve_module_id = client.mk_eve_item(datas=[eve_d1, eve_d2])
    # The same ship ID
    eve_ship_id = client.mk_eve_item(datas=[eve_d1, eve_d2], attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_affector_module.change_module(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1500)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr_id].dogma == approx(1200)
