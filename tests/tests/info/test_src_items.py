import typing

from fw import Absent, check_no_field

if typing.TYPE_CHECKING:
    from fw.request import Request


def test_error_body_malformed(client):
    api_src = client.create_source()
    # Action & verification
    api_src.get_items(type_ids='random', status_code=400, json_predicate={'code': 'JSN-001', 'message': 're:.+'})


def test_error_no_src(client):
    api_src = client.create_source()
    client.remove_source(src_alias=api_src.alias)
    # Action & verification
    api_src.get_items(
        type_ids=[],
        status_code=404,
        json_predicate={'code': 'SRC-001', 'message': f'alias "{api_src.alias}" not found'})


def test_error_src_alias_malformed(client):

    def hook_req(req: Request):
        req.set_url(url=req.get_url().replace(f'/src/{api_src.alias}/', '/src/_..-/'))

    api_src = client.create_source()
    # Action & verification
    api_src.get_items(
        type_ids=[],
        hook_req=hook_req,
        status_code=404,
        json_predicate={'code': 'SRC-003', 'message': 'alias "_..-" not found: alias is empty after pruning'})


def test_detected_kind(client, consts):
    eve_booster_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_implant_attr_id = client.mk_eve_attr(id_=consts.EveAttr.implantness)
    eve_high_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_mid_effect_id = client.mk_eve_effect(id_=consts.EveEffect.med_power)
    eve_low_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lo_power)
    eve_rig_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot)
    eve_service_effect_id = client.mk_eve_effect(id_=consts.EveEffect.service_slot)
    eve_booster_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_booster_attr_id: 1})
    eve_character_id = client.mk_eve_item(grp_id=consts.EveItemGrp.character)
    eve_charge_id = client.mk_eve_item(cat_id=consts.EveItemCat.charge)
    eve_drone_id = client.mk_eve_item(cat_id=consts.EveItemCat.drone)
    eve_fighter_id = client.mk_eve_item(cat_id=consts.EveItemCat.fighter)
    eve_implant_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_implant_attr_id: 1})
    eve_high_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_high_effect_id])
    eve_mid_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_mid_effect_id])
    eve_low_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_low_effect_id])
    eve_rig_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_rig_effect_id])
    eve_service_id = client.mk_eve_item(cat_id=consts.EveItemCat.structure_module, eff_ids=[eve_service_effect_id])
    eve_ship_id = client.mk_eve_item(cat_id=consts.EveItemCat.ship)
    eve_skill_id = client.mk_eve_item(cat_id=consts.EveItemCat.skill)
    eve_stance_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, grp_id=consts.EveItemGrp.ship_modifier)
    eve_subsystem_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem)
    api_src = client.create_source()
    # Action
    api_src_items = api_src.get_items(type_ids=[
        eve_booster_id, eve_character_id, eve_charge_id, eve_drone_id, eve_fighter_id, eve_implant_id,
        eve_high_module_id, eve_mid_module_id, eve_low_module_id, eve_rig_id, eve_service_id, eve_ship_id,
        eve_skill_id, eve_stance_id, eve_subsystem_id])
    # Verification
    assert len(api_src_items) == 15
    assert api_src_items[eve_booster_id].detected_kind == consts.ApiDetectedItemKind.booster
    assert api_src_items[eve_character_id].detected_kind == consts.ApiDetectedItemKind.character
    assert api_src_items[eve_charge_id].detected_kind == consts.ApiDetectedItemKind.charge
    assert api_src_items[eve_drone_id].detected_kind == consts.ApiDetectedItemKind.drone
    assert api_src_items[eve_fighter_id].detected_kind == consts.ApiDetectedItemKind.fighter
    assert api_src_items[eve_implant_id].detected_kind == consts.ApiDetectedItemKind.implant
    assert api_src_items[eve_high_module_id].detected_kind == consts.ApiDetectedItemKind.module_high
    assert api_src_items[eve_mid_module_id].detected_kind == consts.ApiDetectedItemKind.module_mid
    assert api_src_items[eve_low_module_id].detected_kind == consts.ApiDetectedItemKind.module_low
    assert api_src_items[eve_rig_id].detected_kind == consts.ApiDetectedItemKind.rig
    assert api_src_items[eve_service_id].detected_kind == consts.ApiDetectedItemKind.service
    assert api_src_items[eve_ship_id].detected_kind == consts.ApiDetectedItemKind.ship
    assert api_src_items[eve_skill_id].detected_kind == consts.ApiDetectedItemKind.skill
    assert api_src_items[eve_stance_id].detected_kind == consts.ApiDetectedItemKind.stance
    assert api_src_items[eve_subsystem_id].detected_kind == consts.ApiDetectedItemKind.subsystem


