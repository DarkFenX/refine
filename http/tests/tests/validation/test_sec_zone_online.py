from tests import check_no_field
from tests.fw.api import ValOptions


def test_main(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class)
    eve_hisec_item_id = client.mk_eve_item(attrs={eve_attr_id: 2})
    eve_lowsec_item_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_null_item_id = client.mk_eve_item(attrs={eve_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_hisec_item_id, state=consts.ApiModuleState.online)
    api_lowsec_module = api_fit.add_module(type_id=eve_lowsec_item_id, state=consts.ApiModuleState.online)
    api_nullsec_module = api_fit.add_module(type_id=eve_null_item_id, state=consts.ApiModuleState.online)
    api_fit.add_service(type_id=eve_hisec_item_id, state=consts.ApiServiceState.online)
    api_lowsec_service = api_fit.add_service(type_id=eve_lowsec_item_id, state=consts.ApiServiceState.online)
    api_nullsec_service = api_fit.add_service(type_id=eve_null_item_id, state=consts.ApiServiceState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_online.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_online.items == {
        api_lowsec_module.id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_nullsec_module.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_lowsec_service.id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_nullsec_service.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.hisec_c5)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_online.zone == consts.ApiSecZone.hisec_c5
    assert api_val.details.sec_zone_online.items == {
        api_lowsec_module.id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_nullsec_module.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_lowsec_service.id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_nullsec_service.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_online.zone == consts.ApiSecZone.lowsec
    assert api_val.details.sec_zone_online.items == {
        api_nullsec_module.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_nullsec_service.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.lowsec_c5)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_online.zone == consts.ApiSecZone.lowsec_c5
    assert api_val.details.sec_zone_online.items == {
        api_nullsec_module.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard]),
        api_nullsec_service.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.nullsec)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.wspace)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.hazard)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
