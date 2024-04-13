
def test_error_no_fleet_full(client, consts):
    # Send ID in correct format, but there is no fleet with such ID
    client.create_sources()
    api_ss = client.create_ss()
    resp = client.get_fleet_request(
        ss_id=api_ss.id, fleet_id='1', fleet_info_mode=consts.ApiFleetInfoMode.full).send()
    resp.check(status_code=404, json_predicate={'code': 'COR-017', 'message': 'core library error: fleet 1 not found'})


def test_error_no_fleet_id(client, consts):
    # Send ID in correct format, but there is no fleet with such ID
    client.create_sources()
    api_ss = client.create_ss()
    resp = client.get_fleet_request(ss_id=api_ss.id, fleet_id='1', fleet_info_mode=consts.ApiFleetInfoMode.id).send()
    resp.check(status_code=404, json_predicate={'code': 'COR-017', 'message': 'core library error: fleet 1 not found'})


def test_error_no_fleet_malformed(client):
    # Send ID in incorrect format
    client.create_sources()
    api_ss = client.create_ss()
    resp = client.get_fleet_request(ss_id=api_ss.id, fleet_id='abc').send()
    resp.check(status_code=404, json_predicate={'code': 'IDC-002', 'message': 'unable to cast string "abc" to id'})
