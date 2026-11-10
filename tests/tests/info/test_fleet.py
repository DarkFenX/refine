"""
Here we check availability of info of various items via fleet info endpoint.
"""

from fw import Absent, check_no_field


def test_error_params_malformed(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    # Action & verification
    api_fleet.update(
        fleet_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})


def test_mode_id(client, consts):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    # Action
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit1.id], fleet_info_mode=consts.ApiFleetInfoMode.id)
    # Verification
    with check_no_field():
        api_fleet.fits  # ruff:ignore[useless-expression]
    # Action
    api_fleet.change(add_fit_ids=[api_fit2.id])
    api_fleet.update(fleet_info_mode=consts.ApiFleetInfoMode.id)
    # Verification
    with check_no_field():
        api_fleet.fits  # ruff:ignore[useless-expression]


def test_mode_full(client, consts):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit2 = api_sol.create_fit()
    # Action
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit1.id], fleet_info_mode=consts.ApiFleetInfoMode.full)
    # Verification
    assert api_fleet.fits.keys() == {api_fit1.id}
    # Action
    api_fleet.change(add_fit_ids=[api_fit2.id])
    api_fleet.update(fleet_info_mode=consts.ApiFleetInfoMode.full)
    # Verification
    assert api_fleet.fits.keys() == {api_fit1.id, api_fit2.id}


def test_fit(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet.update()
    assert len(api_fleet.fits) == 1
    assert api_fit.id in api_fleet.fits
    # Action
    api_fleet.change(rm_fit_ids=[api_fit.id])
    # Verification
    api_fleet.update()
    with check_no_field():
        api_fleet.fits  # ruff:ignore[useless-expression]


def test_error_no_fleet_full(client, consts):
    # Send ID in correct format, but there is no fleet with such ID
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.get_fleet(
        fleet_id='1',
        fleet_info_mode=consts.ApiFleetInfoMode.full,
        status_code=404,
        json_predicate={'code': 'FLT-001', 'message': 'fleet 1 not found'})


def test_error_no_fleet_id(client, consts):
    # Send ID in correct format, but there is no fleet with such ID
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.get_fleet(
        fleet_id='1',
        fleet_info_mode=consts.ApiFleetInfoMode.id,
        status_code=404,
        json_predicate={'code': 'FLT-001', 'message': 'fleet 1 not found'})


def test_error_no_fleet_malformed(client):
    # Send ID in incorrect format
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.get_fleet(
        fleet_id='abc',
        fleet_info_mode=Absent,
        status_code=404,
        json_predicate={'code': 'FLT-002', 'message': 'invalid digit found in string'})
