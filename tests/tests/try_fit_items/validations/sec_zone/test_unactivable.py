from fw import approx
from fw.api import ValOptions


def test_main_module(client, consts):
    eve_ban_empire_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_ban_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_hisec)
    eve_ban_hazard_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_hazard)
    eve_corrupt_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allow_in_fully_corrupted_hisec)
    eve_corrupt_lowsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allow_in_fully_corrupted_lowsec)
    eve_module1_id = client.mk_eve_module()
    eve_module2_id = client.mk_eve_module(attrs={eve_ban_empire_attr_id: 1})
    eve_module3_id = client.mk_eve_module(attrs={eve_ban_hisec_attr_id: 1})
    eve_module4_id = client.mk_eve_module(attrs={eve_ban_hazard_attr_id: 1})
    eve_module5_id = client.mk_eve_module(attrs={eve_corrupt_hisec_attr_id: 1})
    eve_module6_id = client.mk_eve_module(attrs={eve_corrupt_lowsec_attr_id: 1})
    eve_module7_id = client.mk_eve_module(attrs={eve_ban_empire_attr_id: 1, eve_ban_hisec_attr_id: 1})
    eve_module8_id = client.mk_eve_module(attrs={eve_ban_empire_attr_id: 1, eve_ban_hazard_attr_id: 1})
    eve_module9_id = client.mk_eve_module(attrs={eve_ban_empire_attr_id: 1, eve_corrupt_hisec_attr_id: 1})
    eve_module10_id = client.mk_eve_module(attrs={eve_ban_empire_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_module11_id = client.mk_eve_module(attrs={eve_ban_hisec_attr_id: 1, eve_ban_hazard_attr_id: 1})
    eve_module12_id = client.mk_eve_module(attrs={eve_ban_hisec_attr_id: 1, eve_corrupt_hisec_attr_id: 1})
    eve_module13_id = client.mk_eve_module(attrs={eve_ban_hisec_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_module14_id = client.mk_eve_module(attrs={eve_ban_hazard_attr_id: 1, eve_corrupt_hisec_attr_id: 1})
    eve_module15_id = client.mk_eve_module(attrs={eve_ban_hazard_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_module16_id = client.mk_eve_module(attrs={eve_corrupt_hisec_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_module17_id = client.mk_eve_module(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1})
    eve_module18_id = client.mk_eve_module(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_module19_id = client.mk_eve_module(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module20_id = client.mk_eve_module(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_module21_id = client.mk_eve_module(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module22_id = client.mk_eve_module(attrs={
        eve_ban_empire_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module23_id = client.mk_eve_module(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_module24_id = client.mk_eve_module(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module25_id = client.mk_eve_module(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module26_id = client.mk_eve_module(attrs={
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module27_id = client.mk_eve_module(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_module28_id = client.mk_eve_module(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module29_id = client.mk_eve_module(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module30_id = client.mk_eve_module(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module31_id = client.mk_eve_module(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module32_id = client.mk_eve_module(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=True),
        passing=[
            eve_module1_id,
            eve_module4_id,
            eve_module5_id,
            eve_module6_id,
            eve_module14_id,
            eve_module15_id,
            eve_module16_id,
            eve_module26_id],
        failing=[
            eve_module2_id,
            eve_module3_id,
            eve_module7_id,
            eve_module8_id,
            eve_module9_id,
            eve_module10_id,
            eve_module11_id,
            eve_module12_id,
            eve_module13_id,
            eve_module17_id,
            eve_module18_id,
            eve_module19_id,
            eve_module20_id,
            eve_module21_id,
            eve_module22_id,
            eve_module23_id,
            eve_module24_id,
            eve_module25_id,
            eve_module27_id,
            eve_module28_id,
            eve_module29_id,
            eve_module30_id,
            eve_module31_id,
            eve_module32_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hisec_c5)
    # Verification - same as hisec, but all the items which could be in corrupted hisec are not
    # failing
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=True),
        passing=[
            eve_module1_id,
            eve_module4_id,
            eve_module5_id,
            eve_module6_id,
            eve_module9_id,
            eve_module12_id,
            eve_module14_id,
            eve_module15_id,
            eve_module16_id,
            eve_module18_id,
            eve_module20_id,
            eve_module22_id,
            eve_module23_id,
            eve_module25_id,
            eve_module26_id,
            eve_module27_id,
            eve_module29_id,
            eve_module30_id,
            eve_module31_id,
            eve_module32_id],
        failing=[
            eve_module2_id,
            eve_module3_id,
            eve_module7_id,
            eve_module8_id,
            eve_module10_id,
            eve_module11_id,
            eve_module13_id,
            eve_module17_id,
            eve_module19_id,
            eve_module21_id,
            eve_module24_id,
            eve_module28_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=True),
        passing=[
            eve_module1_id,
            eve_module3_id,
            eve_module4_id,
            eve_module5_id,
            eve_module6_id,
            eve_module11_id,
            eve_module12_id,
            eve_module13_id,
            eve_module14_id,
            eve_module15_id,
            eve_module16_id,
            eve_module23_id,
            eve_module24_id,
            eve_module25_id,
            eve_module26_id,
            eve_module31_id],
        failing=[
            eve_module2_id,
            eve_module7_id,
            eve_module8_id,
            eve_module9_id,
            eve_module10_id,
            eve_module17_id,
            eve_module18_id,
            eve_module19_id,
            eve_module20_id,
            eve_module21_id,
            eve_module22_id,
            eve_module27_id,
            eve_module28_id,
            eve_module29_id,
            eve_module30_id,
            eve_module32_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec_c5)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=True),
        passing=[
            eve_module1_id,
            eve_module3_id,
            eve_module4_id,
            eve_module5_id,
            eve_module6_id,
            eve_module10_id,
            eve_module11_id,
            eve_module12_id,
            eve_module13_id,
            eve_module14_id,
            eve_module15_id,
            eve_module16_id,
            eve_module19_id,
            eve_module21_id,
            eve_module22_id,
            eve_module23_id,
            eve_module24_id,
            eve_module25_id,
            eve_module26_id,
            eve_module28_id,
            eve_module29_id,
            eve_module30_id,
            eve_module31_id,
            eve_module32_id],
        failing=[
            eve_module2_id,
            eve_module7_id,
            eve_module8_id,
            eve_module9_id,
            eve_module17_id,
            eve_module18_id,
            eve_module20_id,
            eve_module27_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=True),
        passing=[
            eve_module1_id,
            eve_module2_id,
            eve_module3_id,
            eve_module4_id,
            eve_module5_id,
            eve_module6_id,
            eve_module7_id,
            eve_module8_id,
            eve_module9_id,
            eve_module10_id,
            eve_module11_id,
            eve_module12_id,
            eve_module13_id,
            eve_module14_id,
            eve_module15_id,
            eve_module16_id,
            eve_module17_id,
            eve_module18_id,
            eve_module19_id,
            eve_module20_id,
            eve_module21_id,
            eve_module22_id,
            eve_module23_id,
            eve_module24_id,
            eve_module25_id,
            eve_module26_id,
            eve_module27_id,
            eve_module28_id,
            eve_module29_id,
            eve_module30_id,
            eve_module31_id,
            eve_module32_id],
        failing=[])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.wspace)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=True),
        passing=[
            eve_module1_id,
            eve_module2_id,
            eve_module3_id,
            eve_module4_id,
            eve_module5_id,
            eve_module6_id,
            eve_module7_id,
            eve_module8_id,
            eve_module9_id,
            eve_module10_id,
            eve_module11_id,
            eve_module12_id,
            eve_module13_id,
            eve_module14_id,
            eve_module15_id,
            eve_module16_id,
            eve_module17_id,
            eve_module18_id,
            eve_module19_id,
            eve_module20_id,
            eve_module21_id,
            eve_module22_id,
            eve_module23_id,
            eve_module24_id,
            eve_module25_id,
            eve_module26_id,
            eve_module27_id,
            eve_module28_id,
            eve_module29_id,
            eve_module30_id,
            eve_module31_id,
            eve_module32_id],
        failing=[])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hazard)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=True),
        passing=[
            eve_module1_id,
            eve_module2_id,
            eve_module3_id,
            eve_module5_id,
            eve_module6_id,
            eve_module7_id,
            eve_module9_id,
            eve_module10_id,
            eve_module12_id,
            eve_module13_id,
            eve_module16_id,
            eve_module18_id,
            eve_module19_id,
            eve_module22_id,
            eve_module25_id,
            eve_module29_id],
        failing=[
            eve_module4_id,
            eve_module8_id,
            eve_module11_id,
            eve_module14_id,
            eve_module15_id,
            eve_module17_id,
            eve_module20_id,
            eve_module21_id,
            eve_module23_id,
            eve_module24_id,
            eve_module26_id,
            eve_module27_id,
            eve_module28_id,
            eve_module30_id,
            eve_module31_id,
            eve_module32_id])


def test_charge(client, consts):
    eve_ban_empire_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_id = client.mk_eve_charge(attrs={eve_ban_empire_attr_id: 1})
    eve_module_id = client.mk_eve_item(attrs={eve_capacity_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.disabled)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(sec_zone_unactivable=True), passing=[], failing=[eve_charge_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(sec_zone_unactivable=True), passing=[eve_charge_id], failing=[])


def test_known_failures(client, consts):
    # Fast validator has KF check at the very end of logic chain, thus we check all those conditions
    # which end up with KF check
    eve_ban_empire_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_ban_hazard_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_hazard)
    eve_module1_id = client.mk_eve_module(attrs={eve_ban_empire_attr_id: 1})
    eve_module2_id = client.mk_eve_module(attrs={eve_ban_hazard_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.disabled)
    api_module2 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.disabled)
    api_module3 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.disabled)
    api_module4 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.disabled)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module1.id]),
        passing=[],
        failing=[eve_module2_id, eve_module1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module2.id]),
        passing=[],
        failing=[eve_module2_id, eve_module1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module1.id, api_module2.id]),
        passing=[eve_module2_id],
        failing=[eve_module1_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hisec_c5)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module1.id]),
        passing=[],
        failing=[eve_module2_id, eve_module1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module2.id]),
        passing=[],
        failing=[eve_module2_id, eve_module1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module1.id, api_module2.id]),
        passing=[eve_module2_id],
        failing=[eve_module1_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module1.id]),
        passing=[],
        failing=[eve_module2_id, eve_module1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module2.id]),
        passing=[],
        failing=[eve_module2_id, eve_module1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module1.id, api_module2.id]),
        passing=[eve_module2_id],
        failing=[eve_module1_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec_c5)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module1.id]),
        passing=[],
        failing=[eve_module2_id, eve_module1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module2.id]),
        passing=[],
        failing=[eve_module2_id, eve_module1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module1.id, api_module2.id]),
        passing=[eve_module2_id],
        failing=[eve_module1_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hazard)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module3.id]),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module4.id]),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=[api_module3.id, api_module4.id]),
        passing=[eve_module1_id],
        failing=[eve_module2_id])


