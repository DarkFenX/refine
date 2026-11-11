from fw import check_no_field


def test_dynamic_buffs_unreachable(client, consts):
    # Item buffs of a dynamic type do not bring it back, so a dynamic type no mapping produces is
    # cleaned along with its buffs
    eve_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_attr_id)])
    client.mk_eve_item(is_dynamic=True, sw_buffs={eve_buff_id: 10})
    # Action
    api_src = client.create_source(cleanup_check=False)
    # Verification
    assert api_src.warnings.adg_cleanup == [
        'cleaned 100.0% of EItem',
        'cleaned 100.0% of EItemGroup',
        'cleaned 100.0% of EBuff',
        'cleaned 100.0% of EItemBuff']


def test_dynamic_buffs_output(client, consts):
    # Item buffs of a dynamic type stay when a mapping produces the type
    eve_attr_id = client.mk_eve_attr()
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_attr_id)])
    eve_mutated_item_id = client.mk_eve_item(is_dynamic=True, sw_buffs={eve_buff_id: 10})
    client.mk_eve_mutator(items=[([client.mk_eve_item()], eve_mutated_item_id)])
    # Action
    api_src = client.create_source(cleanup_check=False)
    # Verification
    with check_no_field():
        api_src.warnings.adg_cleanup  # ruff:ignore[useless-expression]


def test_skipped_mutated_item_absent(client):
    # Mutation item mapping with its mutated type missing is reported as skipped, and the mutator no
    # other mapping keeps is cleaned
    eve_base_item_id = client.mk_eve_item()
    eve_mutated_item_id = client.alloc_item_id()
    client.mk_eve_mutator(items=[([eve_base_item_id], eve_mutated_item_id)])
    # Action
    api_src = client.create_source(cleanup_check=False)
    # Verification
    assert api_src.warnings.adg_cleanup == [
        'cleaned 50.0% of EItem',
        'cleaned 50.0% of EItemGroup',
        'cleaned 100.0% of EMutaItem',
        f'skipped 1 EMutaItem with missing required EItem: {eve_mutated_item_id}']


def test_skipped_mutator_dynamic(client):
    # Mutation item mapping whose mutator type is dynamic is reported as skipped, since the type
    # fails the condition of the mutator key
    eve_data = client.mk_eve_data()
    eve_base_item_id = client.mk_eve_item(datas=[eve_data])
    eve_mutated_item_id = client.mk_eve_item(datas=[eve_data], is_dynamic=True)
    eve_mutator_id = client.mk_eve_item(datas=[eve_data], is_dynamic=True)
    eve_data.mk_mutator(id_=eve_mutator_id, items=[([eve_base_item_id], eve_mutated_item_id)], attrs={})
    # Action
    api_src = client.create_source(data=eve_data, cleanup_check=False)
    # Verification
    assert api_src.warnings.adg_cleanup == [
        'cleaned 66.7% of EItem',
        'cleaned 66.7% of EItemGroup',
        'cleaned 100.0% of EMutaItem',
        f'skipped 1 EMutaItem with missing required EItem: {eve_mutator_id}']
