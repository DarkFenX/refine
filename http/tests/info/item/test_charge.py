from pytest import raises


def test_bundled(client, consts):
    eve_module = client.mk_eve_item()
    eve_charge = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_module = api_fit.add_mod(type_id=eve_module.id, charge_type_id=eve_charge.id)
    assert isinstance(api_module.id, str)
    assert isinstance(api_module.charge_id, str)
    # ID only
    api_module.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert isinstance(api_module.id, str)
    assert isinstance(api_module.charge_id, str)
    # Partial
    api_module.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert isinstance(api_module.id, str)
    assert isinstance(api_module.charge.id, str)
    # Full
    api_module.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert isinstance(api_module.id, str)
    assert isinstance(api_module.charge.id, str)


def test_separate(client, consts):
    eve_module = client.mk_eve_item()
    eve_charge = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_module = api_fit.add_mod(type_id=eve_module.id)
    assert isinstance(api_module.id, str)
    with raises(AttributeError):
        api_module.charge_id  # pylint: disable=W0104
    # ID only
    api_module.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert isinstance(api_module.id, str)
    with raises(AttributeError):
        api_module.charge_id  # pylint: disable=W0104
    # Partial
    api_module.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert isinstance(api_module.id, str)
    with raises(AttributeError):
        api_module.charge  # pylint: disable=W0104
    # Full
    api_module.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert isinstance(api_module.id, str)
    with raises(AttributeError):
        api_module.charge  # pylint: disable=W0104
    # Set charge, for now it doesn't return any response, so doesn't need to test anything here
    api_module.change_mod(charge=eve_charge.id)
    # ID only
    api_module.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert isinstance(api_module.id, str)
    assert isinstance(api_module.charge_id, str)
    # Partial
    api_module.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert isinstance(api_module.id, str)
    assert isinstance(api_module.charge.id, str)
    # Full
    api_module.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert isinstance(api_module.id, str)
    assert isinstance(api_module.charge.id, str)
