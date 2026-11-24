import typing

if typing.TYPE_CHECKING:
    from fw.request import Request


def test_error_cmd_type_unknown(client):

    def hook_req(req: Request):
        data = req.get_json()
        data['type'] = 'random'
        req.set_json(data=data)

    api_src = client.create_source()
    # Action & verification
    api_src.get_items(
        type_ids=[],
        hook_req=hook_req,
        status_code=400,
        json_predicate={'code': 'JSN-001', 'message': 're:.+'})


def test_error_cmd_type_missing(client):

    def hook_req(req: Request):
        data = req.get_json()
        del data['type']
        req.set_json(data=data)

    api_src = client.create_source()
    # Action & verification
    api_src.get_items(
        type_ids=[],
        hook_req=hook_req,
        status_code=400,
        json_predicate={'code': 'JSN-001', 'message': 're:.+'})
