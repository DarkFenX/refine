from fw.api import ValOptions


def test_known_failures(client):
    eve_drone_id = client.mk_eve_drone()
    eve_not_loaded_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone1 = api_fit.add_drone(type_id=eve_not_loaded_id)
    api_drone2 = api_fit.add_drone(type_id=eve_not_loaded_id)
    # Verification - not loaded items are skipped instead of being tried
    api_fit.check_try_fit_items(
        val_options=ValOptions(not_loaded_item=[api_drone1.id]),
        passing=[],
        failing=[eve_drone_id, eve_not_loaded_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(not_loaded_item=[api_drone2.id]),
        passing=[],
        failing=[eve_drone_id, eve_not_loaded_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(not_loaded_item=[api_drone1.id, api_drone2.id]),
        passing=[eve_drone_id],
        failing=[eve_not_loaded_id])


def test_item_dynamic_bare(client):
    # Dynamic types are loaded only as mutated items, so on their own they are skipped instead of
    # being tried
    eve_base_drone_id = client.mk_eve_drone()
    eve_mutated_drone_id = client.mk_eve_drone(is_dynamic=True)
    client.mk_eve_mutator(items=[([eve_base_drone_id], eve_mutated_drone_id)])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit.check_try_fit_items(
        val_options=ValOptions(not_loaded_item=True),
        passing=[eve_base_drone_id],
        failing=[eve_mutated_drone_id])
