import typing

from fw.api import ValOptions

if typing.TYPE_CHECKING:
    from fw.eve.containers import EvePrimitives


def test_types_value(client, log):
    # This test checks what happens when handler sees some valid JSON, but not an item object. The
    # malformed item should be skipped, and the rest of the file processed normally.

    def data_prim_hook(prim_data: EvePrimitives):
        prim_data.types[eve_item1_id] = [1, 2, 3]

    # Same group ID just to avoid cleanup
    eve_group_id = client.mk_eve_item_group()
    eve_item1_id = client.mk_eve_item(grp_id=eve_group_id)
    eve_item2_id = client.mk_eve_item(grp_id=eve_group_id)
    client.create_sources(data_prim_hook=data_prim_hook)
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
