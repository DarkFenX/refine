import json

from fw import check_no_field


def test_error_body_malformed(client):
    api_src = client.create_source()
    # Action & verification
    api_src.get_mutated_items(
        mutations='random',
        status_code=400,
        json_predicate={'code': 'JSN-001', 'message': 're:.+'})


def test_error_no_src(client):
    api_src = client.create_source()
    client.remove_source(src_alias=api_src.alias)
    # Action & verification
    api_src.get_mutated_items(
        mutations=[],
        status_code=404,
        json_predicate={'code': 'SRC-001', 'message': f'alias "{api_src.alias}" not found'})


def test_error_src_alias_malformed(client):
    # Action & verification
    resp = client.get_source_mutated_items_request(src_alias='_..-', mutations=[]).send()
    resp.check(
        status_code=404,
        json_predicate={'code': 'SRC-003', 'message': 'alias "_..-" not found: alias is empty after pruning'})


def test_detected_kind(client, consts):
    # Kind is detected using data of the mutated type, base type does not matter
    eve_high_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_mid_effect_id = client.mk_eve_effect(id_=consts.EveEffect.med_power)
    eve_base_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_high_effect_id])
    eve_mutated_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        is_dynamic=True,
        eff_ids=[eve_mid_effect_id])
    eve_base_drone_id = client.mk_eve_item(cat_id=consts.EveItemCat.drone)
    eve_mutated_drone_id = client.mk_eve_item(cat_id=consts.EveItemCat.drone, is_dynamic=True)
    eve_base_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_high_effect_id])
    eve_mutated_other_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, is_dynamic=True)
    eve_mutator_id = client.mk_eve_mutator(items=[
        ([eve_base_module_id], eve_mutated_module_id),
        ([eve_base_drone_id], eve_mutated_drone_id),
        ([eve_base_other_id], eve_mutated_other_id)])
    api_src = client.create_source()
    # Action
    api_src_items = api_src.get_mutated_items(mutations=[
        (eve_base_module_id, eve_mutator_id),
        (eve_base_drone_id, eve_mutator_id),
        (eve_base_other_id, eve_mutator_id)])
    # Verification
    assert api_src_items.keys() == {eve_base_module_id, eve_base_drone_id, eve_base_other_id}
    assert api_src_items[eve_base_module_id].keys() == {eve_mutator_id}
    assert api_src_items[eve_base_module_id][eve_mutator_id].type_id == eve_mutated_module_id
    assert api_src_items[eve_base_module_id][eve_mutator_id].detected_kind == consts.ApiValItemType.module_mid
    assert api_src_items[eve_base_drone_id].keys() == {eve_mutator_id}
    assert api_src_items[eve_base_drone_id][eve_mutator_id].type_id == eve_mutated_drone_id
    assert api_src_items[eve_base_drone_id][eve_mutator_id].detected_kind == consts.ApiValItemType.drone
    assert api_src_items[eve_base_other_id].keys() == {eve_mutator_id}
    api_src_other = api_src_items[eve_base_other_id][eve_mutator_id]
    assert api_src_other.type_id == eve_mutated_other_id
    with check_no_field():
        api_src_other.detected_kind  # ruff:ignore[useless-expression]
    with check_no_field():
        api_src_other.module_max_state  # ruff:ignore[useless-expression]