def test_modified(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_module1_id = client.mk_eve_module(attrs={eve_attr_id: 0})
    eve_module2_id = client.mk_eve_module(attrs={eve_attr_id: 1})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig1_id = client.mk_eve_item(attrs={eve_mod_attr_id: 0}, eff_ids=[eve_effect_id])
    eve_rig2_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.disabled)
    api_module2 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.disabled)
    # Verification
    assert api_module1.update().attrs[eve_attr_id].modified == approx(0)
    assert api_module2.update().attrs[eve_attr_id].modified == approx(0)
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=True),
        passing=[eve_module1_id, eve_module2_id],
        failing=[])
    # Action - fitted modules would fail validation after the modification, so they are removed
    api_module1.remove()
    api_module2.remove()
    api_rig1.remove()
    api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=True),
        passing=[],
        failing=[eve_module1_id, eve_module2_id])


def test_values(client, consts):
    # Check that only absence of attribute value, or value equal to 0 is treated as flag disabled
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_module1_id = client.mk_eve_module()
    eve_module2_id = client.mk_eve_module(attrs={eve_attr_id: 0})
    eve_module3_id = client.mk_eve_module(attrs={eve_attr_id: 1})
    eve_module4_id = client.mk_eve_module(attrs={eve_attr_id: 0.01})
    eve_module5_id = client.mk_eve_module(attrs={eve_attr_id: -0.01})
    eve_module6_id = client.mk_eve_module(attrs={eve_attr_id: -5000})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=True),
        passing=[eve_module1_id, eve_module2_id],
        failing=[eve_module3_id, eve_module4_id, eve_module5_id, eve_module6_id])


def test_no_attr(client, consts):
    eve_attr_id = consts.EveAttr.disallow_in_empire_space
    eve_module_id = client.mk_eve_module(attrs={eve_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    # Verification - when attribute can't be fetched, it's assumed to be non-limited in any sec zone
    api_fit.check_try_fit_items(val_options=ValOptions(sec_zone_unactivable=True), passing=[eve_module_id], failing=[])


def test_criterion_item_kind(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
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
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_launcher_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_unactivable=True),
        passing=[
            eve_booster_id,
            eve_drone_id,
            eve_fighter_id,
            eve_implant_id,
            eve_rig_id,
            eve_service_id,
            eve_subsystem_id],
        failing=[eve_charge_id, eve_module_id])
