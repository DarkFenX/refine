def test_add_fleet_not_found(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other_fleet = api_sol.create_fleet()
    api_other_fleet.remove()
    # Action & verification - fit is created before setting its fleet fails
    api_sol.create_fit(
        fleet_id=api_other_fleet.id,
        sec_status=-5,
        rah_incoming_dps=(1, 1, 1, 1),
        status_code=400,
        json_predicate={'code': 'FIT-003', 'message': f'failed to set fleet: fleet {api_other_fleet.id} not found'})
    assert api_sol.update().fits.keys() == {api_fit.id}
