"""
Some charges have active projectable effects (e.g. wubbles, HIC rays), here we check how their
effects are applied/removed in different circumstances.
"""

from tests import approx


def test_bundled_proj_unproj(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_charge = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_module = client.mk_eve_item()
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(
        type_id=eve_module.id,
        state=consts.ApiState.active,
        charge_type_id=eve_charge.id)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1200)
    # Action
    api_affector_module.change_mod(rm_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)


def test_bundled_remove(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_charge = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_module = client.mk_eve_item()
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(
        type_id=eve_module.id,
        state=consts.ApiState.active,
        charge_type_id=eve_charge.id)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1200)
    # Action
    api_affector_module.remove()
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)


def test_charge_charge_uncharge(client, consts):
    eve_affector_attr1 = client.mk_eve_attr()
    eve_affector_attr2 = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr1.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect1 = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod1])
    eve_charge1 = client.mk_eve_item(
        attrs={eve_affector_attr1.id: 20},
        eff_ids=[eve_effect1.id],
        defeff_id=eve_effect1.id)
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr2.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect2 = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod2])
    eve_charge2 = client.mk_eve_item(
        attrs={eve_affector_attr2.id: 1.5},
        eff_ids=[eve_effect2.id],
        defeff_id=eve_effect2.id)
    eve_module = client.mk_eve_item()
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_module.change_mod(charge=eve_charge1.id)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1200)
    # Action
    api_affector_module.change_mod(charge=eve_charge2.id)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1500)
    # Action
    api_affector_module.change_mod(charge=None)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)


def test_states(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_charge = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_module = client.mk_eve_item()
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(
        type_id=eve_module.id,
        state=consts.ApiState.online,
        charge_type_id=eve_charge.id)
    api_affector_charge = api_affector_module.charge
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_module.change_mod(state=consts.ApiState.active)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1200)
    # Action
    api_affector_module.change_mod(state=consts.ApiState.online)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_charge.change_charge(state=True)
    # Verification - active charge state does not override too low module state
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_module.change_mod(state=consts.ApiState.active)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1200)
    # Action
    api_affector_charge.change_charge(state=False)
    # Verification - disabled charge state stops effects, even if parent module is in high enough
    # state
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_module.change_mod(state=consts.ApiState.online)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)
    # Action
    api_affector_module.change_mod(state=consts.ApiState.active)
    # Verification - re-enabling module does not enable charge, since it was not enabled after
    # getting disabled
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)


def test_range(client, consts):
    # Check that module range change affects charge as well
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
    eve_charge = client.mk_eve_item(
        attrs={eve_affector_attr.id: 20, eve_optimal_attr.id: 10000, eve_falloff_attr.id: 5000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
    eve_module = client.mk_eve_item()
    eve_ship = client.mk_eve_ship(attrs={eve_affectee_attr.id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship.id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(
        type_id=eve_module.id,
        state=consts.ApiState.active,
        charge_type_id=eve_charge.id)
    api_affector_module.change_mod(add_projs=[(api_affectee_ship.id, 10000)])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1200)
    # Action
    api_affector_module.change_mod(change_projs=[(api_affectee_ship.id, 15000)])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1100)
    # Action
    api_affector_module.change_mod(change_projs=[(api_affectee_ship.id, None)])
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1200)


def test_src_switch(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_affectee_attr_id = eve_d1.mk_attr().id
    eve_d2.mk_attr(id_=eve_affectee_attr_id)
    eve_d1_affector_attr_id = eve_d1.mk_attr().id
    eve_d2_affector_attr_id = eve_d2.alloc_attr_id(avoid_ids=[eve_d1_affector_attr_id])
    eve_d2.mk_attr(id_=eve_d2_affector_attr_id)
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_d1_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_d1_effect_id = eve_d1.mk_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod1]).id
    eve_charge_id = eve_d1.mk_item(
        attrs={eve_d1_affector_attr_id: 20},
        eff_ids=[eve_d1_effect_id],
        defeff_id=eve_d1_effect_id).id
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_d2_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_d2_effect_id = eve_d2.alloc_effect_id(avoid_ids=[eve_d1_effect_id])
    eve_d2_effect = eve_d2.mk_effect(id_=eve_d2_effect_id, cat_id=consts.EveEffCat.target, mod_info=[eve_mod2])
    eve_d2.mk_item(
        id_=eve_charge_id,
        attrs={eve_d2_affector_attr_id: 1.5},
        eff_ids=[eve_d2_effect.id],
        defeff_id=eve_d2_effect.id)
    eve_module_id = eve_d1.mk_item().id
    eve_d2.mk_item(id_=eve_module_id)
    eve_ship_id = eve_d1.mk_ship(attrs={eve_affectee_attr_id: 1000}).id
    eve_d2.mk_ship(id_=eve_ship_id, attrs={eve_affectee_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_affectee_fit = api_sol.create_fit()
    api_affectee_ship = api_affectee_fit.set_ship(type_id=eve_ship_id)
    api_affector_fit = api_sol.create_fit()
    api_affector_module = api_affector_fit.add_mod(
        type_id=eve_module_id,
        state=consts.ApiState.active,
        charge_type_id=eve_charge_id)
    api_affector_module.change_mod(add_projs=[api_affectee_ship.id])
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
