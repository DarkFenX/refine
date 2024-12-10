

def test_charge(client):
    eve_module_id = client.mk_eve_item()
    eve_charge_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_module_id, charge_type_id=eve_charge_id)
    # Check via consistency check if item with charge is properly removed when fit is removed
    api_fit.remove()
