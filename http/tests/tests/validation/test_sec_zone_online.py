from tests import approx, check_no_field
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


def test_known_failures(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class)
    eve_hisec_service_id = client.mk_eve_item(attrs={eve_attr_id: 2})
    eve_lowsec_service_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_null_service_id = client.mk_eve_item(attrs={eve_attr_id: 0})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_rig(type_id=eve_other_id)
    api_hisec_service = api_fit.add_service(type_id=eve_hisec_service_id, state=consts.ApiServiceState.online)
    api_lowsec_service = api_fit.add_service(type_id=eve_lowsec_service_id, state=consts.ApiServiceState.online)
    api_nullsec_service = api_fit.add_service(type_id=eve_null_service_id, state=consts.ApiServiceState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=(True, [api_lowsec_service.id])))
    assert api_val.passed is False
    assert api_val.details.sec_zone_online.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_online.items == {
        api_nullsec_service.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=(True, [api_nullsec_service.id])))
    assert api_val.passed is False
    assert api_val.details.sec_zone_online.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_online.items == {api_lowsec_service.id: sorted([
        consts.ApiSecZone.lowsec,
        consts.ApiSecZone.nullsec,
        consts.ApiSecZone.wspace,
        consts.ApiSecZone.hazard])}
    api_val = api_fit.validate(options=ValOptions(
        sec_zone_online=(True, [api_lowsec_service.id, api_nullsec_service.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        sec_zone_online=(True, [api_lowsec_service.id, api_other.id, api_nullsec_service.id, api_hisec_service.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_rounding(client, consts):
    # There is no rounding for this validator, "higher" item is considered as lowsec-able, while
    # "lower" one is null and alikes only
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class)
    eve_higher_service_id = client.mk_eve_item(attrs={eve_attr_id: 1.01})
    eve_lower_service_id = client.mk_eve_item(attrs={eve_attr_id: 0.99})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_higher_service = api_fit.add_service(type_id=eve_higher_service_id, state=consts.ApiServiceState.online)
    api_lower_service = api_fit.add_service(type_id=eve_lower_service_id, state=consts.ApiServiceState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_online.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_online.items == {
        api_higher_service.id: sorted([
            consts.ApiSecZone.lowsec,
            consts.ApiSecZone.nullsec,
            consts.ApiSecZone.wspace,
            consts.ApiSecZone.hazard]),
        api_lower_service.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}
    # Action
    api_sol.set_sec_zone(sec_zone=consts.ApiSecZone.lowsec)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_online.zone == consts.ApiSecZone.lowsec
    assert api_val.details.sec_zone_online.items == {
        api_lower_service.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}


def test_modified(client, consts):
    # Unrealistic scenario, since this attribute doesn't seem to be modified by anything
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class)
    eve_service_id = client.mk_eve_item(attrs={eve_attr_id: 1})
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
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_fit.add_rig(type_id=eve_rig_id)
    api_service = api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.online)
    # Verification - modification is ignored for the validation purposes
    assert api_service.update().attrs[eve_attr_id].extra == approx(2)
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_online.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_online.items == {api_service.id: sorted([
        consts.ApiSecZone.lowsec,
        consts.ApiSecZone.nullsec,
        consts.ApiSecZone.wspace,
        consts.ApiSecZone.hazard])}


def test_mutation_limit_priority(client, consts):
    # Unrealistic scenario, only standup modules/services uses it, and it can't be mutated
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class)
    eve_base_module_id = client.mk_eve_item(attrs={eve_attr_id: 0})
    eve_mutated_module_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_attr_id: (1, 3)})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_base_module_id,
        state=consts.ApiServiceState.online,
        mutation=(eve_mutator_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 1}}))
    # Verification
    assert api_module.update().attrs[eve_attr_id].extra == approx(3)
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_online.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_online.items == {api_module.id: sorted([
        consts.ApiSecZone.lowsec,
        consts.ApiSecZone.nullsec,
        consts.ApiSecZone.wspace,
        consts.ApiSecZone.hazard])}
    # Action
    api_module.change_module(mutation=None)
    # Verification
    assert api_module.update().attrs[eve_attr_id].extra == approx(0)
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_online.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_online.items == {
        api_module.id: sorted([consts.ApiSecZone.nullsec, consts.ApiSecZone.wspace, consts.ApiSecZone.hazard])}


def test_mutation_limit_inheritance(client, consts):
    # Unrealistic scenario, only standup modules/services uses it, and it can't be mutated
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class)
    eve_base_module_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_mutated_module_id = client.mk_eve_item()
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_module_id], eve_mutated_module_id)],
        attrs={eve_attr_id: (1, 3)})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_base_module_id,
        state=consts.ApiServiceState.online,
        mutation=(eve_mutator_id, {eve_attr_id: {consts.ApiAttrMutation.roll: 1}}))
    # Verification
    assert api_module.update().attrs[eve_attr_id].extra == approx(3)
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_online.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_online.items == {api_module.id: sorted([
        consts.ApiSecZone.lowsec,
        consts.ApiSecZone.nullsec,
        consts.ApiSecZone.wspace,
        consts.ApiSecZone.hazard])}
    # Action
    api_module.change_module(mutation=None)
    # Verification
    assert api_module.update().attrs[eve_attr_id].extra == approx(1)
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_online.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_online.items == {api_module.id: sorted([
        consts.ApiSecZone.lowsec,
        consts.ApiSecZone.nullsec,
        consts.ApiSecZone.wspace,
        consts.ApiSecZone.hazard])}


def test_no_value(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class, def_val=0)
    eve_service_id = client.mk_eve_item()
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class, def_val=0)
    eve_service_id = client.alloc_item_id()
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_attr(client, consts):
    eve_attr_id = consts.EveAttr.online_max_security_class
    eve_service_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_service = api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is False
    assert api_val.details.sec_zone_online.zone == consts.ApiSecZone.hisec
    assert api_val.details.sec_zone_online.items == {api_service.id: sorted([
        consts.ApiSecZone.lowsec,
        consts.ApiSecZone.nullsec,
        consts.ApiSecZone.wspace,
        consts.ApiSecZone.hazard])}


def test_criterion_state(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class)
    eve_service_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.add_service(type_id=eve_service_id, state=consts.ApiServiceState.offline)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_item_kind(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.online_max_security_class)
    eve_rig_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol(sec_zone=consts.ApiSecZone.hisec)
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(sec_zone_online=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
