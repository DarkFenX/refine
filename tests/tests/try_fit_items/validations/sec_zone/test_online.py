from fw import approx
from fw.api import ValOptions


def test_main(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class)
    eve_hisec_module_id = client.mk_eve_module(attrs={eve_attr_id: 2})
    eve_lowsec_module_id = client.mk_eve_module(attrs={eve_attr_id: 1})
    eve_null_module_id = client.mk_eve_module(attrs={eve_attr_id: 0})
    eve_hisec_service_id = client.mk_eve_service(attrs={eve_attr_id: 2})
    eve_lowsec_service_id = client.mk_eve_service(attrs={eve_attr_id: 1})
    eve_null_service_id = client.mk_eve_service(attrs={eve_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_online=True),
        passing=[eve_hisec_module_id, eve_hisec_service_id],
        failing=[eve_lowsec_module_id, eve_null_module_id, eve_lowsec_service_id, eve_null_service_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hisec_c5)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_online=True),
        passing=[eve_hisec_module_id, eve_hisec_service_id],
        failing=[eve_lowsec_module_id, eve_null_module_id, eve_lowsec_service_id, eve_null_service_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_online=True),
        passing=[eve_hisec_module_id, eve_lowsec_module_id, eve_hisec_service_id, eve_lowsec_service_id],
        failing=[eve_null_module_id, eve_null_service_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec_c5)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_online=True),
        passing=[eve_hisec_module_id, eve_lowsec_module_id, eve_hisec_service_id, eve_lowsec_service_id],
        failing=[eve_null_module_id, eve_null_service_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_online=True),
        passing=[
            eve_hisec_module_id,
            eve_lowsec_module_id,
            eve_null_module_id,
            eve_hisec_service_id,
            eve_lowsec_service_id,
            eve_null_service_id],
        failing=[])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.wspace)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_online=True),
        passing=[
            eve_hisec_module_id,
            eve_lowsec_module_id,
            eve_null_module_id,
            eve_hisec_service_id,
            eve_lowsec_service_id,
            eve_null_service_id],
        failing=[])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hazard)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_online=True),
        passing=[
            eve_hisec_module_id,
            eve_lowsec_module_id,
            eve_null_module_id,
            eve_hisec_service_id,
            eve_lowsec_service_id,
            eve_null_service_id],
        failing=[])


def test_known_failures(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class)
    eve_hisec_service_id = client.mk_eve_service(attrs={eve_attr_id: 2})
    eve_lowsec_service_id = client.mk_eve_service(attrs={eve_attr_id: 1})
    eve_null_service_id = client.mk_eve_service(attrs={eve_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_lowsec_service = api_fit.add_service(type_id=eve_lowsec_service_id, state=consts.ApiServiceState.online)
    api_nullsec_service = api_fit.add_service(type_id=eve_null_service_id, state=consts.ApiServiceState.online)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_online=[api_lowsec_service.id]),
        passing=[],
        failing=[eve_hisec_service_id, eve_lowsec_service_id, eve_null_service_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_online=[api_nullsec_service.id]),
        passing=[],
        failing=[eve_hisec_service_id, eve_lowsec_service_id, eve_null_service_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_online=[api_lowsec_service.id, api_nullsec_service.id]),
        passing=[eve_hisec_service_id],
        failing=[eve_lowsec_service_id, eve_null_service_id])


def test_rounding(client, consts):
    # There is no rounding for this validator, "higher" item is considered as lowsec-able, while
    # "lower" one is null and alikes only
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class)
    eve_higher_service_id = client.mk_eve_service(attrs={eve_attr_id: 1.01})
    eve_lower_service_id = client.mk_eve_service(attrs={eve_attr_id: 0.99})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_online=True),
        passing=[],
        failing=[eve_higher_service_id, eve_lower_service_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_online=True),
        passing=[eve_higher_service_id],
        failing=[eve_lower_service_id])


def test_modified(client, consts):
    # Unrealistic scenario, since this attribute doesn't seem to be modified by anything
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class)
    eve_service1_id = client.mk_eve_service(attrs={eve_attr_id: 1})
    eve_service2_id = client.mk_eve_service(attrs={eve_attr_id: 0})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 2}, eff_ids=[eve_effect_id])
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_fit.add_rig(type_id=eve_rig_id)
    api_service = api_fit.add_service(type_id=eve_service1_id, state=consts.ApiServiceState.online)
    # Verification - modification is ignored for the validation purposes
    assert api_service.update().attrs[eve_attr_id].modified == approx(2)
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_online=True),
        passing=[eve_service1_id],
        failing=[eve_service2_id])


def test_no_value(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class, def_val=0)
    eve_service_id = client.mk_eve_service()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(sec_zone_online=True), passing=[eve_service_id], failing=[])


def test_no_attr(client, consts):
    eve_attr_id = consts.EveAttr.online_max_security_class
    eve_service_id = client.mk_eve_service(attrs={eve_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    # Verification - when attribute can't be fetched, it's assumed to be non-limited in any sec zone
    api_fit.check_try_fit_items(val_options=ValOptions(sec_zone_online=True), passing=[eve_service_id], failing=[])


def test_criterion_item_kind(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_booster_id = client.mk_eve_booster(attrs={eve_attr_id: 1})
    eve_charge_id = client.mk_eve_charge(attrs={eve_attr_id: 1})
    eve_drone_id = client.mk_eve_drone(attrs={eve_attr_id: 1})
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_attr_id: 1})
    eve_implant_id = client.mk_eve_implant(attrs={eve_attr_id: 1})
    eve_module_id = client.mk_eve_module(attrs={eve_attr_id: 1})
    eve_rig_id = client.mk_eve_rig(attrs={eve_attr_id: 1})
    eve_service_id = client.mk_eve_service(attrs={eve_attr_id: 1})
    eve_subsystem_id = client.mk_eve_subsystem(attrs={eve_attr_id: 1})
    eve_launcher_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_online=True),
        passing=[
            eve_booster_id,
            eve_charge_id,
            eve_drone_id,
            eve_fighter_id,
            eve_implant_id,
            eve_rig_id,
            eve_subsystem_id],
        failing=[eve_module_id, eve_service_id])
