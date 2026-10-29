from fw import approx
from fw.api import ValOptions


def test_flags_service(client, consts):
    eve_ban_empire_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_ban_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_hisec)
    eve_ban_hazard_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_hazard)
    eve_corrupt_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allow_in_fully_corrupted_hisec)
    eve_corrupt_lowsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allow_in_fully_corrupted_lowsec)
    eve_service1_id = client.mk_eve_service()
    eve_service2_id = client.mk_eve_service(attrs={eve_ban_empire_attr_id: 1})
    eve_service3_id = client.mk_eve_service(attrs={eve_ban_hisec_attr_id: 1})
    eve_service4_id = client.mk_eve_service(attrs={eve_ban_hazard_attr_id: 1})
    eve_service5_id = client.mk_eve_service(attrs={eve_corrupt_hisec_attr_id: 1})
    eve_service6_id = client.mk_eve_service(attrs={eve_corrupt_lowsec_attr_id: 1})
    eve_service7_id = client.mk_eve_service(attrs={eve_ban_empire_attr_id: 1, eve_ban_hisec_attr_id: 1})
    eve_service8_id = client.mk_eve_service(attrs={eve_ban_empire_attr_id: 1, eve_ban_hazard_attr_id: 1})
    eve_service9_id = client.mk_eve_service(attrs={eve_ban_empire_attr_id: 1, eve_corrupt_hisec_attr_id: 1})
    eve_service10_id = client.mk_eve_service(attrs={eve_ban_empire_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_service11_id = client.mk_eve_service(attrs={eve_ban_hisec_attr_id: 1, eve_ban_hazard_attr_id: 1})
    eve_service12_id = client.mk_eve_service(attrs={eve_ban_hisec_attr_id: 1, eve_corrupt_hisec_attr_id: 1})
    eve_service13_id = client.mk_eve_service(attrs={eve_ban_hisec_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_service14_id = client.mk_eve_service(attrs={eve_ban_hazard_attr_id: 1, eve_corrupt_hisec_attr_id: 1})
    eve_service15_id = client.mk_eve_service(attrs={eve_ban_hazard_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_service16_id = client.mk_eve_service(attrs={eve_corrupt_hisec_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_service17_id = client.mk_eve_service(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1})
    eve_service18_id = client.mk_eve_service(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_service19_id = client.mk_eve_service(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service20_id = client.mk_eve_service(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_service21_id = client.mk_eve_service(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service22_id = client.mk_eve_service(attrs={
        eve_ban_empire_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service23_id = client.mk_eve_service(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_service24_id = client.mk_eve_service(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service25_id = client.mk_eve_service(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service26_id = client.mk_eve_service(attrs={
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service27_id = client.mk_eve_service(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_service28_id = client.mk_eve_service(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service29_id = client.mk_eve_service(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service30_id = client.mk_eve_service(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service31_id = client.mk_eve_service(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service32_id = client.mk_eve_service(attrs={
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
        val_options=ValOptions(sec_zone_fitted=True),
        passing=[
            eve_service1_id,
            eve_service4_id,
            eve_service5_id,
            eve_service6_id,
            eve_service14_id,
            eve_service15_id,
            eve_service16_id,
            eve_service26_id],
        failing=[
            eve_service2_id,
            eve_service3_id,
            eve_service7_id,
            eve_service8_id,
            eve_service9_id,
            eve_service10_id,
            eve_service11_id,
            eve_service12_id,
            eve_service13_id,
            eve_service17_id,
            eve_service18_id,
            eve_service19_id,
            eve_service20_id,
            eve_service21_id,
            eve_service22_id,
            eve_service23_id,
            eve_service24_id,
            eve_service25_id,
            eve_service27_id,
            eve_service28_id,
            eve_service29_id,
            eve_service30_id,
            eve_service31_id,
            eve_service32_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hisec_c5)
    # Verification - same as hisec, but all the items which could be in corrupted hisec are not
    # failing
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=True),
        passing=[
            eve_service1_id,
            eve_service4_id,
            eve_service5_id,
            eve_service6_id,
            eve_service9_id,
            eve_service12_id,
            eve_service14_id,
            eve_service15_id,
            eve_service16_id,
            eve_service18_id,
            eve_service20_id,
            eve_service22_id,
            eve_service23_id,
            eve_service25_id,
            eve_service26_id,
            eve_service27_id,
            eve_service29_id,
            eve_service30_id,
            eve_service31_id,
            eve_service32_id],
        failing=[
            eve_service2_id,
            eve_service3_id,
            eve_service7_id,
            eve_service8_id,
            eve_service10_id,
            eve_service11_id,
            eve_service13_id,
            eve_service17_id,
            eve_service19_id,
            eve_service21_id,
            eve_service24_id,
            eve_service28_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=True),
        passing=[
            eve_service1_id,
            eve_service3_id,
            eve_service4_id,
            eve_service5_id,
            eve_service6_id,
            eve_service11_id,
            eve_service12_id,
            eve_service13_id,
            eve_service14_id,
            eve_service15_id,
            eve_service16_id,
            eve_service23_id,
            eve_service24_id,
            eve_service25_id,
            eve_service26_id,
            eve_service31_id],
        failing=[
            eve_service2_id,
            eve_service7_id,
            eve_service8_id,
            eve_service9_id,
            eve_service10_id,
            eve_service17_id,
            eve_service18_id,
            eve_service19_id,
            eve_service20_id,
            eve_service21_id,
            eve_service22_id,
            eve_service27_id,
            eve_service28_id,
            eve_service29_id,
            eve_service30_id,
            eve_service32_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec_c5)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=True),
        passing=[
            eve_service1_id,
            eve_service3_id,
            eve_service4_id,
            eve_service5_id,
            eve_service6_id,
            eve_service10_id,
            eve_service11_id,
            eve_service12_id,
            eve_service13_id,
            eve_service14_id,
            eve_service15_id,
            eve_service16_id,
            eve_service19_id,
            eve_service21_id,
            eve_service22_id,
            eve_service23_id,
            eve_service24_id,
            eve_service25_id,
            eve_service26_id,
            eve_service28_id,
            eve_service29_id,
            eve_service30_id,
            eve_service31_id,
            eve_service32_id],
        failing=[
            eve_service2_id,
            eve_service7_id,
            eve_service8_id,
            eve_service9_id,
            eve_service17_id,
            eve_service18_id,
            eve_service20_id,
            eve_service27_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=True),
        passing=[
            eve_service1_id,
            eve_service2_id,
            eve_service3_id,
            eve_service4_id,
            eve_service5_id,
            eve_service6_id,
            eve_service7_id,
            eve_service8_id,
            eve_service9_id,
            eve_service10_id,
            eve_service11_id,
            eve_service12_id,
            eve_service13_id,
            eve_service14_id,
            eve_service15_id,
            eve_service16_id,
            eve_service17_id,
            eve_service18_id,
            eve_service19_id,
            eve_service20_id,
            eve_service21_id,
            eve_service22_id,
            eve_service23_id,
            eve_service24_id,
            eve_service25_id,
            eve_service26_id,
            eve_service27_id,
            eve_service28_id,
            eve_service29_id,
            eve_service30_id,
            eve_service31_id,
            eve_service32_id],
        failing=[])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.wspace)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=True),
        passing=[
            eve_service1_id,
            eve_service2_id,
            eve_service3_id,
            eve_service4_id,
            eve_service5_id,
            eve_service6_id,
            eve_service7_id,
            eve_service8_id,
            eve_service9_id,
            eve_service10_id,
            eve_service11_id,
            eve_service12_id,
            eve_service13_id,
            eve_service14_id,
            eve_service15_id,
            eve_service16_id,
            eve_service17_id,
            eve_service18_id,
            eve_service19_id,
            eve_service20_id,
            eve_service21_id,
            eve_service22_id,
            eve_service23_id,
            eve_service24_id,
            eve_service25_id,
            eve_service26_id,
            eve_service27_id,
            eve_service28_id,
            eve_service29_id,
            eve_service30_id,
            eve_service31_id,
            eve_service32_id],
        failing=[])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hazard)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=True),
        passing=[
            eve_service1_id,
            eve_service2_id,
            eve_service3_id,
            eve_service5_id,
            eve_service6_id,
            eve_service7_id,
            eve_service9_id,
            eve_service10_id,
            eve_service12_id,
            eve_service13_id,
            eve_service16_id,
            eve_service18_id,
            eve_service19_id,
            eve_service22_id,
            eve_service25_id,
            eve_service29_id],
        failing=[
            eve_service4_id,
            eve_service8_id,
            eve_service11_id,
            eve_service14_id,
            eve_service15_id,
            eve_service17_id,
            eve_service20_id,
            eve_service21_id,
            eve_service23_id,
            eve_service24_id,
            eve_service26_id,
            eve_service27_id,
            eve_service28_id,
            eve_service30_id,
            eve_service31_id,
            eve_service32_id])


def test_flags_rig(client, consts):
    eve_ban_empire_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_rig_id = client.mk_eve_rig(attrs={eve_ban_empire_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(sec_zone_fitted=True), passing=[], failing=[eve_rig_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_fit.check_try_fit_items(val_options=ValOptions(sec_zone_fitted=True), passing=[eve_rig_id], failing=[])


def test_known_failures(client, consts):
    # Fast validator has KF check at the very end of logic chain, thus we check all those conditions
    # which end up with KF check
    eve_ban_empire_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_ban_hazard_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_hazard)
    eve_service1_id = client.mk_eve_service(attrs={eve_ban_empire_attr_id: 1})
    eve_service2_id = client.mk_eve_service(attrs={eve_ban_hazard_attr_id: 1})
    eve_ship_id = client.mk_eve_ship()
    client.mk_eve_item_list(id_=consts.EveItemList.wormhole_jump_blacklist, inc_type_ids=[eve_ship_id])
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_service1 = api_fit.add_service(type_id=eve_service1_id)
    api_service2 = api_fit.add_service(type_id=eve_service1_id)
    api_service3 = api_fit.add_service(type_id=eve_service2_id)
    api_service4 = api_fit.add_service(type_id=eve_service2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service1.id]),
        passing=[],
        failing=[eve_service2_id, eve_service1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service2.id]),
        passing=[],
        failing=[eve_service2_id, eve_service1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service1.id, api_service2.id]),
        passing=[eve_service2_id],
        failing=[eve_service1_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hisec_c5)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service1.id]),
        passing=[],
        failing=[eve_service2_id, eve_service1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service2.id]),
        passing=[],
        failing=[eve_service2_id, eve_service1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service1.id, api_service2.id]),
        passing=[eve_service2_id],
        failing=[eve_service1_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service1.id]),
        passing=[],
        failing=[eve_service2_id, eve_service1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service2.id]),
        passing=[],
        failing=[eve_service2_id, eve_service1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service1.id, api_service2.id]),
        passing=[eve_service2_id],
        failing=[eve_service1_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec_c5)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service1.id]),
        passing=[],
        failing=[eve_service2_id, eve_service1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service2.id]),
        passing=[],
        failing=[eve_service2_id, eve_service1_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service1.id, api_service2.id]),
        passing=[eve_service2_id],
        failing=[eve_service1_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hazard)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service3.id]),
        passing=[],
        failing=[eve_service1_id, eve_service2_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service4.id]),
        passing=[],
        failing=[eve_service1_id, eve_service2_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_service3.id, api_service4.id]),
        passing=[eve_service1_id],
        failing=[eve_service2_id])
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.wspace)
    # Verification - ship is the only item which can fail in w-space
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=True),
        passing=[],
        failing=[eve_service1_id, eve_service2_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=[api_ship.id]),
        passing=[eve_service1_id, eve_service2_id],
        failing=[])


