
def test_multiple(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_module_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.offline)
    api_module2 = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.offline)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_fitted])
    assert api_val.passed is False
    assert api_val.details.max_group_fitted == {eve_grp_id: [2, {api_module1.id: 1, api_module2.id: 1}]}


def test_mix(client, consts):
    eve_grp_id = client.mk_eve_item_group()
    eve_limit_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_group_fitted)
    eve_module1_id = client.mk_eve_item(grp_id=eve_grp_id)
    eve_module2_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 2})
    eve_module3_id = client.mk_eve_item(grp_id=eve_grp_id, attrs={eve_limit_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_module1_id, state=consts.ApiState.offline)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id, state=consts.ApiState.offline)
    api_fit.add_mod(type_id=eve_module3_id, state=consts.ApiState.offline)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.max_group_fitted])
    assert api_val.passed is False
    assert api_val.details.max_group_fitted == {eve_grp_id: [3, {api_module2.id: 2}]}
