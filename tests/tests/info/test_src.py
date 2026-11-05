"""
Here we check info of data sources, which is returned on source creation and via source endpoint.
"""
import typing

from fw import check_no_field

if typing.TYPE_CHECKING:
    from fw.eve.containers import EvePrimitives


def test_mode_alias(client, consts):
    eve_data = client.mk_eve_data()
    # Action
    api_src = client.create_source(data=eve_data, src_info_mode=consts.ApiSrcInfoMode.alias)
    # Verification
    assert api_src.alias == eve_data.alias
    with check_no_field():
        api_src.time_created  # ruff:ignore[useless-expression]
    with check_no_field():
        api_src.origin  # ruff:ignore[useless-expression]
    with check_no_field():
        api_src.default  # ruff:ignore[useless-expression]
    # Action
    api_src.update(src_info_mode=consts.ApiSrcInfoMode.alias)
    # Verification
    assert api_src.alias == eve_data.alias
    with check_no_field():
        api_src.time_created  # ruff:ignore[useless-expression]
    with check_no_field():
        api_src.origin  # ruff:ignore[useless-expression]
    with check_no_field():
        api_src.default  # ruff:ignore[useless-expression]


def test_mode_partial(client, consts):

    def hook_data_prim(prim_data: EvePrimitives):
        prim_data.types = {f'str{k}' if k == eve_item1_id else k: v for k, v in prim_data.types.items()}

    eve_data = client.mk_eve_data()
    # Same group ID just to avoid cleanup
    eve_group_id = client.mk_eve_item_group(datas=[eve_data])
    eve_item1_id = client.mk_eve_item(datas=[eve_data], grp_id=eve_group_id)
    client.mk_eve_item(datas=[eve_data], grp_id=eve_group_id)
    # Action
    api_src = client.create_source(
        data=eve_data,
        src_info_mode=consts.ApiSrcInfoMode.partial,
        hook_data_prim=hook_data_prim)
    # Verification - warnings are not exposed, even if there are some
    assert api_src.alias == eve_data.alias
    time_created = api_src.time_created
    assert api_src.origin.type == 'generated'
    assert api_src.origin.reason.type == 'no_cacher'
    assert api_src.default is False
    with check_no_field():
        api_src.warnings  # ruff:ignore[useless-expression]
    # Action
    api_src.update(src_info_mode=consts.ApiSrcInfoMode.partial)
    # Verification
    assert api_src.alias == eve_data.alias
    assert api_src.time_created == time_created
    assert api_src.origin.type == 'generated'
    assert api_src.origin.reason.type == 'no_cacher'
    assert api_src.default is False
    with check_no_field():
        api_src.warnings  # ruff:ignore[useless-expression]


def test_mode_full(client, consts):

    def hook_data_prim(prim_data: EvePrimitives):
        prim_data.types = {f'str{k}' if k == eve_item1_id else k: v for k, v in prim_data.types.items()}

    eve_data = client.mk_eve_data()
    # Same group ID just to avoid cleanup
    eve_group_id = client.mk_eve_item_group(datas=[eve_data])
    eve_item1_id = client.mk_eve_item(datas=[eve_data], grp_id=eve_group_id)
    client.mk_eve_item(datas=[eve_data], grp_id=eve_group_id)
    warning = f'failed to fetch EItem: failed to cast key "str{eve_item1_id}" to integer'
    # Action
    api_src = client.create_source(
        data=eve_data,
        src_info_mode=consts.ApiSrcInfoMode.full,
        hook_data_prim=hook_data_prim)
    # Verification
    assert api_src.alias == eve_data.alias
    time_created = api_src.time_created
    assert api_src.origin.type == 'generated'
    assert api_src.origin.reason.type == 'no_cacher'
    assert api_src.default is False
    assert api_src.warnings.eve_data_fetch == [warning]
    # Action
    api_src.update(src_info_mode=consts.ApiSrcInfoMode.full)
    # Verification
    assert api_src.alias == eve_data.alias
    assert api_src.time_created == time_created
    assert api_src.origin.type == 'generated'
    assert api_src.origin.reason.type == 'no_cacher'
    assert api_src.default is False
    assert api_src.warnings.eve_data_fetch == [warning]


def test_default(client, consts):
    eve_data1 = client.mk_eve_data()
    eve_data2 = client.mk_eve_data()
    eve_data3 = client.mk_eve_data()
    # Action
    api_src1 = client.create_source(data=eve_data1, make_default=True)
    api_src2 = client.create_source(data=eve_data2)
    # Verification
    assert api_src1.default is True
    assert api_src2.default is False
    # Action
    api_src3 = client.create_source(data=eve_data3, make_default=True)
    # Verification - previous default source is not default anymore
    assert api_src3.default is True
    assert api_src1.update().default is False
    assert api_src2.update().default is False
    assert api_src3.update().default is True
    assert api_src3.update(src_info_mode=consts.ApiSrcInfoMode.partial).default is True


def test_error_params_malformed(client):
    api_src = client.create_source()
    # Action & verification
    api_src.update(src_info_mode='random', status_code=400, json_predicate={'code': 'PRM-001', 'message': 're:.+'})
