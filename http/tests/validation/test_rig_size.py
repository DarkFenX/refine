
def test_fail(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.rig_size)
    eve_rig_id = client.mk_eve_item(attrs={eve_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.rig_size])
    assert api_val.passed is False
    assert api_val.details.rig_size.allowed_size == 3
    assert api_val.details.rig_size.mismatches == {api_rig.id: 1}
