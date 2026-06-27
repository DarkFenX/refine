

def test_fit_create(client):
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.commands() as api_sol_cmds:
        api_fleet = api_sol_cmds.create_fleet()
        api_fit = api_sol_cmds.create_fit(fleet_id=api_fleet.id)
    # Verification
    assert api_fit.update().fleet_id == api_fleet.id
