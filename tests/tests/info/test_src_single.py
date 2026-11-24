def test_error_cmd_type_unknown(client):
    api_src = client.create_source()
    req = client.get_source_items_request(src_alias=api_src.alias, type_ids=[])
    data = req.get_json()
    data['type'] = 'random'
    req.set_json(data=data)
    # Action & verification
    req.send().check(status_code=400, json_predicate={'code': 'JSN-001', 'message': 're:.+'})


def test_error_cmd_type_missing(client):
    api_src = client.create_source()
    req = client.get_source_items_request(src_alias=api_src.alias, type_ids=[])
    data = req.get_json()
    del data['type']
    req.set_json(data=data)
    # Action & verification
    req.send().check(status_code=400, json_predicate={'code': 'JSN-001', 'message': 're:.+'})
