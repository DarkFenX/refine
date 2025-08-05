"""
There are no charges with active effects which affect fit-local items, but it is a possibility with
the lib, so we check it nevertheless.
"""

from tests import Effect, approx


def test_bundled_remove(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_act_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affectee_attr_id: 100}, eff_ids=[eve_act_effect_id], defeff_id=eve_act_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action - remove module just for the sake of consistency check
    api_module.remove()


def test_charge_charge_uncharge(client, consts):
    eve_affector_attr1_id = client.mk_eve_attr()
    eve_affector_attr2_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
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
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_affector_attr2_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect2_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod2])
    eve_charge2_id = client.mk_eve_item(
        attrs={eve_affector_attr2_id: 1.5},
        eff_ids=[eve_effect2_id],
        defeff_id=eve_effect2_id)
    eve_act_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affectee_attr_id: 100},
        eff_ids=[eve_act_effect_id],
        defeff_id=eve_act_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_module.change_module(charge_type_id=eve_charge1_id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(150)
    # Action
    api_module.change_module(charge_type_id=None)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_states(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.target, mod_info=[eve_mod])
    eve_act_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_affectee_attr_id: 100},
        eff_ids=[eve_act_effect_id],
        defeff_id=eve_act_effect_id)
    client.create_sources()
    api_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_effect_id)
    api_act_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_act_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.online,
        charge_type_id=eve_charge_id)
    api_charge = api_module.charge
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_charge.change_charge(state=True)
    # Verification - active charge state does not override too low module state
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_charge.change_charge(state=False)
    # Verification - disabled charge state stops effects, even if parent module is in high enough
    # state
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification - re-enabling module does not enable charge, since it was not enabled after
    # getting disabled
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_charge.change_charge(state=True)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_charge.change_charge(effect_modes={api_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_charge.change_charge(effect_modes={api_effect_id: consts.ApiEffMode.full_compliance})
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_module.change_module(effect_modes={api_act_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_module.change_module(effect_modes={api_act_effect_id: consts.ApiEffMode.full_compliance})
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_module.change_module(effect_modes={api_act_effect_id: consts.ApiEffMode.force_run})
    # Verification
    assert api_module.update().attrs[eve_affectee_attr_id].dogma == approx(120)


def test_switch_src(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    # The same affectee attr ID
    eve_affectee_attr_id = client.mk_eve_attr(datas=[eve_d1, eve_d2])
    # Different affector attr IDs
    eve_d1_affector_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d1], id_=eve_d1_affector_attr_id)
    eve_d2_affector_attr_id = client.alloc_attr_id(datas=[eve_d1, eve_d2])
    client.mk_eve_attr(datas=[eve_d2], id_=eve_d2_affector_attr_id)
    # Different effect IDs
    eve_mod1 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_d1_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_d1_effect_id = client.alloc_effect_id(datas=[eve_d1, eve_d2])
    client.mk_eve_effect(datas=[eve_d1], id_=eve_d1_effect_id, cat_id=consts.EveEffCat.target, mod_info=[eve_mod1])
    eve_mod2 = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.other,
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
    eve_act_effect_id = client.mk_eve_effect(
        datas=[eve_d1, eve_d2],
        id_=consts.UtilEffect.activates_charge,
        cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        datas=[eve_d1, eve_d2],
        attrs={eve_affectee_attr_id: 100},
        eff_ids=[eve_act_effect_id],
        defeff_id=eve_act_effect_id)
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
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


def test_non_activating(client, consts):
    # Modules with effects which do not activate charge should never apply charge active effects
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod])
    eve_act_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_nonact_effect_id = client.mk_eve_effect(
        id_=consts.UtilEffect.not_activates_charge, cat_id=consts.EveEffCat.active)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_module1_id = client.mk_eve_item(eff_ids=[eve_nonact_effect_id], defeff_id=eve_nonact_effect_id)
    eve_module2_id = client.mk_eve_item(eff_ids=[eve_act_effect_id], defeff_id=eve_act_effect_id)
    eve_character_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_nonact_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_nonact_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_character = api_fit.set_character(type_id=eve_character_id)
    api_module = api_fit.add_module(
        type_id=eve_module1_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_module.change_module(effect_modes={api_nonact_effect_id: consts.ApiEffMode.force_run})
    # Verification
    assert api_character.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_module.change_module(type_id=eve_module2_id)
    # Verification
    assert api_character.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_module.change_module(type_id=eve_module1_id)
    # Verification
    assert api_character.update().attrs[eve_affectee_attr_id].dogma == approx(100)


def test_non_default_effect(client, consts):
    # Only default effect can activate charge
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod])
    eve_act_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20}, eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_module1_id = client.mk_eve_item(eff_ids=[eve_act_effect_id])
    eve_module2_id = client.mk_eve_item(eff_ids=[eve_act_effect_id], defeff_id=eve_act_effect_id)
    eve_character_id = client.mk_eve_item(attrs={eve_affectee_attr_id: 100})
    client.create_sources()
    api_act_effect_id = Effect.dogma_to_api(dogma_effect_id=eve_act_effect_id)
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_character = api_fit.set_character(type_id=eve_character_id)
    api_module = api_fit.add_module(
        type_id=eve_module1_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_module.change_module(effect_modes={api_act_effect_id: consts.ApiEffMode.force_run})
    # Verification
    assert api_character.update().attrs[eve_affectee_attr_id].dogma == approx(100)
    # Action
    api_module.change_module(type_id=eve_module2_id)
    # Verification
    assert api_character.update().attrs[eve_affectee_attr_id].dogma == approx(120)
    # Action
    api_module.change_module(type_id=eve_module1_id)
    # Verification
    assert api_character.update().attrs[eve_affectee_attr_id].dogma == approx(100)
