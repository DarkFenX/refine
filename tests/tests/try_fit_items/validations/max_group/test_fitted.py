from fw import Muta, approx
from fw.api import ValOptions


def test_same_value_module(client, consts):
    # Simple but realistic scenario
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_module1_id = client.mk_eve_module(grp_id=eve_grp1_id, attrs={eve_limit_attr_id: 1})
    eve_module2_id = client.mk_eve_module(grp_id=eve_grp2_id, attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.offline)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_group_fitted=True),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_same_value_rig(client, consts):
    # Simple but realistic scenario
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_rig1_id = client.mk_eve_rig(grp_id=eve_grp1_id, attrs={eve_limit_attr_id: 1})
    eve_rig2_id = client.mk_eve_rig(grp_id=eve_grp2_id, attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_rig1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_group_fitted=True),
        passing=[eve_rig2_id],
        failing=[eve_rig1_id])


def test_same_value_service(client, consts):
    # As of 2025-03-18, services do not seem to have this limit in EVE, but they are in many ways
    # are similar to modules, so we subject them to this limit just in case
    eve_grp1_id = client.mk_eve_item_group(cat_id=consts.EveItemCat.structure_module)
    eve_grp2_id = client.mk_eve_item_group(cat_id=consts.EveItemCat.structure_module)
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_service1_id = client.mk_eve_service(grp_id=eve_grp1_id, attrs={eve_limit_attr_id: 1})
    eve_service2_id = client.mk_eve_service(grp_id=eve_grp2_id, attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_service(type_id=eve_service1_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_group_fitted=True),
        passing=[eve_service2_id],
        failing=[eve_service1_id])


def test_different_values(client, consts):
    # Checks in details how validation works, but uses unrealistic scenario, since modules in the
    # same EVE group have the same value of the attribute
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_module1_id = client.mk_eve_module(grp_id=eve_grp_id)
    eve_module2_id = client.mk_eve_module(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 2})
    eve_module3_id = client.mk_eve_module(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module3_id, state=consts.ApiModuleState.offline)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_group_fitted=True),
        passing=[eve_module1_id, eve_module2_id, eve_module3_id],
        failing=[])
    # Action
    api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.offline)
    # Verification - third module in the group fails limit of the second module
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_group_fitted=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id, eve_module3_id])


def test_known_failures(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_module1_id = client.mk_eve_module(grp_id=eve_grp1_id, attrs={eve_limit_attr_id: 1})
    eve_module2_id = client.mk_eve_module(grp_id=eve_grp2_id, attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.offline)
    api_module2 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.offline)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_group_fitted=[api_module1.id]),
        passing=[],
        failing=[eve_module2_id, eve_module1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_group_fitted=[api_module2.id]),
        passing=[],
        failing=[eve_module2_id, eve_module1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_group_fitted=[api_module1.id, api_module2.id]),
        passing=[eve_module2_id],
        failing=[eve_module1_id])


def test_modified(client, consts):
    # Check rounding in this case too
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_limit_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_module1_id = client.mk_eve_module(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 0.6})
    eve_module2_id = client.mk_eve_module(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1.4})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.offline)
    # Verification
    assert api_module1.update().attrs[eve_limit_attr_id].modified == approx(0.6)
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_group_fitted=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    assert api_module1.update().attrs[eve_limit_attr_id].modified == approx(1.6)
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_group_fitted=True),
        passing=[eve_module1_id, eve_module2_id],
        failing=[])


def test_mutation_limit_priority(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_base_module_id = client.mk_eve_module(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 2})
    eve_mutated_module_id = client.mk_eve_module(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_limit_attr_id: (0.5, 2)})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_base_module_id,
        state=consts.ApiModuleState.offline,
        mutation=eve_mutator_id)
    # Verification
    assert api_module.update().attrs[eve_limit_attr_id].modified == approx(2)
    api_fit.check_try_fit_items(val_options=ValOptions(max_group_fitted=True), passing=[eve_base_module_id], failing=[])
    # Action
    api_module.change_module(mutation=(eve_mutator_id, {eve_limit_attr_id: Muta.abs_to_api(val=1)}))
    # Verification
    assert api_module.update().attrs[eve_limit_attr_id].modified == approx(1)
    api_fit.check_try_fit_items(val_options=ValOptions(max_group_fitted=True), passing=[], failing=[eve_base_module_id])


def test_mutation_limit_inheritance(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_base_module_id = client.mk_eve_module(grp_id=eve_grp1_id, attrs={eve_limit_attr_id: 1})
    eve_mutated_module_id = client.mk_eve_module(grp_id=eve_grp2_id)
    eve_other_module_id = client.mk_eve_module(grp_id=eve_grp2_id)
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
        val_options=ValOptions(max_group_fitted=True),
        passing=[],
        failing=[eve_other_module_id])
    # Action
    api_module.change_module(mutation=(eve_mutator_id, {eve_limit_attr_id: Muta.abs_to_api(val=2)}))
    # Verification
    assert api_module.update().attrs[eve_limit_attr_id].modified == approx(2)
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_group_fitted=True),
        passing=[eve_other_module_id],
        failing=[])


def test_no_attr(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = consts.EveAttr.max_group_fitted
    eve_module_id = client.mk_eve_module(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification - when attribute is not available, validation passes
    api_fit.check_try_fit_items(val_options=ValOptions(max_group_fitted=True), passing=[eve_module_id], failing=[])


def test_criterion_module_state(client, consts):
    # Disabled modules and offline rigs are affected
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_module_id = client.mk_eve_module(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1})
    eve_rig_id = client.mk_eve_rig(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.disabled)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(max_group_fitted=True),
        passing=[],
        failing=[eve_module_id, eve_rig_id])


def test_criterion_item_kind(client, consts):
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
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
        val_options=ValOptions(max_group_fitted=True),
        passing=[eve_booster_id, eve_charge_id, eve_drone_id, eve_fighter_id, eve_implant_id, eve_subsystem_id],
        failing=[eve_module_id, eve_rig_id, eve_service_id])