def test_modified(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_service1_id = client.mk_eve_service(attrs={eve_attr_id: 0})
    eve_service2_id = client.mk_eve_service(attrs={eve_attr_id: 1})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig1_id = client.mk_eve_item(attrs={eve_mod_attr_id: 0}, eff_ids=[eve_effect_id])
    eve_rig2_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig1_id)
    api_service1 = api_fit.add_service(type_id=eve_service1_id)
    api_service2 = api_fit.add_service(type_id=eve_service2_id)
    # Verification
    assert api_service1.update().attrs[eve_attr_id].modified == approx(0)
    assert api_service2.update().attrs[eve_attr_id].modified == approx(0)
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=True),
        passing=[eve_service1_id, eve_service2_id],
        failing=[])
    # Action - fitted services would fail validation after the modification, so they are removed
    api_service1.remove()
    api_service2.remove()
    api_rig1.remove()
    api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=True),
        passing=[],
        failing=[eve_service1_id, eve_service2_id])


def test_values(client, consts):
    # Check that only absence of attribute value, or value equal to 0 is treated as flag disabled
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_service1_id = client.mk_eve_service()
    eve_service2_id = client.mk_eve_service(attrs={eve_attr_id: 0})
    eve_service3_id = client.mk_eve_service(attrs={eve_attr_id: 1})
    eve_service4_id = client.mk_eve_service(attrs={eve_attr_id: 0.01})
    eve_service5_id = client.mk_eve_service(attrs={eve_attr_id: -0.01})
    eve_service6_id = client.mk_eve_service(attrs={eve_attr_id: -5000})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(sec_zone_fitted=True),
        passing=[eve_service1_id, eve_service2_id],
        failing=[eve_service3_id, eve_service4_id, eve_service5_id, eve_service6_id])


def test_no_attr(client, consts):
    eve_attr_id = consts.EveAttr.disallow_in_empire_space
    eve_service_id = client.mk_eve_service(attrs={eve_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    # Verification - when attribute can't be fetched, it's assumed to be non-limited in any sec zone
    api_fit.check_try_fit_items(val_options=ValOptions(sec_zone_fitted=True), passing=[eve_service_id], failing=[])


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
        val_options=ValOptions(sec_zone_fitted=True),
        passing=[
            eve_booster_id,
            eve_charge_id,
            eve_drone_id,
            eve_fighter_id,
            eve_implant_id,
            eve_module_id,
            eve_subsystem_id],
        failing=[eve_rig_id, eve_service_id])
