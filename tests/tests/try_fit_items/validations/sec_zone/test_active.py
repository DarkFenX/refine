from fw import approx
from fw.api import ValOptions


def test_charge(client, consts):
    eve_ban_empire_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_charge1_id = client.mk_eve_charge()
    eve_charge2_id = client.mk_eve_charge(attrs={eve_ban_empire_attr_id: 1})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=True),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=True),
        passing=[eve_charge1_id, eve_charge2_id],
        failing=[])
    # Action
    api_module.change_module(state=consts.ApiModuleState.overload)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=True),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=True),
        passing=[eve_charge1_id, eve_charge2_id],
        failing=[])


def test_known_failures(client, consts):
    # Fast validator has KF check at the very end of logic chain, thus we check all those conditions
    # which end up with KF check
    eve_ban_empire_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_ban_hazard_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_hazard)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_ban_empire_attr_id: 1})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_ban_hazard_attr_id: 1})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module1 = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id)
    api_module2 = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id)
    api_module3 = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge2_id)
    api_module4 = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge2_id)
    # Verification - charges are tried in the module without charge
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module1.charge.id]),
        passing=[],
        failing=[eve_charge2_id, eve_charge1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module2.charge.id]),
        passing=[],
        failing=[eve_charge2_id, eve_charge1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module1.charge.id, api_module2.charge.id]),
        passing=[eve_charge2_id],
        failing=[eve_charge1_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hisec_c5)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module1.charge.id]),
        passing=[],
        failing=[eve_charge2_id, eve_charge1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module2.charge.id]),
        passing=[],
        failing=[eve_charge2_id, eve_charge1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module1.charge.id, api_module2.charge.id]),
        passing=[eve_charge2_id],
        failing=[eve_charge1_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module1.charge.id]),
        passing=[],
        failing=[eve_charge2_id, eve_charge1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module2.charge.id]),
        passing=[],
        failing=[eve_charge2_id, eve_charge1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module1.charge.id, api_module2.charge.id]),
        passing=[eve_charge2_id],
        failing=[eve_charge1_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec_c5)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module1.charge.id]),
        passing=[],
        failing=[eve_charge2_id, eve_charge1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module2.charge.id]),
        passing=[],
        failing=[eve_charge2_id, eve_charge1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module1.charge.id, api_module2.charge.id]),
        passing=[eve_charge2_id],
        failing=[eve_charge1_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hazard)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module3.charge.id]),
        passing=[],
        failing=[eve_charge1_id, eve_charge2_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module4.charge.id]),
        passing=[],
        failing=[eve_charge1_id, eve_charge2_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=[api_module3.charge.id, api_module4.charge.id]),
        passing=[eve_charge1_id],
        failing=[eve_charge2_id])


def test_modified(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_charge1_id = client.mk_eve_charge(attrs={eve_attr_id: 0})
    eve_charge2_id = client.mk_eve_charge(attrs={eve_attr_id: 1})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig1_id = client.mk_eve_item(attrs={eve_mod_attr_id: 0}, eff_ids=[eve_mod_effect_id])
    eve_rig2_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module1 = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id)
    api_module2 = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge2_id)
    # Verification
    assert api_module1.charge.update().attrs[eve_attr_id].modified == approx(0)
    assert api_module2.charge.update().attrs[eve_attr_id].modified == approx(0)
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=True),
        passing=[eve_charge1_id, eve_charge2_id],
        failing=[])
    # Action - fitted charges would fail validation after the modification, so their modules are
    # removed
    api_module1.remove()
    api_module2.remove()
    api_rig1.remove()
    api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=True),
        passing=[],
        failing=[eve_charge1_id, eve_charge2_id])


def test_values(client, consts):
    # Check that only absence of attribute value, or value equal to 0 is treated as flag disabled
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_charge1_id = client.mk_eve_charge()
    eve_charge2_id = client.mk_eve_charge(attrs={eve_attr_id: 0})
    eve_charge3_id = client.mk_eve_charge(attrs={eve_attr_id: 1})
    eve_charge4_id = client.mk_eve_charge(attrs={eve_attr_id: 0.01})
    eve_charge5_id = client.mk_eve_charge(attrs={eve_attr_id: -0.01})
    eve_charge6_id = client.mk_eve_charge(attrs={eve_attr_id: -5000})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=True),
        passing=[eve_charge1_id, eve_charge2_id],
        failing=[eve_charge3_id, eve_charge4_id, eve_charge5_id, eve_charge6_id])


def test_no_attr(client, consts):
    eve_attr_id = consts.EveAttr.disallow_in_empire_space
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_charge_id = client.mk_eve_charge(attrs={eve_attr_id: 1})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification - when attribute can't be fetched, it's assumed to be non-limited in any sec zone
    api_fit.check_try_fit_items(val_options=ValOptions(sec_zone_active=True), passing=[eve_charge_id], failing=[])


def test_state(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_charge_id = client.mk_eve_charge(attrs={eve_attr_id: 1})
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    # Verification - charge is active only when module is active
    api_fit.check_try_fit_items(val_options=ValOptions(sec_zone_active=True), passing=[eve_charge_id], failing=[])
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(sec_zone_active=True), passing=[eve_charge_id], failing=[])
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(sec_zone_active=True), passing=[], failing=[eve_charge_id])
    # Action
    api_module.change_module(state=consts.ApiModuleState.overload)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(sec_zone_active=True), passing=[], failing=[eve_charge_id])


def test_criterion_item_kind(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.activates_charge, cat_id=consts.EveEffCat.active)
    eve_booster_id = client.mk_eve_booster(attrs={eve_attr_id: 1})
    eve_charge_id = client.mk_eve_charge(attrs={eve_attr_id: 1})
    eve_drone_id = client.mk_eve_drone(attrs={eve_attr_id: 1})
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_attr_id: 1})
    eve_implant_id = client.mk_eve_implant(attrs={eve_attr_id: 1})
    eve_module_id = client.mk_eve_module(attrs={eve_attr_id: 1})
    eve_rig_id = client.mk_eve_rig(attrs={eve_attr_id: 1})
    eve_service_id = client.mk_eve_service(attrs={eve_attr_id: 1})
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_attr_id: 1})
    eve_launcher_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_launcher_id, state=consts.ApiModuleState.active)
    # Verification - modules are tried at most online, so only charges loaded into active modules
    # can fail
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_active=True),
        passing=[
            eve_booster_id,
            eve_drone_id,
            eve_fighter_id,
            eve_implant_id,
            eve_module_id,
            eve_rig_id,
            eve_service_id,
            eve_subsystem_id],
        failing=[eve_charge_id])
