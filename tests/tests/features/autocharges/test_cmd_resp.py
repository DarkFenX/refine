from fw import check_no_field
from fw.api.types.helpers import effect_fw_to_http


def test_fit_batch_add(client, consts):
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_autocharge_id = client.mk_eve_item()
    eve_fighter_with_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_autocharge_id},
        eff_ids=[eve_effect_id])
    eve_fighter_without_id = client.mk_eve_fighter()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    with api_fit.batch(json_predicate=[
            {'autocharge_item_ids': {}},
            {'autocharge_item_ids': {effect_fw_to_http(eve_effect_id): 're:.+'}},
    ]) as api_fit_batch:
        api_fighter_without = api_fit_batch.add_fighter(type_id=eve_fighter_without_id)
        api_fighter_with = api_fit_batch.add_fighter(type_id=eve_fighter_with_id)
    api_fighter_without.update(item_info_mode=consts.ApiItemInfoMode.id)
    with check_no_field():
        api_fighter_without.autocharges  # ruff:ignore[useless-expression]
    api_fighter_with.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert len(api_fighter_with.autocharges) == 1


def test_fit_batch_change(client, consts):
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_autocharge_id = client.mk_eve_item()
    eve_fighter_with_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_autocharge_id},
        eff_ids=[eve_effect_id])
    eve_fighter_without_id = client.mk_eve_fighter()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter_without_id)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter_with_id)
    # Verification
    with api_fit.batch(json_predicate=[
            {'autocharge_item_ids': {effect_fw_to_http(eve_effect_id): 're:.+'}},
            {'autocharge_item_ids': {}},
    ]) as api_fit_batch:
        api_fit_batch.change_fighter(item_id=api_fighter1.id, type_id=eve_fighter_with_id)
        api_fit_batch.change_fighter(item_id=api_fighter2.id, type_id=eve_fighter_without_id)
    api_fighter1.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert len(api_fighter1.autocharges) == 1
    api_fighter2.update(item_info_mode=consts.ApiItemInfoMode.id)
    with check_no_field():
        api_fighter2.autocharges  # ruff:ignore[useless-expression]


def test_sol_batch_add(client, consts):
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_autocharge_id = client.mk_eve_item()
    eve_fighter_with_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_autocharge_id},
        eff_ids=[eve_effect_id])
    eve_fighter_without_id = client.mk_eve_fighter()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    with api_sol.batch(json_predicate=[
            {'autocharge_item_ids': {}},
            {'autocharge_item_ids': {effect_fw_to_http(eve_effect_id): 're:.+'}},
    ]) as api_sol_batch:
        api_fighter_without = api_sol_batch.add_fighter(fit_id=api_fit.id, type_id=eve_fighter_without_id)
        api_fighter_with = api_sol_batch.add_fighter(fit_id=api_fit.id, type_id=eve_fighter_with_id)
    api_fighter_without.update(item_info_mode=consts.ApiItemInfoMode.id)
    with check_no_field():
        api_fighter_without.autocharges  # ruff:ignore[useless-expression]
    api_fighter_with.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert len(api_fighter_with.autocharges) == 1


def test_sol_batch_change(client, consts):
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.ftr_abil_launch_bomb, cat_id=consts.EveEffCat.active)
    eve_autocharge_id = client.mk_eve_item()
    eve_fighter_with_id = client.mk_eve_fighter(
        attrs={eve_autocharge_attr_id: eve_autocharge_id},
        eff_ids=[eve_effect_id])
    eve_fighter_without_id = client.mk_eve_fighter()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter_without_id)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter_with_id)
    # Verification
    with api_sol.batch(json_predicate=[
            {'autocharge_item_ids': {effect_fw_to_http(eve_effect_id): 're:.+'}},
            {'autocharge_item_ids': {}},
    ]) as api_sol_batch:
        api_sol_batch.change_fighter(item_id=api_fighter1.id, type_id=eve_fighter_with_id)
        api_sol_batch.change_fighter(item_id=api_fighter2.id, type_id=eve_fighter_without_id)
    api_fighter1.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert len(api_fighter1.autocharges) == 1
    api_fighter2.update(item_info_mode=consts.ApiItemInfoMode.id)
    with check_no_field():
        api_fighter2.autocharges  # ruff:ignore[useless-expression]
