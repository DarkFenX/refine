
def test_mismatch_group(client, consts):
    eve_grp1_id = client.mk_eve_ship_group()
    eve_grp2_id = client.mk_eve_ship_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.can_fit_ship_group1)
    eve_module_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id})
    eve_ship_id = client.mk_eve_ship(grp_id=eve_grp2_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.ship_limit])
    assert api_val.passed is False
    assert api_val.details.ship_limit.ship_type_id == eve_ship_id
    assert api_val.details.ship_limit.ship_group_id == eve_grp2_id
