from tests.support.util import Absent


def test_error_no_item_full(client, consts):
    # Send ID in correct format, but there is no fleet with such ID
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.get_item(
        item_id='1',
        item_info_mode=consts.ApiItemInfoMode.full,
        status_code=404,
        json_predicate={'code': 'EXC-013', 'message': 'item 1 not found'})


def test_error_no_item_id(client, consts):
    # Send ID in correct format, but there is no fleet with such ID
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.get_item(
        item_id='1',
        item_info_mode=consts.ApiItemInfoMode.id,
        status_code=404,
        json_predicate={'code': 'EXC-013', 'message': 'item 1 not found'})


def test_error_no_item_malformed(client):
    # Send ID in incorrect format
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.get_item(
        item_id='abc',
        item_info_mode=Absent,
        status_code=404,
        json_predicate={'code': 'IDC-003', 'message': 'unable to cast string "abc" to id'})
