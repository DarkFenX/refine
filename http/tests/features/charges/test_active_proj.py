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
    eve_module = client.mk_eve_item()
    eve_charge = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
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
    eve_module = client.mk_eve_item()
    eve_charge = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
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


def test_charge_uncharge(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item()
    eve_charge = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
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
    api_affector_module.change_mod(charge=eve_charge.id)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1200)
    # Action
    api_affector_module.change_mod(charge=None)
    # Verification
    assert api_affectee_ship.update().attrs[eve_affectee_attr.id].dogma == approx(1000)


def test_state_up_state_down(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.tgt,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_module = client.mk_eve_item()
    eve_charge = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
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
    eve_module = client.mk_eve_item()
    eve_charge = client.mk_eve_item(
        attrs={eve_affector_attr.id: 20, eve_optimal_attr.id: 10000, eve_falloff_attr.id: 5000},
        eff_ids=[eve_effect.id],
        defeff_id=eve_effect.id)
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
