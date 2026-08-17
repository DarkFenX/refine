"""
Body limits apply to all requests equally. The reason it's tested under sol change command is
because it's the one which most likely breaks through the limit in case of huge command batches.
"""

import typing

from fw import check_no_field

if typing.TYPE_CHECKING:
    from fw.api.aliases import ReqHook
    from fw.request import Request


def make_req_hook(*, body_size: int, chunk: bool = False) -> ReqHook:

    def hook_req(req: Request):
        data = req.get_json()
        data[0]['padding'] = ''
        req.set_json(data=data)
        assert body_size >= req.get_body_size()
        data = req.get_json()
        data[0]['padding'] = 'a' * (body_size - req.get_body_size())
        req.set_json(data=data, chunk=128 * 1024 if chunk else None)

    return hook_req


def test_max_normal(client, consts, run_config):
    req_body_size = run_config.max_request_body_size
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.batch(hook_req=make_req_hook(body_size=req_body_size)) as api_sol_batch:
        api_sol_batch.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
    # Verification
    assert api_fit.update(item_info_mode=consts.ApiItemInfoMode.full).ship.type_id == eve_ship_id


def test_max_chunked(client, consts, run_config):
    req_body_size = run_config.max_request_body_size
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.batch(hook_req=make_req_hook(body_size=req_body_size, chunk=True)) as api_sol_batch:
        api_sol_batch.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
    # Verification
    assert api_fit.update(item_info_mode=consts.ApiItemInfoMode.full).ship.type_id == eve_ship_id


def test_error_normal(client, log, run_config):
    req_body_size = run_config.max_request_body_size + 1
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.batch(
            hook_req=make_req_hook(body_size=req_body_size),
            status_code=413,
            json_predicate={
                'code': 'REQ-002',
                'message': f'failed to process request body: received length {req_body_size} '
                           f'is bigger than limit {run_config.max_request_body_size}'},
    ) as api_sol_batch:
        api_sol_batch.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.ship  # ruff:ignore[useless-expression]
    log.wait_log_entry(
        msg=f'>>> rx body: <received length {req_body_size} is bigger than limit {run_config.max_request_body_size}>')


def test_error_chunked(client, log, run_config):
    req_body_size = run_config.max_request_body_size + 1
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_sol.batch(
            hook_req=make_req_hook(body_size=req_body_size, chunk=True),
            status_code=413,
            json_predicate={
                'code': 'REQ-002',
                'message': 'failed to process request body: received length <unknown> '
                           f'is bigger than limit {run_config.max_request_body_size}'},
    ) as api_sol_batch:
        api_sol_batch.set_ship(fit_id=api_fit.id, type_id=eve_ship_id)
    # Verification
    api_fit.update()
    with check_no_field():
        api_fit.ship  # ruff:ignore[useless-expression]
    log.wait_log_entry(
        msg=f'>>> rx body: <received length <unknown> is bigger than limit {run_config.max_request_body_size}>')
