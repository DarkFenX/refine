from fw import approx, check_no_field


def test_info_modes(client, consts):
    eve_affector_attr_id = client.mk_eve_attr()
    eve_affectee_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_affector_attr_id,
        affectee_attr_id=eve_affectee_attr_id)
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive, mod_info=[eve_mod])
    eve_item_id = client.mk_eve_item(
        attrs={eve_affector_attr_id: 20, eve_affectee_attr_id: 100},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(
        type_id=eve_item_id,
        effect_modes={eve_effect_id: consts.ApiEffMode.force_run})
    api_item_id = api_item.id
    # ID only
    api_item.update(item_info_mode=consts.ApiItemInfoMode.id)
    assert api_item.id == api_item_id
    with check_no_field():
        api_item.kind  # ruff:ignore[useless-expression]
    with check_no_field():
        api_item.type_id  # ruff:ignore[useless-expression]
    with check_no_field():
        api_item.effect_mode_overrides  # ruff:ignore[useless-expression]
    with check_no_field():
        api_item.attrs  # ruff:ignore[useless-expression]
    with check_no_field():
        api_item.effects  # ruff:ignore[useless-expression]
    with check_no_field():
        api_item.mods  # ruff:ignore[useless-expression]
    # Partial
    api_item.update(item_info_mode=consts.ApiItemInfoMode.partial)
    assert api_item.id == api_item_id
    assert api_item.kind == consts.ApiItemKind.module
    assert api_item.type_id == eve_item_id
    assert len(api_item.effect_mode_overrides) == 1
    assert api_item.effect_mode_overrides[eve_effect_id] == consts.ApiEffMode.force_run
    with check_no_field():
        api_item.attrs  # ruff:ignore[useless-expression]
    with check_no_field():
        api_item.effects  # ruff:ignore[useless-expression]
    with check_no_field():
        api_item.mods  # ruff:ignore[useless-expression]
    # Full
    api_item.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert api_item.id == api_item_id
    assert api_item.kind == consts.ApiItemKind.module
    assert api_item.type_id == eve_item_id
    assert len(api_item.effect_mode_overrides) == 1
    assert api_item.effect_mode_overrides[eve_effect_id] == consts.ApiEffMode.force_run
    assert api_item.attrs[eve_affectee_attr_id].modified == approx(120)
    assert api_item.effects[eve_effect_id].running is True
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.force_run
    assert len(api_item.mods.find_by_affector_attr(
        affectee_attr_id=eve_affectee_attr_id,
        affector_attr_id=eve_affector_attr_id)) == 1


def test_info_modes_no_overrides(client, consts):
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.passive)
    eve_item_id = client.mk_eve_item(eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_item = api_fit.add_module(type_id=eve_item_id)
    # Partial
    api_item.update(item_info_mode=consts.ApiItemInfoMode.partial)
    with check_no_field():
        api_item.effect_mode_overrides  # ruff:ignore[useless-expression]
    # Full
    api_item.update(item_info_mode=consts.ApiItemInfoMode.full)
    assert api_item.effects[eve_effect_id].mode == consts.ApiEffMode.full_compliance
    with check_no_field():
        api_item.effect_mode_overrides  # ruff:ignore[useless-expression]
