from tests import check_no_field


def test_mismatch_group(client, consts):
    eve_allowed_grp_id = client.mk_eve_ship_group()
    eve_disallowed_grp_id = client.mk_eve_ship_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_allowed_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_disallowed_grp_id)
    eve_module_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_allowed_grp_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_disallowed_grp_id
    assert api_val.details.ship_limit.mismatches[api_module.id].allowed_type_ids == []
    assert api_val.details.ship_limit.mismatches[api_module.id].allowed_group_ids == [eve_allowed_grp_id]
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_mismatch_type(client, consts):
    eve_ship_grp_id = client.mk_eve_ship_group()
    eve_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_type1, unit_id=consts.EveAttrUnit.item_id)
    eve_allowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_disallowed_ship_id = client.mk_eve_ship(grp_id=eve_ship_grp_id)
    eve_module_id = client.mk_eve_item(attrs={eve_type_attr_id: eve_allowed_ship_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_disallowed_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_disallowed_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_ship_grp_id
    assert api_val.details.ship_limit.mismatches[api_module.id].allowed_type_ids == [eve_allowed_ship_id]
    assert api_val.details.ship_limit.mismatches[api_module.id].allowed_group_ids == []
    # Action
    api_fit.set_ship(type_id=eve_allowed_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
