from fw import approx
from fw.api import ValOptions


def test_same_value_module(client, consts):
    # As of 2025-03-18, only PANIC uses it
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_module1_id = client.mk_eve_module(attrs={eve_limit_attr_id: 1})
    eve_module2_id = client.mk_eve_module(attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.offline)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_type_fitted=True),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_same_value_service(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_service1_id = client.mk_eve_service(attrs={eve_limit_attr_id: 1})
    eve_service2_id = client.mk_eve_service(attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_service(type_id=eve_service1_id, state=consts.ApiServiceState.offline)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_type_fitted=True),
        passing=[eve_service2_id],
        failing=[eve_service1_id])


def test_known_failures(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_service1_id = client.mk_eve_service(attrs={eve_limit_attr_id: 1})
    eve_service2_id = client.mk_eve_service(attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_service1 = api_fit.add_service(type_id=eve_service1_id, state=consts.ApiServiceState.offline)
    api_service2 = api_fit.add_service(type_id=eve_service1_id, state=consts.ApiServiceState.offline)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_type_fitted=[api_service1.id]),
        passing=[],
        failing=[eve_service2_id, eve_service1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_type_fitted=[api_service2.id]),
        passing=[],
        failing=[eve_service2_id, eve_service1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_type_fitted=[api_service1.id, api_service2.id]),
        passing=[eve_service2_id],
        failing=[eve_service1_id])


def test_switch_type_id(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_module1_id = client.mk_eve_module(attrs={eve_limit_attr_id: 1})
    eve_module2_id = client.mk_eve_module()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.offline)
    api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.offline)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_type_fitted=True),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_rounding(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_service1_id = client.mk_eve_service(attrs={eve_limit_attr_id: 0.6})
    eve_service2_id = client.mk_eve_service(attrs={eve_limit_attr_id: 1.4})
    eve_service3_id = client.mk_eve_service(attrs={eve_limit_attr_id: 1.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_service(type_id=eve_service1_id, state=consts.ApiServiceState.offline)
    api_fit.add_service(type_id=eve_service2_id, state=consts.ApiServiceState.offline)
    api_fit.add_service(type_id=eve_service3_id, state=consts.ApiServiceState.offline)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_type_fitted=True),
        passing=[eve_service3_id],
        failing=[eve_service1_id, eve_service2_id])


def test_non_positive(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_service1_id = client.mk_eve_service(attrs={eve_limit_attr_id: -2})
    eve_service2_id = client.mk_eve_service(attrs={eve_limit_attr_id: 0})
    eve_service3_id = client.mk_eve_service(attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_type_fitted=True),
        passing=[eve_service3_id],
        failing=[eve_service1_id, eve_service2_id])


def test_modified(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_limit_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_service_id = client.mk_eve_service(attrs={eve_limit_attr_id: 1, eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_service = api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.offline)
    # Verification
    assert api_service.update().attrs[eve_limit_attr_id].modified == approx(2)
    api_fit.check_try_fit_items(val_options=ValOptions(max_type_fitted=True), passing=[], failing=[eve_service_id])


def test_mutation_limit_priority(client, consts):
    # Unrealistic scenario, just checking what happens
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_base_module_id = client.mk_eve_module(attrs={eve_limit_attr_id: 2})
    eve_mutated_module_id = client.mk_eve_module(attrs={eve_limit_attr_id: 1})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_limit_attr_id: (1, 5)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_base_module_id,
        state=consts.ApiModuleState.offline,
        mutation=eve_mutator_id)
    # Verification - mutated item counts as item of mutated type, and uses its limit
    assert api_module.update().attrs[eve_limit_attr_id].modified == approx(2)
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_type_fitted=True),
        passing=[eve_base_module_id],
        failing=[eve_mutated_module_id])


def test_mutation_limit_inheritance(client, consts):
    # Mutated items consider the restriction only when it is defined directly on mutated type
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_base_module_id = client.mk_eve_module(attrs={eve_limit_attr_id: 1})
    eve_mutated_module_id = client.mk_eve_module()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_limit_attr_id: (1, 5)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_base_module_id,
        state=consts.ApiModuleState.offline,
        mutation=eve_mutator_id)
    # Verification
    assert api_module.update().attrs[eve_limit_attr_id].modified == approx(1)
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_type_fitted=True),
        passing=[eve_base_module_id, eve_mutated_module_id],
        failing=[])
    # Action
    api_module.change_module(mutation=None)
    # Verification
    assert api_module.update().attrs[eve_limit_attr_id].modified == approx(1)
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_type_fitted=True),
        passing=[eve_mutated_module_id],
        failing=[eve_base_module_id])


def test_no_attr(client, consts):
    eve_limit_attr_id = consts.EveAttr.max_type_fitted
    eve_service_id = client.mk_eve_service(attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.offline)
    # Verification - when attribute is not available, validation passes
    api_fit.check_try_fit_items(val_options=ValOptions(max_type_fitted=True), passing=[eve_service_id], failing=[])


def test_criterion_item_state(client, consts):
    # Disabled modules and services are affected
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_module_id = client.mk_eve_module(attrs={eve_limit_attr_id: 1})
    eve_service_id = client.mk_eve_service(attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.disabled)
    api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.disabled)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_type_fitted=True),
        passing=[],
        failing=[eve_module_id, eve_service_id])


def test_criterion_item_kind(client, consts):
    # Validation applies only to modules and services
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_type_fitted)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_booster_id = client.mk_eve_booster(attrs={eve_limit_attr_id: 0})
    eve_charge_id = client.mk_eve_charge(attrs={eve_limit_attr_id: 0})
    eve_drone_id = client.mk_eve_drone(attrs={eve_limit_attr_id: 0})
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_limit_attr_id: 0})
    eve_implant_id = client.mk_eve_implant(attrs={eve_limit_attr_id: 0})
    eve_module_id = client.mk_eve_module(attrs={eve_limit_attr_id: 0})
    eve_rig_id = client.mk_eve_rig(attrs={eve_limit_attr_id: 0})
    eve_service_id = client.mk_eve_service(attrs={eve_limit_attr_id: 0})
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_limit_attr_id: 0})
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.offline)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_type_fitted=True),
        passing=[
            eve_booster_id,
            eve_charge_id,
            eve_drone_id,
            eve_fighter_id,
            eve_implant_id,
            eve_rig_id,
            eve_subsystem_id],
        failing=[eve_module_id, eve_service_id])
