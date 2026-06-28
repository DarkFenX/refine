"""
This module has tests which checks what happens when EVE data handler receives malformed data. Data
errors can be recoverable (valid JSON, but unexpected data), and unrecoverable. In case of
recoverable errors, data handler should skip malformed entity and parse everything else.

Since some phobos files contain data for 2 separate EVE data entities (typedogma,
dynamicitemattributes), data handler implementations have different functions to read/parse them,
and they have to be tested separately.
"""

import typing

from fw import check_no_field
from fw.api import ValOptions

if typing.TYPE_CHECKING:
    from fw.eve.containers import EvePrimitives


def test_types_key(client, log):

    def hook_data_prim(prim_data: EvePrimitives):
        prim_data.types = {
            f'str{k}' if k == eve_item1_id else k: v
            for k, v in prim_data.types.items()}

    # Same group ID just to avoid cleanup
    eve_group_id = client.mk_eve_item_group()
    eve_item1_id = client.mk_eve_item(grp_id=eve_group_id)
    eve_item2_id = client.mk_eve_item(grp_id=eve_group_id)
    client.create_sources(hook_data_prim=hook_data_prim)
    log.wait_log_entry(
        msg='1 warnings encountered during fetching of EItem, showing up to 5:',
        level='WARN',
        span='src-new:adg')
    log.wait_log_entry(
        msg=f'failed to cast key "str{eve_item1_id}" to integer',
        level='WARN',
        span='src-new:adg')
    api_sol = client.create_sol()
    api_item1 = api_sol.add_sw_effect(type_id=eve_item1_id)
    api_sol.add_sw_effect(type_id=eve_item2_id)
    # Verification - item which was failed is not in EVE item database, so it cannot be loaded
    api_val = api_sol.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_item1.id]


def test_types_value(client, log):

    def hook_data_prim(prim_data: EvePrimitives):
        prim_data.types[eve_item1_id] = [1, 2, 3]

    # Same group ID just to avoid cleanup
    eve_group_id = client.mk_eve_item_group()
    eve_item1_id = client.mk_eve_item(grp_id=eve_group_id)
    eve_item2_id = client.mk_eve_item(grp_id=eve_group_id)
    client.create_sources(hook_data_prim=hook_data_prim)
    log.wait_log_entry(
        msg='1 warnings encountered during fetching of EItem, showing up to 5:',
        level='WARN',
        span='src-new:adg')
    log.wait_log_entry(
        msg=f're:failed to parse value with key "{eve_item1_id}":.+',
        level='WARN',
        span='src-new:adg')
    api_sol = client.create_sol()
    api_item1 = api_sol.add_sw_effect(type_id=eve_item1_id)
    api_sol.add_sw_effect(type_id=eve_item2_id)
    # Verification - item which was failed is not in EVE item database, so it cannot be loaded
    api_val = api_sol.validate(options=ValOptions(not_loaded_item=True))
    assert api_val.passed is False
    assert api_val.details.not_loaded_item == [api_item1.id]


def test_typedogma_key(client, log):

    def hook_data_prim(prim_data: EvePrimitives):
        prim_data.typedogma = {
            f'pre{k}' if k == eve_item1_id else k: v
            for k, v in prim_data.typedogma.items()}

    eve_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect()
    eve_item1_id = client.mk_eve_item(attrs={eve_attr_id: 5}, eff_ids=[eve_effect_id])
    eve_item2_id = client.mk_eve_item(attrs={eve_attr_id: 7}, eff_ids=[eve_effect_id])
    client.create_sources(hook_data_prim=hook_data_prim)
    log.wait_log_entry(
        msg='1 warnings encountered during fetching of EItemAttr, showing up to 5:',
        level='WARN',
        span='src-new:adg')
    log.wait_log_entry(
        msg=f'failed to cast key "pre{eve_item1_id}" to integer',
        level='WARN',
        span='src-new:adg')
    log.wait_log_entry(
        msg='1 warnings encountered during fetching of EItemEffect, showing up to 5:',
        level='WARN',
        span='src-new:adg')
    log.wait_log_entry(
        msg=f'failed to cast key "pre{eve_item1_id}" to integer',
        level='WARN',
        span='src-new:adg')
    api_sol = client.create_sol()
    api_item1 = api_sol.add_sw_effect(type_id=eve_item1_id)
    api_item2 = api_sol.add_sw_effect(type_id=eve_item2_id)
    # Verification
    api_item1.update()
    with check_no_field():
        api_item1.attrs  # noqa: B018
    with check_no_field():
        api_item1.effects  # noqa: B018
    api_item2.update()
    assert eve_attr_id in api_item2.attrs
    assert eve_effect_id in api_item2.effects


def test_typedogma_value(client, log):

    def hook_data_prim(prim_data: EvePrimitives):
        prim_data.typedogma[eve_item1_id] = 'random'

    eve_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect()
    eve_item1_id = client.mk_eve_item(attrs={eve_attr_id: 5}, eff_ids=[eve_effect_id])
    eve_item2_id = client.mk_eve_item(attrs={eve_attr_id: 7}, eff_ids=[eve_effect_id])
    client.create_sources(hook_data_prim=hook_data_prim)
    log.wait_log_entry(
        msg='1 warnings encountered during fetching of EItemAttr, showing up to 5:',
        level='WARN',
        span='src-new:adg')
    log.wait_log_entry(
        msg=f're:failed to parse value with key "{eve_item1_id}":.+',
        level='WARN',
        span='src-new:adg')
    log.wait_log_entry(
        msg='1 warnings encountered during fetching of EItemEffect, showing up to 5:',
        level='WARN',
        span='src-new:adg')
    log.wait_log_entry(
        msg=f're:failed to parse value with key "{eve_item1_id}":.+',
        level='WARN',
        span='src-new:adg')
    api_sol = client.create_sol()
    api_item1 = api_sol.add_sw_effect(type_id=eve_item1_id)
    api_item2 = api_sol.add_sw_effect(type_id=eve_item2_id)
    # Verification
    api_item1.update()
    with check_no_field():
        api_item1.attrs  # noqa: B018
    with check_no_field():
        api_item1.effects  # noqa: B018
    api_item2.update()
    assert eve_attr_id in api_item2.attrs
    assert eve_effect_id in api_item2.effects
