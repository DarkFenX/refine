from tests import check_no_field


def test_bundled(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_vol_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_charge1_id = client.mk_eve_item(attrs={eve_vol_attr_id: 1})
    eve_charge2_id = client.mk_eve_item(attrs={eve_vol_attr_id: 1.2})
    eve_module_id = client.mk_eve_item(attrs={eve_cap_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_volume])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module1.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_volume])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module2 = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_volume])
    assert api_val.passed is False
    assert api_val.details.charge_volume == {api_module2.charge.id: (api_module2.id, 1.2, 1)}
    # Action
    api_module2.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_volume])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_separate(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_vol_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_charge1_id = client.mk_eve_item(attrs={eve_vol_attr_id: 1})
    eve_charge2_id = client.mk_eve_item(attrs={eve_vol_attr_id: 1.2})
    eve_module_id = client.mk_eve_item(attrs={eve_cap_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_volume])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_mod(charge=eve_charge1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_volume])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_mod(charge=eve_charge2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_volume])
    assert api_val.passed is False
    assert api_val.details.charge_volume == {api_module.charge.id: (api_module.id, 1.2, 1)}
    # Action
    api_module.change_mod(charge=None)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.charge_volume])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
