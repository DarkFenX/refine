from fw import check_no_field
from fw.api import ValOptions


def test_fit_fit(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_item = api_fit_batch.add_implant(type_id=eve_not_loaded_id)
        api_val1 = api_fit_batch.validate_fit(
            options=ValOptions(not_loaded_item=True),
            info_mode=consts.ApiValInfoMode.detailed)
        api_val2 = api_fit_batch.validate_fit(
            options=ValOptions(not_loaded_item=True),
            info_mode=consts.ApiValInfoMode.simple)
    # Verification
    assert api_val1.passed is False
    assert api_val1.details.not_loaded_item == [api_item.id]
    assert api_val2.passed is False
    with check_no_field():
        api_val2.details  # ruff:ignore[useless-expression]


def test_fit_item_kfs(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_not_loaded_id)
    with api_fit.batch() as api_fit_batch:
        api_val1 = api_fit_batch.validate_fit(
            options=ValOptions(not_loaded_item=True),
            info_mode=consts.ApiValInfoMode.detailed)
        api_val2 = api_fit_batch.validate_fit(
            options=ValOptions(not_loaded_item=[api_item.id]),
            info_mode=consts.ApiValInfoMode.detailed)
    # Verification
    assert api_val1.passed is False
    assert api_val1.details.not_loaded_item == [api_item.id]
    assert api_val2.passed is True
    with check_no_field():
        api_val2.details  # ruff:ignore[useless-expression]


def test_fit_item_kfs_backref(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_item = api_fit_batch.add_implant(type_id=eve_not_loaded_id)
        api_val1 = api_fit_batch.validate_fit(
            options=ValOptions(not_loaded_item=True),
            info_mode=consts.ApiValInfoMode.detailed)
        api_val2 = api_fit_batch.validate_fit(
            options=ValOptions(not_loaded_item=[api_item.id]),
            info_mode=consts.ApiValInfoMode.detailed)
    # Verification
    assert api_val1.passed is False
    assert api_val1.details.not_loaded_item == [api_item.id]
    assert api_val2.passed is True
    with check_no_field():
        api_val2.details  # ruff:ignore[useless-expression]


def test_fit_item_kfs_backref_error(client, consts):
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_fit_batch.change_fit(sec_status=2.5)
        api_item = api_fit_batch.add_implant(type_id=eve_not_loaded_id)
        api_val1 = api_fit_batch.validate_fit(
            options=ValOptions(not_loaded_item=['#0', '#5']),
            info_mode=consts.ApiValInfoMode.detailed)
        api_val2 = api_fit_batch.validate_fit(
            options=ValOptions(not_loaded_item=['#0', '#5', api_item.id]),
            info_mode=consts.ApiValInfoMode.detailed)
    # Verification - #0 references existing command which does not return an item ID, #5 references
    # command which doesn't exist, so validation is run with no known failures
    assert api_val1.passed is False
    assert api_val1.details.not_loaded_item == [api_item.id]
    assert api_val2.passed is True
    with check_no_field():
        api_val2.details  # ruff:ignore[useless-expression]