def test_detected_kind_none(client, consts):
    # Types which match no kind or several kinds are reported, but without kind and max state
    eve_high_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_mid_effect_id = client.mk_eve_effect(id_=consts.EveEffect.med_power)
    eve_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_no_match_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_active_effect_id])
    eve_multi_match_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        eff_ids=[eve_high_effect_id, eve_mid_effect_id, eve_active_effect_id])
    api_src = client.create_source()
    # Action
    api_src_items = api_src.get_items(type_ids=[eve_no_match_id, eve_multi_match_id])
    # Verification
    assert api_src_items.keys() == {eve_no_match_id, eve_multi_match_id}
    with check_no_field():
        api_src_items[eve_no_match_id].detected_kind  # ruff:ignore[useless-expression]
    with check_no_field():
        api_src_items[eve_no_match_id].module_max_state  # ruff:ignore[useless-expression]
    with check_no_field():
        api_src_items[eve_multi_match_id].detected_kind  # ruff:ignore[useless-expression]
    with check_no_field():
        api_src_items[eve_multi_match_id].module_max_state  # ruff:ignore[useless-expression]


def test_module_max_state(client, consts):
    eve_high_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_mid_effect_id = client.mk_eve_effect(id_=consts.EveEffect.med_power)
    eve_low_effect_id = client.mk_eve_effect(id_=consts.EveEffect.lo_power)
    eve_online_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online)
    eve_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_overload_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.overload)
    eve_offline_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_high_effect_id])
    eve_online_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        eff_ids=[eve_high_effect_id, eve_online_effect_id])
    eve_active_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        eff_ids=[eve_mid_effect_id, eve_online_effect_id, eve_active_effect_id])
    eve_overload_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.structure_module,
        eff_ids=[eve_low_effect_id, eve_online_effect_id, eve_active_effect_id, eve_overload_effect_id])
    api_src = client.create_source()
    # Action
    api_src_items = api_src.get_items(type_ids=[
        eve_offline_module_id, eve_online_module_id, eve_active_module_id, eve_overload_module_id])
    # Verification
    assert len(api_src_items) == 4
    assert api_src_items[eve_offline_module_id].detected_kind == consts.ApiDetectedItemKind.module_high
    assert api_src_items[eve_offline_module_id].module_max_state == consts.ApiModuleState.offline
    assert api_src_items[eve_online_module_id].detected_kind == consts.ApiDetectedItemKind.module_high
    assert api_src_items[eve_online_module_id].module_max_state == consts.ApiModuleState.online
    assert api_src_items[eve_active_module_id].detected_kind == consts.ApiDetectedItemKind.module_mid
    assert api_src_items[eve_active_module_id].module_max_state == consts.ApiModuleState.active
    assert api_src_items[eve_overload_module_id].detected_kind == consts.ApiDetectedItemKind.module_low
    assert api_src_items[eve_overload_module_id].module_max_state == consts.ApiModuleState.overload


def test_module_max_state_other_kinds(client, consts):
    # Max state is exposed only for types detected as modules, whatever effects other types have
    eve_rig_effect_id = client.mk_eve_effect(id_=consts.EveEffect.rig_slot)
    eve_service_effect_id = client.mk_eve_effect(id_=consts.EveEffect.service_slot)
    eve_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_drone_id = client.mk_eve_item(cat_id=consts.EveItemCat.drone, eff_ids=[eve_active_effect_id])
    eve_rig_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        eff_ids=[eve_rig_effect_id, eve_active_effect_id])
    eve_service_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.structure_module,
        eff_ids=[eve_service_effect_id, eve_active_effect_id])
    api_src = client.create_source()
    # Action
    api_src_items = api_src.get_items(type_ids=[eve_drone_id, eve_rig_id, eve_service_id])
    # Verification
    assert len(api_src_items) == 3
    assert api_src_items[eve_drone_id].detected_kind == consts.ApiDetectedItemKind.drone
    with check_no_field():
        api_src_items[eve_drone_id].module_max_state  # ruff:ignore[useless-expression]
    assert api_src_items[eve_rig_id].detected_kind == consts.ApiDetectedItemKind.rig
    with check_no_field():
        api_src_items[eve_rig_id].module_max_state  # ruff:ignore[useless-expression]
    assert api_src_items[eve_service_id].detected_kind == consts.ApiDetectedItemKind.service
    with check_no_field():
        api_src_items[eve_service_id].module_max_state  # ruff:ignore[useless-expression]


