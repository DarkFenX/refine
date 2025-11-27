from tests import Muta, approx, check_no_field
from tests.fw.api import ValOptions


def test_main_module(client, consts):
    eve_ban_empire_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_ban_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_hisec)
    eve_ban_hazard_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_hazard)
    eve_corrupt_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allow_in_fully_corrupted_hisec)
    eve_corrupt_lowsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allow_in_fully_corrupted_lowsec)
    eve_module1_id = client.mk_eve_item()
    eve_module2_id = client.mk_eve_item(attrs={eve_ban_empire_attr_id: 1})
    eve_module3_id = client.mk_eve_item(attrs={eve_ban_hisec_attr_id: 1})
    eve_module4_id = client.mk_eve_item(attrs={eve_ban_hazard_attr_id: 1})
    eve_module5_id = client.mk_eve_item(attrs={eve_corrupt_hisec_attr_id: 1})
    eve_module6_id = client.mk_eve_item(attrs={eve_corrupt_lowsec_attr_id: 1})
    eve_module7_id = client.mk_eve_item(attrs={eve_ban_empire_attr_id: 1, eve_ban_hisec_attr_id: 1})
    eve_module8_id = client.mk_eve_item(attrs={eve_ban_empire_attr_id: 1, eve_ban_hazard_attr_id: 1})
    eve_module9_id = client.mk_eve_item(attrs={eve_ban_empire_attr_id: 1, eve_corrupt_hisec_attr_id: 1})
    eve_module10_id = client.mk_eve_item(attrs={eve_ban_empire_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_module11_id = client.mk_eve_item(attrs={eve_ban_hisec_attr_id: 1, eve_ban_hazard_attr_id: 1})
    eve_module12_id = client.mk_eve_item(attrs={eve_ban_hisec_attr_id: 1, eve_corrupt_hisec_attr_id: 1})
    eve_module13_id = client.mk_eve_item(attrs={eve_ban_hisec_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_module14_id = client.mk_eve_item(attrs={eve_ban_hazard_attr_id: 1, eve_corrupt_hisec_attr_id: 1})
    eve_module15_id = client.mk_eve_item(attrs={eve_ban_hazard_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_module16_id = client.mk_eve_item(attrs={eve_corrupt_hisec_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_module17_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1})
    eve_module18_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_module19_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module20_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_module21_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module22_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module23_id = client.mk_eve_item(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_module24_id = client.mk_eve_item(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module25_id = client.mk_eve_item(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module26_id = client.mk_eve_item(attrs={
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module27_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_module28_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module29_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module30_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module31_id = client.mk_eve_item(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_module32_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.disabled)
    api_module2 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.disabled)
    api_module3 = api_fit.add_module(type_id=eve_module3_id, state=consts.ApiModuleState.disabled)
    api_module4 = api_fit.add_module(type_id=eve_module4_id, state=consts.ApiModuleState.disabled)
    api_fit.add_module(type_id=eve_module5_id, state=consts.ApiModuleState.disabled)
    api_fit.add_module(type_id=eve_module6_id, state=consts.ApiModuleState.disabled)
    api_module7 = api_fit.add_module(type_id=eve_module7_id, state=consts.ApiModuleState.disabled)
    api_module8 = api_fit.add_module(type_id=eve_module8_id, state=consts.ApiModuleState.disabled)
    api_module9 = api_fit.add_module(type_id=eve_module9_id, state=consts.ApiModuleState.disabled)
    api_module10 = api_fit.add_module(type_id=eve_module10_id, state=consts.ApiModuleState.disabled)
    api_module11 = api_fit.add_module(type_id=eve_module11_id, state=consts.ApiModuleState.disabled)
    api_module12 = api_fit.add_module(type_id=eve_module12_id, state=consts.ApiModuleState.disabled)
    api_module13 = api_fit.add_module(type_id=eve_module13_id, state=consts.ApiModuleState.disabled)
    api_module14 = api_fit.add_module(type_id=eve_module14_id, state=consts.ApiModuleState.disabled)
    api_module15 = api_fit.add_module(type_id=eve_module15_id, state=consts.ApiModuleState.disabled)
    api_fit.add_module(type_id=eve_module16_id, state=consts.ApiModuleState.disabled)
    api_module17 = api_fit.add_module(type_id=eve_module17_id, state=consts.ApiModuleState.disabled)
    api_module18 = api_fit.add_module(type_id=eve_module18_id, state=consts.ApiModuleState.disabled)
    api_module19 = api_fit.add_module(type_id=eve_module19_id, state=consts.ApiModuleState.disabled)
    api_module20 = api_fit.add_module(type_id=eve_module20_id, state=consts.ApiModuleState.disabled)
    api_module21 = api_fit.add_module(type_id=eve_module21_id, state=consts.ApiModuleState.disabled)
    api_module22 = api_fit.add_module(type_id=eve_module22_id, state=consts.ApiModuleState.disabled)
    api_module23 = api_fit.add_module(type_id=eve_module23_id, state=consts.ApiModuleState.disabled)
    api_module24 = api_fit.add_module(type_id=eve_module24_id, state=consts.ApiModuleState.disabled)
    api_module25 = api_fit.add_module(type_id=eve_module25_id, state=consts.ApiModuleState.disabled)
    api_module26 = api_fit.add_module(type_id=eve_module26_id, state=consts.ApiModuleState.disabled)
    api_module27 = api_fit.add_module(type_id=eve_module27_id, state=consts.ApiModuleState.disabled)
    api_module28 = api_fit.add_module(type_id=eve_module28_id, state=consts.ApiModuleState.disabled)
    api_module29 = api_fit.add_module(type_id=eve_module29_id, state=consts.ApiModuleState.disabled)
    api_module30 = api_fit.add_module(type_id=eve_module30_id, state=consts.ApiModuleState.disabled)
    api_module31 = api_fit.add_module(type_id=eve_module31_id, state=consts.ApiModuleState.disabled)
    api_module32 = api_fit.add_module(type_id=eve_module32_id, state=consts.ApiModuleState.disabled)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_unactivable.items == {
        api_module2.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_module3.id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module7.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_module8.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace.wspace]),
        api_module9.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module10.id: sorted([
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module11.id: sorted([consts.ApiSecZone.lowsec, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module12.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module13.id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module17.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module18.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module19.id: sorted([
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module20.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module21.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module22.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module23.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_module24.id: sorted([consts.ApiSecZone.lowsec, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module25.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module27.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module28.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module29.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module30.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_module31.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_module32.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace])}
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hisec_c5)
    # Verification - same as hisec, but all the items which could be in corrupted hisec are not
    # failing
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.hisec_c5
    assert api_val.details.sec_zone_unactivable.items == {
        api_module2.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_module3.id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module7.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_module8.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace.wspace]),
        api_module10.id: sorted([
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module11.id: sorted([consts.ApiSecZone.lowsec, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module13.id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module17.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module19.id: sorted([
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module21.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module24.id: sorted([consts.ApiSecZone.lowsec, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module28.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace])}
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.lowsec
    assert api_val.details.sec_zone_unactivable.items == {
        api_module2.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_module7.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_module8.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace.wspace]),
        api_module9.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module10.id: sorted([
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module17.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module18.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module19.id: sorted([
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module20.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module21.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module22.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module27.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module28.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module29.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module30.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_module32.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace])}
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec_c5)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.lowsec_c5
    assert api_val.details.sec_zone_unactivable.items == {
        api_module2.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_module7.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_module8.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace.wspace]),
        api_module9.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module17.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module18.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_module20.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module27.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace])}
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.wspace)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hazard)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.hazard
    assert api_val.details.sec_zone_unactivable.items == {
        api_module4.id: sorted([
            consts.ApiSecZone.hisec,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_module8.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace.wspace]),
        api_module11.id: sorted([consts.ApiSecZone.lowsec, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module14.id: sorted([
            consts.ApiSecZone.hisec,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_module15.id: sorted([
            consts.ApiSecZone.hisec,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_module17.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module20.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module21.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module23.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_module24.id: sorted([consts.ApiSecZone.lowsec, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module26.id: sorted([
            consts.ApiSecZone.hisec,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_module27.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module28.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_module30.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_module31.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_module32.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace])}


def test_charge(client, consts):
    eve_ban_empire_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_charge_id = client.mk_eve_item(attrs={eve_ban_empire_attr_id: 1})
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        charge_type_id=eve_charge_id,
        state=consts.ApiModuleState.disabled)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_unactivable.items == {
        api_module.charge.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    # Fast validator has KF check at the very end of logic chain, thus we check all those conditions
    # which end up with KF check
    eve_ban_empire_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_ban_hazard_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_hazard)
    eve_module1_id = client.mk_eve_item(attrs={eve_ban_empire_attr_id: 1})
    eve_module2_id = client.mk_eve_item(attrs={eve_ban_hazard_attr_id: 1})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_rig(type_id=eve_other_id)
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.disabled)
    api_module2 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.disabled)
    api_module3 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.disabled)
    api_module4 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.disabled)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module1.id])))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_unactivable.items == {
        api_module2.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_unactivable.items == {
        api_module1.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        sec_zone_unactivable=(True, [api_module1.id, api_other.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hisec_c5)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module1.id])))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.hisec_c5
    assert api_val.details.sec_zone_unactivable.items == {
        api_module2.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.hisec_c5
    assert api_val.details.sec_zone_unactivable.items == {
        api_module1.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        sec_zone_unactivable=(True, [api_module1.id, api_other.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module1.id])))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.lowsec
    assert api_val.details.sec_zone_unactivable.items == {
        api_module2.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.lowsec
    assert api_val.details.sec_zone_unactivable.items == {
        api_module1.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        sec_zone_unactivable=(True, [api_module1.id, api_other.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.lowsec_c5)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module1.id])))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.lowsec_c5
    assert api_val.details.sec_zone_unactivable.items == {
        api_module2.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.lowsec_c5
    assert api_val.details.sec_zone_unactivable.items == {
        api_module1.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module1.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        sec_zone_unactivable=(True, [api_module1.id, api_other.id, api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_sol.change(sec_zone=consts.ApiSecZone.hazard)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module3.id])))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.hazard
    assert api_val.details.sec_zone_unactivable.items == {api_module4.id: sorted([
        consts.ApiSecZone.hisec,
        consts.ApiSecZone.lowsec,
        consts.ApiSecZone.nullsec,
        consts.ApiSecZone.wspace])}
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module4.id])))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.hazard
    assert api_val.details.sec_zone_unactivable.items == {api_module3.id: sorted([
        consts.ApiSecZone.hisec,
        consts.ApiSecZone.lowsec,
        consts.ApiSecZone.nullsec,
        consts.ApiSecZone.wspace])}
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=(True, [api_module3.id, api_module4.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        sec_zone_unactivable=(True, [api_module3.id, api_other.id, api_module4.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_module1_id = client.mk_eve_item(attrs={eve_attr_id: 0})
    eve_module2_id = client.mk_eve_item(attrs={eve_attr_id: 1})
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
    assert api_module1.update().attrs[eve_attr_id].extra == approx(0)
    assert api_module2.update().attrs[eve_attr_id].extra == approx(0)
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_rig1.remove()
    api_fit.add_rig(type_id=eve_rig2_id)
    # Verification
    assert api_module1.update().attrs[eve_attr_id].extra == approx(1)
    assert api_module2.update().attrs[eve_attr_id].extra == approx(1)
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_unactivable.items == {
        api_module1.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_module2.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}


def test_mutation(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_base_module_id = client.mk_eve_item(attrs={eve_attr_id: 0})
    eve_mutated_module_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_attr_id: (0, 3)})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_base_module_id, state=consts.ApiModuleState.disabled)
    # Verification
    assert api_module.update().attrs[eve_attr_id].extra == approx(0)
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(mutation=eve_mutator_id)
    # Verification
    assert api_module.update().attrs[eve_attr_id].extra == approx(1)
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_unactivable.items == {
        api_module.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    # Action
    api_module.change_module(mutation={eve_attr_id: Muta.roll_to_api(val=0)})
    # Verification
    assert api_module.update().attrs[eve_attr_id].extra == approx(0)
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_module(mutation={eve_attr_id: Muta.roll_to_api(val=0.3)})
    # Verification
    assert api_module.update().attrs[eve_attr_id].extra == approx(0.9)
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_unactivable.items == {
        api_module.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    # Action
    api_module.change_module(mutation=None)
    # Verification
    assert api_module.update().attrs[eve_attr_id].extra == approx(0)
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_values(client, consts):
    # Check that only absence of attribute value, or value equal to 0 is treated as flag disabled
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_module1_id = client.mk_eve_item()
    eve_module2_id = client.mk_eve_item(attrs={eve_attr_id: 0})
    eve_module3_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_module4_id = client.mk_eve_item(attrs={eve_attr_id: 0.01})
    eve_module5_id = client.mk_eve_item(attrs={eve_attr_id: -0.01})
    eve_module6_id = client.mk_eve_item(attrs={eve_attr_id: -5000})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.disabled)
    api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.disabled)
    api_module3 = api_fit.add_module(type_id=eve_module3_id, state=consts.ApiModuleState.disabled)
    api_module4 = api_fit.add_module(type_id=eve_module4_id, state=consts.ApiModuleState.disabled)
    api_module5 = api_fit.add_module(type_id=eve_module5_id, state=consts.ApiModuleState.disabled)
    api_module6 = api_fit.add_module(type_id=eve_module6_id, state=consts.ApiModuleState.disabled)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_unactivable.zone == consts.ApiSecZone.lowsec
    assert api_val.details.sec_zone_unactivable.items == {
        api_module3.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_module4.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_module5.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_module6.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}


def test_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.disabled)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_kind(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_item_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_item_id, eve_attr_id: 1},
        eff_ids=[eve_autocharge_effect_id])
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.lowsec)
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_item_id)
    api_fit.set_character(type_id=eve_item_id)
    api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_item_id)
    api_fit.add_implant(type_id=eve_item_id)
    api_fit.add_rig(type_id=eve_item_id)
    api_fit.add_service(type_id=eve_item_id, state=consts.ApiServiceState.online)
    api_fit.set_ship(type_id=eve_item_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    api_fit.set_stance(type_id=eve_item_id)
    api_fit.add_subsystem(type_id=eve_item_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_val = api_fit.validate(options=ValOptions(sec_zone_unactivable=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
