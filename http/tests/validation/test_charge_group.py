from tests import check_no_field


def test_bundled(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_charge1_id = client.mk_eve_item(grp_id=eve_grp1_id)
    eve_charge2_id = client.mk_eve_item(grp_id=eve_grp2_id)
    eve_module_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module1.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module2 = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_group])
    assert api_val.passed is False
    assert api_val.details.charge_group == {api_module2.charge.id: (api_module2.id, eve_grp2_id, [eve_grp1_id])}
    # Action
    api_module2.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_separate(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_group_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_charge1_id = client.mk_eve_item(grp_id=eve_grp1_id)
    eve_charge2_id = client.mk_eve_item(grp_id=eve_grp2_id)
    eve_module_id = client.mk_eve_item(attrs={eve_group_attr_id: eve_grp1_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module.change_mod(charge=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module.change_mod(charge=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_group])
    assert api_val.passed is False
    assert api_val.details.charge_group == {api_module.charge.id: (api_module.id, eve_grp2_id, [eve_grp1_id])}
    # Action
    api_module.change_mod(charge=None)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_multiple_different(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_grp3_id = client.mk_eve_item_group()
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group3, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group4, unit_id=consts.EveAttrUnit.group_id)
    eve_charge1_id = client.mk_eve_item(grp_id=eve_grp1_id)
    eve_charge2_id = client.mk_eve_item(grp_id=eve_grp2_id)
    eve_charge3_id = client.mk_eve_item(grp_id=eve_grp3_id)
    eve_module_id = client.mk_eve_item(attrs={eve_group_attr1_id: eve_grp1_id, eve_group_attr2_id: eve_grp2_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module.change_mod(charge=eve_charge3_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_group])
    assert api_val.passed is False
    assert api_val.details.charge_group == {
        api_module.charge.id: (api_module.id, eve_grp3_id, [eve_grp1_id, eve_grp2_id])}
    # Action
    api_module.change_mod(charge=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_multiple_same(client, consts):
    eve_grp1_id = client.mk_eve_item_group()
    eve_grp2_id = client.mk_eve_item_group()
    eve_grp3_id = client.mk_eve_item_group()
    eve_group_attr1_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group1, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr2_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group2, unit_id=consts.EveAttrUnit.group_id)
    eve_group_attr3_id = client.mk_eve_attr(id_=consts.EveAttr.charge_group3, unit_id=consts.EveAttrUnit.group_id)
    eve_charge1_id = client.mk_eve_item(grp_id=eve_grp1_id)
    eve_charge2_id = client.mk_eve_item(grp_id=eve_grp2_id)
    eve_charge3_id = client.mk_eve_item(grp_id=eve_grp3_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_group_attr1_id: eve_grp1_id, eve_group_attr2_id: eve_grp2_id, eve_group_attr3_id: eve_grp1_id})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_module.change_mod(charge=eve_charge3_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_group])
    assert api_val.passed is False
    assert api_val.details.charge_group == {
        api_module.charge.id: (api_module.id, eve_grp3_id, [eve_grp1_id, eve_grp2_id])}
    # Action
    api_module.change_mod(charge=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_group])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
