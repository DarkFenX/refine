from fw import check_no_field


def test_fit_fit(client, consts):
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


def test_fit_item_override(client, consts):
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


def test_fit_item_override_backref(client, consts):
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


def test_fit_item_override_backref_error(client, consts):
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
    # Verification
    assert api_fit_info.implants[api_item.id].type_id == eve_item2_id


def test_item_item(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_item_id)
    with api_fit.batch() as api_fit_batch:
        api_item_info1 = api_fit_batch.get_item_info(item_id=api_item.id, item_mode=consts.ApiItemInfoMode.partial)
        api_item_info2 = api_fit_batch.get_item_info(item_id=api_item.id, item_mode=consts.ApiItemInfoMode.id)
    # Verification
    assert api_item_info1.type_id == eve_item_id
    with check_no_field():
        api_item_info2.type_id  # ruff:ignore[useless-expression]


def test_item_item_backref(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_item = api_fit_batch.add_implant(type_id=eve_item_id)
        api_item_info1 = api_fit_batch.get_item_info(item_id=api_item.id, item_mode=consts.ApiItemInfoMode.partial)
        api_item_info2 = api_fit_batch.get_item_info(item_id=api_item.id, item_mode=consts.ApiItemInfoMode.id)
    # Verification
    assert api_item_info1.type_id == eve_item_id
    with check_no_field():
        api_item_info2.type_id  # ruff:ignore[useless-expression]


def test_item_item_backref_error_range(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    with api_fit.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #2 does not have results recorded',
            'cmd_index': 1,
    }) as api_fit_batch:
        api_fit_batch.add_implant(type_id=eve_item_id)
        api_fit_batch.get_item_info(item_id='#2', item_mode=consts.ApiItemInfoMode.partial)


def test_item_item_backref_error_kind(client, consts):
    eve_item_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    with api_fit.batch(status_code=400, json_predicate={
        'code': 'BRF-001',
        'message': 'referenced command #0 exists, but does not have item ID info',
        'cmd_index': 2,
    }) as api_fit_batch:
        api_fit_batch.change_fit(sec_status=3.5)
        api_fit_batch.add_implant(type_id=eve_item_id)
        api_fit_batch.get_item_info(item_id='#0', item_mode=consts.ApiItemInfoMode.partial)
