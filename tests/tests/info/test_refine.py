"""
Here we check info of refine itself, which is exposed via root endpoint.
"""
import typing

from fw import check_no_field

if typing.TYPE_CHECKING:
    from fw.eve.containers import EvePrimitives


def test_error_params_malformed(client):
    # Action & verification
    client.get_refine_info(
        src_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    client.get_refine_info(
        sol_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    client.get_refine_info(
        fleet_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    client.get_refine_info(
        fit_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})
    client.get_refine_info(
        item_info_mode='random',
        status_code=400,
        json_predicate={'code': 'PRM-001', 'message': 're:.+'})


def test_threads(client):
    # Action
    api_refine = client.get_refine_info()
    # Verification - async thread count is set in the server code, others in test server config
    assert api_refine.threads_async == 2
    assert api_refine.threads_sync_standard == 2
    assert api_refine.threads_sync_heavy == 4


def test_empty(client):
    # Action
    api_refine = client.get_refine_info()
    # Verification
    assert api_refine.srcs == {}
    assert api_refine.sols == {}


def test_srcs(client, consts):

    def hook_data_prim(prim_data: EvePrimitives):
        prim_data.types = {f'str{k}' if k == eve_item1_id else k: v for k, v in prim_data.types.items()}

    eve_data1 = client.mk_eve_data()
    eve_data2 = client.mk_eve_data()
    # Same group ID just to avoid cleanup
    eve_group_id = client.mk_eve_item_group(datas=[eve_data2])
    eve_item1_id = client.mk_eve_item(datas=[eve_data2], grp_id=eve_group_id)
    client.mk_eve_item(datas=[eve_data2], grp_id=eve_group_id)
    warning = f'failed to fetch EItem: failed to cast key "str{eve_item1_id}" to integer'
    client.create_source(data=eve_data1, make_default=True)
    api_src2 = client.create_source(data=eve_data2, hook_data_prim=hook_data_prim)
    # Action
    api_refine = client.get_refine_info()
    # Verification - alias mode is used by default
    assert api_refine.srcs.keys() == {eve_data1.alias, eve_data2.alias}
    with check_no_field():
        api_refine.srcs[eve_data1.alias].default  # ruff:ignore[useless-expression]
    with check_no_field():
        api_refine.srcs[eve_data2.alias].default  # ruff:ignore[useless-expression]
    # Action
    api_refine = client.get_refine_info(src_info_mode=consts.ApiSrcInfoMode.partial)
    # Verification
    assert api_refine.srcs.keys() == {eve_data1.alias, eve_data2.alias}
    assert api_refine.srcs[eve_data1.alias].default is True
    assert api_refine.srcs[eve_data2.alias].default is False
    assert api_refine.srcs[eve_data2.alias].time_created == api_src2.time_created
    with check_no_field():
        api_refine.srcs[eve_data2.alias].warnings  # ruff:ignore[useless-expression]
    # Action
    api_refine = client.get_refine_info(src_info_mode=consts.ApiSrcInfoMode.full)
    # Verification
    assert api_refine.srcs.keys() == {eve_data1.alias, eve_data2.alias}
    assert api_refine.srcs[eve_data1.alias].default is True
    assert api_refine.srcs[eve_data2.alias].default is False
    assert api_refine.srcs[eve_data2.alias].time_created == api_src2.time_created
    assert api_refine.srcs[eve_data2.alias].warnings.eve_data_fetch == [warning]


def test_sols(client, consts):
    eve_data1 = client.mk_eve_data()
    eve_data2 = client.mk_eve_data()
    eve_ship_id = client.mk_eve_ship(datas=[eve_data2])
    client.create_sources()
    api_sol1 = client.create_sol(data=eve_data1)
    api_sol2 = client.create_sol(data=eve_data2, sec_zone=consts.ApiSecZone.nullsec)
    api_fit = api_sol2.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Action
    api_refine = client.get_refine_info()
    # Verification - ID mode is used by default
    assert api_refine.sols.keys() == {api_sol1.id, api_sol2.id}
    with check_no_field():
        api_refine.sols[api_sol1.id].src_alias  # ruff:ignore[useless-expression]
    with check_no_field():
        api_refine.sols[api_sol2.id].src_alias  # ruff:ignore[useless-expression]
    # Action
    api_refine = client.get_refine_info(sol_info_mode=consts.ApiSolInfoMode.partial)
    # Verification
    assert api_refine.sols.keys() == {api_sol1.id, api_sol2.id}
    assert api_refine.sols[api_sol1.id].src_alias == eve_data1.alias
    assert api_refine.sols[api_sol2.id].src_alias == eve_data2.alias
    with check_no_field():
        api_refine.sols[api_sol2.id].sec_zone  # ruff:ignore[useless-expression]
    with check_no_field():
        api_refine.sols[api_sol2.id].fits  # ruff:ignore[useless-expression]
    # Action
    api_refine = client.get_refine_info(
        sol_info_mode=consts.ApiSolInfoMode.full,
        fit_info_mode=consts.ApiFitInfoMode.full)
    # Verification
    assert api_refine.sols.keys() == {api_sol1.id, api_sol2.id}
    assert api_refine.sols[api_sol1.id].src_alias == eve_data1.alias
    assert api_refine.sols[api_sol2.id].src_alias == eve_data2.alias
    assert api_refine.sols[api_sol2.id].sec_zone == consts.ApiSecZone.nullsec
    assert api_refine.sols[api_sol2.id].fits.keys() == {api_fit.id}
    assert api_refine.sols[api_sol2.id].fits[api_fit.id].ship.id == api_ship.id


def test_sol_src_switch(client, consts):
    eve_data1 = client.mk_eve_data()
    eve_data2 = client.mk_eve_data()
    client.create_sources()
    api_sol = client.create_sol(data=eve_data1)
    # Action
    api_sol.change_src(data=eve_data2)
    # Verification
    api_refine = client.get_refine_info(sol_info_mode=consts.ApiSolInfoMode.partial)
    assert api_refine.sols[api_sol.id].src_alias == eve_data2.alias
