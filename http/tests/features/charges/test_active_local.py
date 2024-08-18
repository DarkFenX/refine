"""
There are no charges with active effects which affect fit-local items, but it is a possibility with
the lib, so we check it nevertheless.
"""

from tests import approx


def test_bundled_remove(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_charge = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_module = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active, charge_type_id=eve_charge.id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    # Action - remove module just for the sake of consistency check
    api_module.remove()


def test_charge_charge_uncharge(client, consts):
    eve_affector_attr1 = client.mk_eve_attr()
    eve_affector_attr2 = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
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
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr2.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect2 = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod2])
    eve_charge2 = client.mk_eve_item(
        attrs={eve_affector_attr2.id: 1.5},
        eff_ids=[eve_effect2.id],
        defeff_id=eve_effect2.id)
    eve_module = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.active)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    # Action
    api_module.change_mod(charge=eve_charge1.id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    # Action
    api_module.change_mod(charge=eve_charge2.id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(150)
    # Action
    api_module.change_mod(charge=None)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_states(client, consts):
    eve_affector_attr = client.mk_eve_attr()
    eve_affectee_attr = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr.id,
        affectee_attr_id=eve_affectee_attr.id)
    eve_effect = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_charge = client.mk_eve_item(attrs={eve_affector_attr.id: 20}, eff_ids=[eve_effect.id], defeff_id=eve_effect.id)
    eve_module = client.mk_eve_item(attrs={eve_affectee_attr.id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module.id, state=consts.ApiState.online, charge_type_id=eve_charge.id)
    api_charge = api_module.charge
    # Verification
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    # Action
    api_module.change_mod(state=consts.ApiState.active)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    # Action
    api_module.change_mod(state=consts.ApiState.online)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    # Action
    api_charge.change_charge(state=True)
    # Verification - active charge state does not override too low module state
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    # Action
    api_module.change_mod(state=consts.ApiState.active)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(120)
    # Action
    api_charge.change_charge(state=False)
    # Verification - disabled charge state stops effects, even if parent module is in high enough
    # state
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    # Action
    api_module.change_mod(state=consts.ApiState.online)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(100)
    # Action
    api_module.change_mod(state=consts.ApiState.active)
    # Verification - re-enabling module does not enable charge, since it was not enabled after
    # getting disabled
    assert api_module.update().attrs[eve_affectee_attr.id].dogma == approx(100)


def test_src_switch(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    # The same affectee attr ID
    eve_affectee_attr_id = eve_d1.mk_attr().id
    eve_d2.mk_attr(id_=eve_affectee_attr_id)
    # Different affector attr IDs
    eve_d1_affector_attr = eve_d1.mk_attr()
    eve_d2_affector_attr = eve_d2.mk_attr(avoid_ids=[eve_d1_affector_attr.id])
    # Different effect IDs
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_d1_affector_attr.id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_d1_effect = eve_d1.mk_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.other,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_d2_affector_attr.id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_d2_effect = eve_d2.mk_effect(avoid_ids=[eve_d1_effect.id], cat_id=consts.EveEffCat.target, mod_info=[eve_mod2])
    # The same charge ID
    eve_charge_id = eve_d1.mk_item(
        attrs={eve_d1_affector_attr.id: 20},
        eff_ids=[eve_d1_effect.id],
        defeff_id=eve_d1_effect.id).id
    eve_d2.mk_item(
        id_=eve_charge_id,
        attrs={eve_d2_affector_attr.id: 1.5},
        eff_ids=[eve_d2_effect.id],
        defeff_id=eve_d2_effect.id)
    # The same module IID
    eve_module_id = eve_d1.mk_item(attrs={eve_affectee_attr_id: 100}).id
    eve_d2.mk_item(id_=eve_module_id, attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.active, charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(150)
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)
