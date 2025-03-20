from tests import approx, check_no_field
from tests.fw.api import ValOptions


def test_main_service(client, consts):
    eve_ban_empire_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_ban_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_hisec)
    eve_ban_hazard_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_hazard)
    eve_corrupt_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allow_in_fully_corrupted_hisec)
    eve_corrupt_lowsec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.allow_in_fully_corrupted_lowsec)
    eve_service1_id = client.mk_eve_item()
    eve_service2_id = client.mk_eve_item(attrs={eve_ban_empire_attr_id: 1})
    eve_service3_id = client.mk_eve_item(attrs={eve_ban_hisec_attr_id: 1})
    eve_service4_id = client.mk_eve_item(attrs={eve_ban_hazard_attr_id: 1})
    eve_service5_id = client.mk_eve_item(attrs={eve_corrupt_hisec_attr_id: 1})
    eve_service6_id = client.mk_eve_item(attrs={eve_corrupt_lowsec_attr_id: 1})
    eve_service7_id = client.mk_eve_item(attrs={eve_ban_empire_attr_id: 1, eve_ban_hisec_attr_id: 1})
    eve_service8_id = client.mk_eve_item(attrs={eve_ban_empire_attr_id: 1, eve_ban_hazard_attr_id: 1})
    eve_service9_id = client.mk_eve_item(attrs={eve_ban_empire_attr_id: 1, eve_corrupt_hisec_attr_id: 1})
    eve_service10_id = client.mk_eve_item(attrs={eve_ban_empire_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_service11_id = client.mk_eve_item(attrs={eve_ban_hisec_attr_id: 1, eve_ban_hazard_attr_id: 1})
    eve_service12_id = client.mk_eve_item(attrs={eve_ban_hisec_attr_id: 1, eve_corrupt_hisec_attr_id: 1})
    eve_service13_id = client.mk_eve_item(attrs={eve_ban_hisec_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_service14_id = client.mk_eve_item(attrs={eve_ban_hazard_attr_id: 1, eve_corrupt_hisec_attr_id: 1})
    eve_service15_id = client.mk_eve_item(attrs={eve_ban_hazard_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_service16_id = client.mk_eve_item(attrs={eve_corrupt_hisec_attr_id: 1, eve_corrupt_lowsec_attr_id: 1})
    eve_service17_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1})
    eve_service18_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_service19_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service20_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_service21_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service22_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service23_id = client.mk_eve_item(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_service24_id = client.mk_eve_item(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service25_id = client.mk_eve_item(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service26_id = client.mk_eve_item(attrs={
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service27_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1})
    eve_service28_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service29_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service30_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service31_id = client.mk_eve_item(attrs={
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    eve_service32_id = client.mk_eve_item(attrs={
        eve_ban_empire_attr_id: 1,
        eve_ban_hisec_attr_id: 1,
        eve_ban_hazard_attr_id: 1,
        eve_corrupt_hisec_attr_id: 1,
        eve_corrupt_lowsec_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.add_service(type_id=eve_service1_id)
    api_service2 = api_fit.add_service(type_id=eve_service2_id)
    api_service3 = api_fit.add_service(type_id=eve_service3_id)
    api_service4 = api_fit.add_service(type_id=eve_service4_id)
    api_fit.add_service(type_id=eve_service5_id)
    api_fit.add_service(type_id=eve_service6_id)
    api_service7 = api_fit.add_service(type_id=eve_service7_id)
    api_service8 = api_fit.add_service(type_id=eve_service8_id)
    api_service9 = api_fit.add_service(type_id=eve_service9_id)
    api_service10 = api_fit.add_service(type_id=eve_service10_id)
    api_service11 = api_fit.add_service(type_id=eve_service11_id)
    api_service12 = api_fit.add_service(type_id=eve_service12_id)
    api_service13 = api_fit.add_service(type_id=eve_service13_id)
    api_service14 = api_fit.add_service(type_id=eve_service14_id)
    api_service15 = api_fit.add_service(type_id=eve_service15_id)
    api_fit.add_service(type_id=eve_service16_id)
    api_service17 = api_fit.add_service(type_id=eve_service17_id)
    api_service18 = api_fit.add_service(type_id=eve_service18_id)
    api_service19 = api_fit.add_service(type_id=eve_service19_id)
    api_service20 = api_fit.add_service(type_id=eve_service20_id)
    api_service21 = api_fit.add_service(type_id=eve_service21_id)
    api_service22 = api_fit.add_service(type_id=eve_service22_id)
    api_service23 = api_fit.add_service(type_id=eve_service23_id)
    api_service24 = api_fit.add_service(type_id=eve_service24_id)
    api_service25 = api_fit.add_service(type_id=eve_service25_id)
    api_service26 = api_fit.add_service(type_id=eve_service26_id)
    api_service27 = api_fit.add_service(type_id=eve_service27_id)
    api_service28 = api_fit.add_service(type_id=eve_service28_id)
    api_service29 = api_fit.add_service(type_id=eve_service29_id)
    api_service30 = api_fit.add_service(type_id=eve_service30_id)
    api_service31 = api_fit.add_service(type_id=eve_service31_id)
    api_service32 = api_fit.add_service(type_id=eve_service32_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_fitted=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_fitted.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_fitted.items == {
        api_service2.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_service3.id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service7.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_service8.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace.wspace]),
        api_service9.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service10.id: sorted([
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service11.id: sorted([consts.ApiSecZone.lowsec, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service12.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service13.id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service17.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service18.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service19.id: sorted([
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service20.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service21.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service22.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service23.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_service24.id: sorted([consts.ApiSecZone.lowsec, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service25.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service27.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service28.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service29.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service30.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_service31.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_service32.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace])}
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.hisec_c5)
    # Verification - same as hisec, but all the items which could be in corrupted hisec are not
    # failing
    api_val = api_fit.validate(options=ValOptions(sec_zone_fitted=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_fitted.zone == consts.ApiSecZone.hisec_c5
    assert api_val.details.sec_zone_fitted.items == {
        api_service2.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_service3.id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service7.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_service8.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace.wspace]),
        api_service10.id: sorted([
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service11.id: sorted([consts.ApiSecZone.lowsec, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service13.id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service17.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service19.id: sorted([
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service21.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service24.id: sorted([consts.ApiSecZone.lowsec, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service28.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace])}
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_fitted=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_fitted.zone == consts.ApiSecZone.lowsec
    assert api_val.details.sec_zone_fitted.items == {
        api_service2.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_service7.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_service8.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace.wspace]),
        api_service9.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service10.id: sorted([
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service17.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service18.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service19.id: sorted([
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service20.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service21.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service22.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service27.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service28.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service29.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service30.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_service32.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace])}
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.lowsec_c5)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_fitted=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_fitted.zone == consts.ApiSecZone.lowsec_c5
    assert api_val.details.sec_zone_fitted.items == {
        api_service2.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_service7.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_service8.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace.wspace]),
        api_service9.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service17.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service18.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_service20.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service27.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace])}
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_fitted=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.wspace)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_fitted=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.hazard)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_fitted=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_fitted.zone == consts.ApiSecZone.hazard
    assert api_val.details.sec_zone_fitted.items == {
        api_service4.id: sorted([
            consts.ApiSecZone.hisec,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_service8.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace.wspace]),
        api_service11.id: sorted([consts.ApiSecZone.lowsec, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service14.id: sorted([
            consts.ApiSecZone.hisec,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_service15.id: sorted([
            consts.ApiSecZone.hisec,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_service17.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service20.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service21.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service23.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_service24.id: sorted([consts.ApiSecZone.lowsec, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service26.id: sorted([
            consts.ApiSecZone.hisec,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_service27.id: sorted([consts.ApiSecZone.hisec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service28.id: sorted([consts.ApiSecZone.lowsec_c5, consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace]),
        api_service30.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_service31.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace]),
        api_service32.id: sorted([
            consts.ApiSecZone.hisec_c5,
            consts.ApiSecZone.lowsec_c5,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace])}


def test_rig(client, consts):
    eve_ban_empire_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_empire_space)
    eve_rig_id = client.mk_eve_item(attrs={eve_ban_empire_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_fitted=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_fitted.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_fitted.items == {
        api_rig.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_fitted=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_ship(client, consts):
    eve_ban_hisec_attr_id = client.mk_eve_attr(id_=consts.EveAttr.disallow_in_hisec)
    eve_ship_id = client.mk_eve_ship(attrs={eve_ban_hisec_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_fitted=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_fitted.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_fitted.items == {api_ship.id: sorted([
        consts.ApiSecZone.lowsec,
        consts.ApiSecZone.nullsec,
        consts.ApiSecZone.wspace,
        consts.ApiSecZone.hazard])}
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_fitted=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