def test_unknown_type(client, consts):
    eve_item_id = client.mk_eve_item(cat_id=consts.EveItemCat.ship)
    eve_other_id = client.alloc_item_id()
    api_src = client.create_source()
    # Action
    api_src_items = api_src.get_items(type_ids=[eve_other_id, eve_item_id])
    # Verification - types the source does not have are not reported
    assert api_src_items.keys() == {eve_item_id}
    # Action
    api_src_items = api_src.get_items(type_ids=[eve_other_id])
    # Verification
    assert len(api_src_items) == 0
    # Action
    api_src_items = api_src.get_items(type_ids=[])
    # Verification
    assert len(api_src_items) == 0


def test_all(client, consts):
    # Without type IDs all types of the source are reported
    eve_high_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active)
    eve_ship_id = client.mk_eve_item(cat_id=consts.EveItemCat.ship)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_high_effect_id])
    eve_no_match_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_active_effect_id])
    api_src = client.create_source()
    # Action
    api_src_items = api_src.get_items(type_ids=Absent)
    # Verification
    assert api_src_items.keys() == {eve_ship_id, eve_module_id, eve_no_match_id}
    assert api_src_items[eve_ship_id].detected_kind == consts.ApiDetectedItemKind.ship
    assert api_src_items[eve_module_id].detected_kind == consts.ApiDetectedItemKind.module_high
    assert api_src_items[eve_module_id].module_max_state == consts.ApiModuleState.offline
    with check_no_field():
        api_src_items[eve_no_match_id].detected_kind  # ruff:ignore[useless-expression]


def test_dynamic_type(client, consts):
    # Dynamic types exist only as a result of mutation, the source does not have them on their own
    eve_high_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_base_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_high_effect_id])
    eve_mutated_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        is_dynamic=True,
        eff_ids=[eve_high_effect_id])
    client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    api_src = client.create_source()
    # Action
    api_src_items = api_src.get_items(type_ids=[eve_base_module_id, eve_mutated_module_id])
    # Verification
    assert api_src_items.keys() == {eve_base_module_id}
    assert api_src_items[eve_base_module_id].detected_kind == consts.ApiDetectedItemKind.module_high
    # Action
    api_src_items = api_src.get_items(type_ids=Absent)
    # Verification
    assert eve_mutated_module_id not in api_src_items


def test_multiple_sources(client, consts):
    # Each source reports its own data
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_item1_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    eve_item2_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(datas=[eve_d1], id_=eve_item1_id, cat_id=consts.EveItemCat.ship)
    client.mk_eve_item(datas=[eve_d2], id_=eve_item1_id, cat_id=consts.EveItemCat.drone)
    client.mk_eve_item(datas=[eve_d1], id_=eve_item2_id, cat_id=consts.EveItemCat.charge)
    api_src1 = client.create_source(data=eve_d1)
    api_src2 = client.create_source(data=eve_d2)
    # Action
    api_src1_items = api_src1.get_items(type_ids=[eve_item1_id, eve_item2_id])
    api_src2_items = api_src2.get_items(type_ids=[eve_item1_id, eve_item2_id])
    # Verification
    assert api_src1_items.keys() == {eve_item1_id, eve_item2_id}
    assert api_src1_items[eve_item1_id].detected_kind == consts.ApiDetectedItemKind.ship
    assert api_src1_items[eve_item2_id].detected_kind == consts.ApiDetectedItemKind.charge
    assert api_src2_items.keys() == {eve_item1_id}
    assert api_src2_items[eve_item1_id].detected_kind == consts.ApiDetectedItemKind.drone
