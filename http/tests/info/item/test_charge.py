from tests import approx, check_no_field


def test_bundled(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_module_id = client.mk_eve_item()
    eve_charge_id = client.mk_eve_item(attrs={eve_attr_id: 10})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_module = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge_id)
    api_charge = api_module.charge
    assert isinstance(api_charge.id, str)
    with check_no_field():
        api_charge.kind  # pylint: disable=W0104
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    api_charge_id = api_charge.id
    # ID only
    api_module.update(item_info_mode=consts.ApiItemInfoMode.id)
    api_charge = api_module.charge
    assert api_charge.id == api_charge_id
    with check_no_field():
        api_charge.kind  # pylint: disable=W0104
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    api_charge.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert api_charge.id == api_charge_id
    with check_no_field():
        api_charge.kind  # pylint: disable=W0104
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    # Partial
    api_module.update(item_info_mode=consts.ApiItemInfoMode.partial)
    api_charge = api_module.charge
    assert api_charge.id == api_charge_id
    assert api_charge.kind == consts.ApiItemKind.charge
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    api_charge.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert api_charge.id == api_charge_id
    assert api_charge.kind == consts.ApiItemKind.charge
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    # Full
    api_module.update(item_info_mode=consts.ApiItemInfoMode.full)
    api_charge = api_module.charge
    assert api_charge.id == api_charge_id
    assert api_charge.kind == consts.ApiItemKind.charge
    assert api_charge.attrs[eve_attr_id].dogma == approx(10)
    api_charge.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert api_charge.id == api_charge_id
    assert api_charge.kind == consts.ApiItemKind.charge
    assert api_charge.attrs[eve_attr_id].dogma == approx(10)


def test_separate(client, consts):
    eve_attr_id = client.mk_eve_attr()
    eve_module_id = client.mk_eve_item()
    eve_charge_id = client.mk_eve_item(attrs={eve_attr_id: 10})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_module = api_fit.add_mod(type_id=eve_module_id)
    with check_no_field():
        api_module.charge  # pylint: disable=W0104
    # ID only
    api_module.update(item_info_mode=consts.ApiItemInfoMode.id)
    with check_no_field():
        api_module.charge  # pylint: disable=W0104
    # Partial
    api_module.update(item_info_mode=consts.ApiItemInfoMode.partial)
    with check_no_field():
        api_module.charge  # pylint: disable=W0104
    # Full
    api_module.update(item_info_mode=consts.ApiItemInfoMode.full)
    with check_no_field():
        api_module.charge  # pylint: disable=W0104
    # Set charge and test response
    api_module.change_mod(charge=eve_charge_id)
    api_charge = api_module.charge
    assert isinstance(api_charge.id, str)
    with check_no_field():
        api_charge.kind  # pylint: disable=W0104
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    api_charge_id = api_charge.id
    # ID only
    api_module.update(item_info_mode=consts.ApiItemInfoMode.id)
    api_charge = api_module.charge
    assert api_charge.id == api_charge_id
    with check_no_field():
        api_charge.kind  # pylint: disable=W0104
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    api_charge.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert api_charge.id == api_charge_id
    with check_no_field():
        api_charge.kind  # pylint: disable=W0104
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    # Partial
    api_module.update(item_info_mode=consts.ApiItemInfoMode.partial)
    api_charge = api_module.charge
    assert api_charge.id == api_charge_id
    assert api_charge.kind == consts.ApiItemKind.charge
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    api_charge.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert api_charge.id == api_charge_id
    assert api_charge.kind == consts.ApiItemKind.charge
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    # Full
    api_module.update(item_info_mode=consts.ApiItemInfoMode.full)
    api_charge = api_module.charge
    assert api_charge.id == api_charge_id
    assert api_charge.kind == consts.ApiItemKind.charge
    assert api_charge.attrs[eve_attr_id].dogma == approx(10)
    api_charge.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert api_charge.id == api_charge_id
    assert api_charge.kind == consts.ApiItemKind.charge
    assert api_charge.attrs[eve_attr_id].dogma == approx(10)


def test_unloaded(client, consts):
    eve_module_id = client.mk_eve_item()
    eve_charge_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Check default upon addition
    api_module = api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge_id)
    api_charge = api_module.charge
    assert isinstance(api_charge.id, str)
    with check_no_field():
        api_charge.kind  # pylint: disable=W0104
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    api_charge_id = api_charge.id
    # ID only
    api_module.update(item_info_mode=consts.ApiItemInfoMode.id)
    api_charge = api_module.charge
    assert api_charge.id == api_charge_id
    with check_no_field():
        api_charge.kind  # pylint: disable=W0104
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    api_charge.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert api_charge.id == api_charge_id
    with check_no_field():
        api_charge.kind  # pylint: disable=W0104
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    # Partial
    api_module.update(item_info_mode=consts.ApiItemInfoMode.partial)
    api_charge = api_module.charge
    assert api_charge.id == api_charge_id
    assert api_charge.kind == consts.ApiItemKind.charge
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    api_charge.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert api_charge.id == api_charge_id
    assert api_charge.kind == consts.ApiItemKind.charge
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    # Full
    api_module.update(item_info_mode=consts.ApiItemInfoMode.full)
    api_charge = api_module.charge
    assert api_charge.id == api_charge_id
    assert api_charge.kind == consts.ApiItemKind.charge
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
    api_charge.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert api_charge.id == api_charge_id
    assert api_charge.kind == consts.ApiItemKind.charge
    with check_no_field():
        api_charge.attrs  # pylint: disable=W0104
