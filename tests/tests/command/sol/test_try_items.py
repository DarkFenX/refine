from fw.api import ValOptions


def test_fit(client):
    eve_drone_id = client.mk_eve_drone()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.batch() as api_sol_batch:
        api_sol_batch.add_implant(fit_id=api_fit.id, type_id=eve_not_loaded_id)
        api_type_ids1 = api_sol_batch.try_fit_items(
            fit_id=api_fit.id,
            type_ids=[eve_drone_id],
            val_options=ValOptions(not_loaded_item=True))
        api_type_ids2 = api_sol_batch.try_fit_items(
            fit_id=api_fit.id,
            type_ids=[eve_drone_id],
            val_options=ValOptions())
    # Verification
    assert api_type_ids1 == []
    assert api_type_ids2 == [eve_drone_id]


def test_fit_backref(client):
    eve_drone_id = client.mk_eve_drone()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit()
        api_sol_batch.add_implant(fit_id=api_fit.id, type_id=eve_not_loaded_id)
        api_type_ids1 = api_sol_batch.try_fit_items(
            fit_id=api_fit.id,
            type_ids=[eve_drone_id],
            val_options=ValOptions(not_loaded_item=True))
        api_type_ids2 = api_sol_batch.try_fit_items(
            fit_id=api_fit.id,
            type_ids=[eve_drone_id],
            val_options=ValOptions())
    # Verification
    assert api_type_ids1 == []
    assert api_type_ids2 == [eve_drone_id]


def test_fit_backref_error_range(client):
    eve_drone_id = client.mk_eve_drone()
    client.create_sources()
    api_sol = client.create_sol()
    # Verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #2 does not have results recorded',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fit()
        api_sol_batch.try_fit_items(fit_id='#2', type_ids=[eve_drone_id])


def test_fit_backref_error_kind(client):
    eve_drone_id = client.mk_eve_drone()
    client.create_sources()
    api_sol = client.create_sol()
    # Verification
    with api_sol.batch(status_code=400, json_predicate={
            'code': 'BRF-001',
            'message': 'referenced command #0 exists, but does not have fit ID info',
            'cmd_index': 1,
    }) as api_sol_batch:
        api_sol_batch.create_fleet()
        api_sol_batch.try_fit_items(fit_id='#0', type_ids=[eve_drone_id])


def test_fit_kfs(client):
    eve_drone_id = client.mk_eve_drone()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_not_loaded_id)
    with api_sol.batch() as api_sol_batch:
        api_type_ids1 = api_sol_batch.try_fit_items(
            fit_id=api_fit.id,
            type_ids=[eve_drone_id],
            val_options=ValOptions(not_loaded_item=True))
        api_type_ids2 = api_sol_batch.try_fit_items(
            fit_id=api_fit.id,
            type_ids=[eve_drone_id],
            val_options=ValOptions(not_loaded_item=[api_item.id]))
    # Verification
    assert api_type_ids1 == []
    assert api_type_ids2 == [eve_drone_id]


def test_fit_kfs_backref(client):
    eve_drone_id = client.mk_eve_drone()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_fit = api_sol_batch.create_fit()
        api_item = api_sol_batch.add_implant(fit_id=api_fit.id, type_id=eve_not_loaded_id)
        api_type_ids1 = api_sol_batch.try_fit_items(
            fit_id=api_fit.id,
            type_ids=[eve_drone_id],
            val_options=ValOptions(not_loaded_item=['#0', '#5']))
        api_type_ids2 = api_sol_batch.try_fit_items(
            fit_id=api_fit.id,
            type_ids=[eve_drone_id],
            val_options=ValOptions(not_loaded_item=['#0', '#5', api_item.id]))
    # Verification
    assert api_type_ids1 == []
    assert api_type_ids2 == [eve_drone_id]
