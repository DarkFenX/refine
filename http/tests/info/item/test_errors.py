
def test_error_no_item_full(client, consts):
    # Send ID in correct format, but there is no fleet with such ID
    client.create_sources()
    api_sol = client.create_sol()
    resp = api_sol.get_item_request(item_id='1', item_info_mode=consts.ApiItemInfoMode.full).send()
    resp.check(status_code=404, json_predicate={'code': 'COR-004', 'message': 'core library error: item 1 not found'})
    api_sol.check()


def test_error_no_item_id(client, consts):
    # Send ID in correct format, but there is no fleet with such ID
    client.create_sources()
    api_sol = client.create_sol()
    resp = api_sol.get_item_request(item_id='1', item_info_mode=consts.ApiItemInfoMode.id).send()
    resp.check(status_code=404, json_predicate={'code': 'COR-004', 'message': 'core library error: item 1 not found'})
    api_sol.check()


def test_error_no_item_malformed(client):
    # Send ID in incorrect format
    client.create_sources()
    api_sol = client.create_sol()
    resp = api_sol.get_item_request(item_id='abc').send()
    resp.check(status_code=404, json_predicate={'code': 'IDC-003', 'message': 'unable to cast string "abc" to id'})
    api_sol.check()
