"""
Here we check availability of info of various items via fleet info endpoint.
"""

from fw import check_no_field
from fw.util import Absent


def test_error_params_malformed(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fleet = api_sol.create_fleet()
    # Verification
    api_fleet.update(
        fleet_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})


def test_fit(client):
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet.update()
    assert len(api_fleet.fit_ids) == 1
    assert api_fit.id in api_fleet.fit_ids
    # Action
    api_fleet.change(rm_fit_ids=[api_fit.id])
    # Verification
    api_fleet.update()
    with check_no_field():
        api_fleet.fit_ids  # noqa: B018


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
        json_predicate={'code': 'IDC-002', 'message': 'unable to cast string "abc" to id'})
