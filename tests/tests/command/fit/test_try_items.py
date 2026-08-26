from fw.api import ValOptions


def test_fit(client):
    eve_drone_id = client.mk_eve_drone()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_fit_batch.add_implant(type_id=eve_not_loaded_id)
        api_type_ids1 = api_fit_batch.try_fit_items(
            type_ids=[eve_drone_id],
            val_options=ValOptions(not_loaded_item=True))
        api_type_ids2 = api_fit_batch.try_fit_items(
            type_ids=[eve_drone_id],
            val_options=ValOptions())
    # Verification
    assert api_type_ids1 == []
    assert api_type_ids2 == [eve_drone_id]


def test_fit_kfs(client):
    eve_drone_id = client.mk_eve_drone()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_implant(type_id=eve_not_loaded_id)
    with api_fit.batch() as api_fit_batch:
        api_type_ids1 = api_fit_batch.try_fit_items(
            type_ids=[eve_drone_id],
            val_options=ValOptions(not_loaded_item=True))
        api_type_ids2 = api_fit_batch.try_fit_items(
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
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_fit_batch.change_fit(sec_status=2.5)
        api_item = api_fit_batch.add_implant(type_id=eve_not_loaded_id)
        api_type_ids1 = api_fit_batch.try_fit_items(
            type_ids=[eve_drone_id],
            val_options=ValOptions(not_loaded_item=['#0', '#9']))
        api_type_ids2 = api_fit_batch.try_fit_items(
            type_ids=[eve_drone_id],
            val_options=ValOptions(not_loaded_item=['#0', '#9', api_item.id]))
    # Verification
    assert api_type_ids1 == []
    assert api_type_ids2 == [eve_drone_id]
