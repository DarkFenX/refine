from fw import check_no_field


def test_fit(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item_id)
    with api_fit.batch() as api_fit_batch:
        api_fit_info1 = api_fit_batch.get_fit_info(fit_mode=consts.ApiFitInfoMode.full)
        api_fit_info2 = api_fit_batch.get_fit_info(fit_mode=consts.ApiFitInfoMode.id)
    # Verification
    assert api_item.id in api_fit_info1.implants
    with check_no_field():
        api_fit_info2.implants  # ruff:ignore[useless-expression]


def test_item_override(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item1 = api_fit.add_implant(type_id=eve_item_id)
    api_item2 = api_fit.add_implant(type_id=eve_item_id)
    with api_fit.batch() as api_fit_batch:
        api_fit_info = api_fit_batch.get_fit_info(
            fit_mode=consts.ApiFitInfoMode.full,
            item_mode=(consts.ApiItemInfoMode.id, [(consts.ApiItemInfoMode.partial, [api_item2.id])]))
    # Verification
    api_item1_info = api_fit_info.implants[api_item1.id]
    with check_no_field():
        api_item1_info.type_id  # ruff:ignore[useless-expression]
    api_item2_info = api_fit_info.implants[api_item2.id]
    assert api_item2_info.type_id == eve_item_id


def test_item_override_backref(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_item1 = api_fit_batch.add_implant(type_id=eve_item_id)
        api_item2 = api_fit_batch.add_implant(type_id=eve_item_id)
        api_fit_info = api_fit_batch.get_fit_info(
            fit_mode=consts.ApiFitInfoMode.full,
            item_mode=(consts.ApiItemInfoMode.id, [(consts.ApiItemInfoMode.partial, [api_item2.id])]))
    # Verification
    api_item1_info = api_fit_info.implants[api_item1.id]
    with check_no_field():
        api_item1_info.type_id  # ruff:ignore[useless-expression]
    api_item2_info = api_fit_info.implants[api_item2.id]
    assert api_item2_info.type_id == eve_item_id


def test_item_override_backref_error(client, consts):
    eve_item1_id = client.mk_eve_item()
    eve_item2_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_item = api_fit_batch.add_implant(type_id=eve_item1_id)
        api_fit_batch.change_implant(item_id=api_item.id, type_id=eve_item2_id)
        api_fit_info = api_fit_batch.get_fit_info(
            fit_mode=consts.ApiFitInfoMode.full,
            item_mode=(consts.ApiItemInfoMode.partial, [(consts.ApiItemInfoMode.id, ['#1', '#5'])]))
    # Verification - #1 references existing command which does not return an item ID, #5 references
    # command which doesn't exist, so default is used
    assert api_fit_info.implants[api_item.id].type_id == eve_item2_id
