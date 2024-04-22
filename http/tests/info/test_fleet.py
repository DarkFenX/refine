
def test_fit(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet.update()
    assert len(api_fleet.fits) == 1
    assert api_fit.id in api_fleet.fits
    # Action
    api_fleet.change(remove_fits=[api_fit.id])
    # Verification
    api_fleet.update()
    assert len(api_fleet.fits) == 0


def test_error_no_fleet_full(client, consts):
    # Send ID in correct format, but there is no fleet with such ID
    client.create_sources()
    api_sol = client.create_sol()
    resp = api_sol.get_fleet_request(fleet_id='1', fleet_info_mode=consts.ApiFleetInfoMode.full).send()
    resp.check(status_code=404, json_predicate={'code': 'COR-017', 'message': 'core library error: fleet 1 not found'})
    api_sol.check()


def test_error_no_fleet_id(client, consts):
    # Send ID in correct format, but there is no fleet with such ID
    client.create_sources()
    api_sol = client.create_sol()
    resp = api_sol.get_fleet_request(fleet_id='1', fleet_info_mode=consts.ApiFleetInfoMode.id).send()
    resp.check(status_code=404, json_predicate={'code': 'COR-017', 'message': 'core library error: fleet 1 not found'})
    api_sol.check()


def test_error_no_fleet_malformed(client):
    # Send ID in incorrect format
    client.create_sources()
    api_sol = client.create_sol()
    resp = api_sol.get_fleet_request(fleet_id='abc').send()
    resp.check(status_code=404, json_predicate={'code': 'IDC-002', 'message': 'unable to cast string "abc" to id'})
    api_sol.check()
