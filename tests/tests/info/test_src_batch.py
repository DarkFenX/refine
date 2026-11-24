import typing

if typing.TYPE_CHECKING:
    from fw.request import Request


def test_error_cmd_malformed(client, consts):

    def hook_req(req: Request):
        data = req.get_json()
        del data[1]['mutations']
        req.set_json(data=data)

    eve_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.ship)
    api_src = client.create_source()
    # Action & verification
    with api_src.batch(
            hook_req=hook_req,
            status_code=400,
            json_predicate={'code': 'JSN-002', 'message': 're:.+', 'cmd_index': 1},
    ) as api_src_batch:
        api_src_batch.get_items(type_ids=[eve_item_id])
        api_src_batch.get_mutated_items(mutations=[])


def test_error_cmd_type_unknown(client, consts):

    def hook_req(req: Request):
        data = req.get_json()
        data[0]['type'] = 'random'
        req.set_json(data=data)

    eve_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.ship)
    api_src = client.create_source()
    # Action & verification
    with api_src.batch(
            hook_req=hook_req,
            status_code=400,
            json_predicate={'code': 'JSN-002', 'message': 're:.+', 'cmd_index': 0},
    ) as api_src_batch:
        api_src_batch.get_items(type_ids=[eve_item_id])


def test_error_no_src(client):
    api_src = client.create_source()
    client.remove_source(src_alias=api_src.alias)
    # Action & verification
    with api_src.batch(
            status_code=404,
            json_predicate={'code': 'SRC-001', 'message': f'alias "{api_src.alias}" not found'},
    ) as api_src_batch:
        api_src_batch.get_items(type_ids=[])


def test_error_src_alias_malformed(client):
    # Action & verification
    resp = client.source_command_batch_request(src_alias='_..-', commands=[]).send()
    resp.check(
        status_code=404,
        json_predicate={'code': 'SRC-003', 'message': 'alias "_..-" not found: alias is empty after pruning'})


def test_mixed(client, consts):
    # Commands of different kinds are answered in the order they were sent in
    eve_high_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_mid_effect_id = client.mk_eve_effect(id_=consts.EveEffect.med_power)
    eve_ship_id = client.mk_eve_item(cat_id=consts.EveItemCat.ship)
    eve_drone_id = client.mk_eve_item(cat_id=consts.EveItemCat.drone)
    eve_base_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_high_effect_id])
    eve_mutated_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        is_dynamic=True,
        eff_ids=[eve_mid_effect_id])
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    api_src = client.create_source()
    # Action
    with api_src.batch() as api_src_batch:
        api_src_items1 = api_src_batch.get_items(type_ids=[eve_ship_id, eve_base_module_id])
        api_src_mutated_items = api_src_batch.get_mutated_items(mutations=[(eve_base_module_id, eve_mutator_id)])
        api_src_items2 = api_src_batch.get_items(type_ids=[eve_drone_id])
    # Verification
    assert api_src_items1.keys() == {eve_ship_id, eve_base_module_id}
    assert api_src_items1[eve_ship_id].detected_kind == consts.ApiDetectedItemKind.ship
    assert api_src_items1[eve_base_module_id].detected_kind == consts.ApiDetectedItemKind.module_high
    assert api_src_mutated_items.keys() == {eve_base_module_id}
    assert api_src_mutated_items[eve_base_module_id].keys() == {eve_mutator_id}
    api_src_mutated_module = api_src_mutated_items[eve_base_module_id][eve_mutator_id]
    assert api_src_mutated_module.type_id == eve_mutated_module_id
    assert api_src_mutated_module.detected_kind == consts.ApiDetectedItemKind.module_mid
    assert api_src_items2.keys() == {eve_drone_id}
    assert api_src_items2[eve_drone_id].detected_kind == consts.ApiDetectedItemKind.drone


def test_no_cmds(client):
    api_src = client.create_source()
    # Action
    resp = client.source_command_batch_request(src_alias=api_src.alias, commands=[]).send()
    # Verification
    resp.check(status_code=200)
    assert resp.json() == []