def test_module_max_state(client, consts):
    # Max state is taken from the mutated type as well, and only when it is detected as a module
    eve_high_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_base_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_high_effect_id])
    eve_mutated_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        is_dynamic=True,
        eff_ids=[eve_high_effect_id, eve_active_effect_id])
    eve_base_drone_id = client.mk_eve_item(cat_id=consts.EveItemCat.drone)
    eve_mutated_drone_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.drone,
        is_dynamic=True,
        eff_ids=[eve_active_effect_id])
    eve_mutator_id = client.mk_eve_mutator(items=[
        ([eve_base_module_id], eve_mutated_module_id),
        ([eve_base_drone_id], eve_mutated_drone_id)])
    api_src = client.create_source()
    # Action
    api_src_items = api_src.get_items(type_ids=[eve_base_module_id])
    api_src_mutated_items = api_src.get_mutated_items(mutations=[
        (eve_base_module_id, eve_mutator_id),
        (eve_base_drone_id, eve_mutator_id)])
    # Verification
    assert api_src_items[eve_base_module_id].module_max_state == consts.ApiModuleState.offline
    assert api_src_mutated_items.keys() == {eve_base_module_id, eve_base_drone_id}
    assert api_src_mutated_items[eve_base_module_id][eve_mutator_id].type_id == eve_mutated_module_id
    assert api_src_mutated_items[eve_base_module_id][eve_mutator_id].module_max_state == consts.ApiModuleState.active
    api_src_drone = api_src_mutated_items[eve_base_drone_id][eve_mutator_id]
    assert api_src_drone.type_id == eve_mutated_drone_id
    with check_no_field():
        api_src_drone.module_max_state  # ruff:ignore[useless-expression]


def test_multiple_mutations(client, consts):
    # Results are grouped by base type, then by mutator
    eve_base1_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_base2_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_mutated1_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, is_dynamic=True)
    eve_mutated2_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, is_dynamic=True)
    eve_mutated3_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, is_dynamic=True)
    eve_mutator1_id = client.mk_eve_mutator(items=[
        ([eve_base1_id], eve_mutated1_id),
        ([eve_base2_id], eve_mutated2_id)])
    eve_mutator2_id = client.mk_eve_mutator(items=[([eve_base1_id], eve_mutated3_id)])
    api_src = client.create_source()
    # Action
    api_src_items = api_src.get_mutated_items(mutations=[
        (eve_base1_id, eve_mutator1_id),
        (eve_base2_id, eve_mutator1_id),
        (eve_base1_id, eve_mutator2_id)])
    # Verification
    assert api_src_items.keys() == {eve_base1_id, eve_base2_id}
    assert api_src_items[eve_base1_id].keys() == {eve_mutator1_id, eve_mutator2_id}
    assert api_src_items[eve_base1_id][eve_mutator1_id].type_id == eve_mutated1_id
    assert api_src_items[eve_base1_id][eve_mutator2_id].type_id == eve_mutated3_id
    assert api_src_items[eve_base2_id].keys() == {eve_mutator1_id}
    assert api_src_items[eve_base2_id][eve_mutator1_id].type_id == eve_mutated2_id


def test_unknown_mutation(client, consts):
    eve_base_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_mutated_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, is_dynamic=True)
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_id], eve_mutated_id)])
    eve_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_other_id = client.alloc_item_id()
    api_src = client.create_source()
    # Action
    api_src_items = api_src.get_mutated_items(mutations=[
        (eve_other_id, eve_mutator_id),
        (eve_base_id, eve_other_id),
        (eve_item_id, eve_mutator_id),
        (eve_base_id, eve_mutator_id)])
    # Verification - unknown base types, unknown mutators, and base types the mutator does not
    # accept are not reported
    assert api_src_items.keys() == {eve_base_id}
    assert api_src_items[eve_base_id].keys() == {eve_mutator_id}
    assert api_src_items[eve_base_id][eve_mutator_id].type_id == eve_mutated_id
    # Action
    api_src_items = api_src.get_mutated_items(mutations=[(eve_item_id, eve_mutator_id)])
    # Verification
    assert len(api_src_items) == 0
    # Action
    api_src_items = api_src.get_mutated_items(mutations=[])
    # Verification
    assert len(api_src_items) == 0


def test_repeated_mutation(client, consts):
    eve_base_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
    eve_mutated_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, is_dynamic=True)
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_id], eve_mutated_id)])
    api_src = client.create_source()
    # Action
    resp = client.get_source_mutated_items_request(
        src_alias=api_src.alias,
        mutations=[(eve_base_id, eve_mutator_id), (eve_base_id, eve_mutator_id)]).send()
    # Verification - parsing JSON into a dictionary would hide repeated keys, so check key-value
    # pairs
    resp.check(status_code=200)
    api_src_items = json.loads(resp.text, object_pairs_hook=list)
    assert api_src_items == [(str(eve_base_id), [(str(eve_mutator_id), [('type_id', eve_mutated_id)])])]
