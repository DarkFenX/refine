from fw.api import ValOptions


def test_known_failures(client):
    eve_drone_id = client.mk_eve_drone()
    eve_module_id = client.mk_eve_module()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item1 = api_fit.add_implant(type_id=eve_drone_id)
    api_item2 = api_fit.add_drone(type_id=eve_module_id)
    # Verification - items are tried in containers matching their kinds
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_kind=[api_item1.id]),
        passing=[],
        failing=[eve_drone_id, eve_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_kind=[api_item2.id]),
        passing=[],
        failing=[eve_drone_id, eve_module_id])
    api_fit.check_try_fit_items(
        val_options=ValOptions(item_kind=[api_item1.id, api_item2.id]),
        passing=[eve_drone_id, eve_module_id],
        failing=[])
