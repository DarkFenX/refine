from fw import check_no_field
from fw.api import ValOptions


def test_sol(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_item = api_sol_batch.add_sw_effect(type_id=eve_not_loaded_id)
        api_val1 = api_sol_batch.validate_sol(
            options=ValOptions(not_loaded_item=True),
            info_mode=consts.ApiValInfoMode.detailed)
        api_val2 = api_sol_batch.validate_sol(
            options=ValOptions(not_loaded_item=True),
            info_mode=consts.ApiValInfoMode.simple)
    # Verification
    assert api_val1.passed is False
    assert api_val1.details.not_loaded_item == [api_item.id]
    assert api_val2.passed is False
    with check_no_field():
        api_val2.details  # ruff:ignore[useless-expression]


def test_sol_fit_ids(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_not_loaded_id)
    with api_sol.batch() as api_sol_batch:
        api_val1 = api_sol_batch.validate_sol(
            options=ValOptions(not_loaded_item=True),
            fit_ids=[api_fit.id],
            info_mode=consts.ApiValInfoMode.detailed)
        api_val2 = api_sol_batch.validate_sol(
            options=ValOptions(not_loaded_item=True),
            info_mode=consts.ApiValInfoMode.detailed)
    # Verification
    assert api_val1.passed is False
    assert api_val1.details.fits[api_fit.id].not_loaded_item == [api_item.id]
    assert api_val2.passed is True
    with check_no_field():
        api_val2.details  # ruff:ignore[useless-expression]


def test_sol_fit_ids_backref(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit()
        api_item = api_sol_batch.add_implant(fit_id=api_fit.id, type_id=eve_not_loaded_id)
        api_val1 = api_sol_batch.validate_sol(
            options=ValOptions(not_loaded_item=True),
            fit_ids=['#1', '#5'],
            info_mode=consts.ApiValInfoMode.detailed)
        api_val2 = api_sol_batch.validate_sol(
            options=ValOptions(not_loaded_item=True),
            fit_ids=['#1', '#5', api_fit.id],
            info_mode=consts.ApiValInfoMode.detailed)
    # Verification
    assert api_val1.passed is True
    with check_no_field():
        api_val1.details  # ruff:ignore[useless-expression]
    assert api_val2.passed is False
    assert api_val2.details.fits[api_fit.id].not_loaded_item == [api_item.id]


def test_sol_kfs(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_item = api_sol.add_sw_effect(type_id=eve_not_loaded_id)
    with api_sol.batch() as api_sol_batch:
        api_val1 = api_sol_batch.validate_sol(
            options=ValOptions(not_loaded_item=True),
            info_mode=consts.ApiValInfoMode.detailed)
        api_val2 = api_sol_batch.validate_sol(
            options=ValOptions(not_loaded_item=[api_item.id]),
            info_mode=consts.ApiValInfoMode.detailed)
    # Verification
    assert api_val1.passed is False
    assert api_val1.details.not_loaded_item == [api_item.id]
    assert api_val2.passed is True
    with check_no_field():
        api_val2.details  # ruff:ignore[useless-expression]


def test_sol_kfs_backref(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_sol_batch.create_fit()
        api_item = api_sol_batch.add_sw_effect(type_id=eve_not_loaded_id)
        api_val1 = api_sol_batch.validate_sol(
            options=ValOptions(not_loaded_item=['#0', '#5']),
            info_mode=consts.ApiValInfoMode.detailed)
        api_val2 = api_sol_batch.validate_sol(
            options=ValOptions(not_loaded_item=['#0', '#5', api_item.id]),
            info_mode=consts.ApiValInfoMode.detailed)
    # Verification
    assert api_val1.passed is False
    assert api_val1.details.not_loaded_item == [api_item.id]
    assert api_val2.passed is True
    with check_no_field():
        api_val2.details  # ruff:ignore[useless-expression]


def test_fit(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.batch() as api_sol_batch:
        api_item = api_sol_batch.add_implant(fit_id=api_fit.id, type_id=eve_not_loaded_id)
        api_val1 = api_sol_batch.validate_fit(
            fit_id=api_fit.id,
            options=ValOptions(not_loaded_item=True),
            info_mode=consts.ApiValInfoMode.detailed)
        api_val2 = api_sol_batch.validate_fit(
            fit_id=api_fit.id,
            options=ValOptions(not_loaded_item=True),
            info_mode=consts.ApiValInfoMode.simple)
    # Verification
    assert api_val1.passed is False
    assert api_val1.details.not_loaded_item == [api_item.id]
    assert api_val2.passed is False
    with check_no_field():
        api_val2.details  # ruff:ignore[useless-expression]


def test_fit_backref(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit()
        api_item = api_sol_batch.add_implant(fit_id=api_fit.id, type_id=eve_not_loaded_id)
        api_val1 = api_sol_batch.validate_fit(
            fit_id=api_fit.id,
            options=ValOptions(not_loaded_item=True),
            info_mode=consts.ApiValInfoMode.detailed)
        api_val2 = api_sol_batch.validate_fit(
            fit_id=api_fit.id,
            options=ValOptions(not_loaded_item=True),
            info_mode=consts.ApiValInfoMode.simple)
    # Verification
    assert api_val1.passed is False
    assert api_val1.details.not_loaded_item == [api_item.id]
    assert api_val2.passed is False
    with check_no_field():
        api_val2.details  # ruff:ignore[useless-expression]


def test_fit_backref_error_range(client):
    client.create_sources()
    api_sol = client.create_sol()
    # Verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #2 does not have results recorded',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fit()
        api_sol_batch.validate_fit(fit_id='#2', options=ValOptions(not_loaded_item=True))


def test_fit_backref_error_kind(client):
    client.create_sources()
    api_sol = client.create_sol()
    # Verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #0 exists, but does not have fit ID info',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fleet()
        api_sol_batch.validate_fit(fit_id='#0', options=ValOptions(not_loaded_item=True))


def test_fit_kfs(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_not_loaded_id)
    with api_sol.batch() as api_sol_batch:
        api_val1 = api_sol_batch.validate_fit(
            fit_id=api_fit.id,
            options=ValOptions(not_loaded_item=True),
            info_mode=consts.ApiValInfoMode.detailed)
        api_val2 = api_sol_batch.validate_fit(
            fit_id=api_fit.id,
            options=ValOptions(not_loaded_item=[api_item.id]),
            info_mode=consts.ApiValInfoMode.detailed)
    # Verification
    assert api_val1.passed is False
    assert api_val1.details.not_loaded_item == [api_item.id]
    assert api_val2.passed is True
    with check_no_field():
        api_val2.details  # ruff:ignore[useless-expression]


def test_fit_kfs_backref(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit()
        api_item = api_sol_batch.add_implant(fit_id=api_fit.id, type_id=eve_not_loaded_id)
        api_val1 = api_sol_batch.validate_fit(
            fit_id=api_fit.id,
            options=ValOptions(not_loaded_item=['#0', '#5']),
            info_mode=consts.ApiValInfoMode.detailed)
        api_val2 = api_sol_batch.validate_fit(
            fit_id=api_fit.id,
            options=ValOptions(not_loaded_item=['#0', '#5', api_item.id]),
            info_mode=consts.ApiValInfoMode.detailed)
    # Verification
    assert api_val1.passed is False
    assert api_val1.details.not_loaded_item == [api_item.id]
    assert api_val2.passed is True
    with check_no_field():
        api_val2.details  # ruff:ignore[useless-expression]
