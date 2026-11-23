from fw import check_no_field


def test_optional_reload_set_not_loaded(client, consts):
    eve_d1 = client.mk_eve_data()
    eve_d2 = client.mk_eve_data()
    eve_capacity_attr_id = client.mk_eve_attr(datas=[eve_d1], id_=consts.EveAttr.capacity)
    eve_cycle_time_attr_id = client.mk_eve_attr(datas=[eve_d1])
    eve_effect_id = client.mk_eve_effect(
        datas=[eve_d1],
        id_=consts.EveEffect.fueled_armor_repair,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.alloc_item_id(datas=[eve_d1, eve_d2])
    client.mk_eve_item(
        datas=[eve_d1],
        id_=eve_module_id,
        attrs={eve_capacity_attr_id: 0.32, eve_cycle_time_attr_id: 12000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol(data=eve_d1, default_optional_reloads=consts.ApiOptionalReload.disabled)
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_module.update()
    assert api_module.optional_reload_effective == consts.ApiOptionalReload.disabled
    with check_no_field():
        api_module.optional_reload_override  # ruff:ignore[useless-expression]
    # Action
    api_sol.change_src(data=eve_d2)
    # Verification
    api_module.update()
    with check_no_field():
        api_module.optional_reload_effective  # ruff:ignore[useless-expression]
    with check_no_field():
        api_module.optional_reload_override  # ruff:ignore[useless-expression]
    # Action
    api_module.change_module(optional_reload_override=consts.ApiOptionalReload.on_empty)
    # Verification
    api_module.update()
    with check_no_field():
        api_module.optional_reload_effective  # ruff:ignore[useless-expression]
    assert api_module.optional_reload_override == consts.ApiOptionalReload.on_empty
    # Action
    api_sol.change_src(data=eve_d1)
    # Verification
    api_module.update()
    assert api_module.optional_reload_effective == consts.ApiOptionalReload.on_empty
    assert api_module.optional_reload_override == consts.ApiOptionalReload.on_empty


def test_optional_reload_charge_use(client, consts):
    # The setting is used only by modules which can hold charges and can cycle without them
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_optional_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fueled_armor_repair,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_required_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.use_missiles,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module1_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.32, eve_cycle_time_attr_id: 12000},
        eff_ids=[eve_optional_effect_id],
        defeff_id=eve_optional_effect_id)
    eve_module2_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 0.32, eve_cycle_time_attr_id: 12000},
        eff_ids=[eve_required_effect_id],
        defeff_id=eve_required_effect_id)
    eve_module3_id = client.mk_eve_item(
        attrs={eve_cycle_time_attr_id: 12000},
        eff_ids=[eve_optional_effect_id],
        defeff_id=eve_optional_effect_id)
    client.create_sources()
    api_sol = client.create_sol(default_optional_reloads=consts.ApiOptionalReload.disabled)
    api_fit = api_sol.create_fit()
    # Action
    api_module = api_fit.add_module(type_id=eve_module1_id)
    # Verification
    api_module.update()
    assert api_module.optional_reload_effective == consts.ApiOptionalReload.disabled
    with check_no_field():
        api_module.optional_reload_override  # ruff:ignore[useless-expression]
    # Action
    api_module.change_module(optional_reload_override=consts.ApiOptionalReload.on_empty)
    # Verification
    api_module.update()
    assert api_module.optional_reload_effective == consts.ApiOptionalReload.on_empty
    assert api_module.optional_reload_override == consts.ApiOptionalReload.on_empty
    # Action
    api_module.change_module(type_id=eve_module2_id)
    # Verification - override is kept, but nothing uses it when the module needs charges to cycle
    api_module.update()
    with check_no_field():
        api_module.optional_reload_effective  # ruff:ignore[useless-expression]
    assert api_module.optional_reload_override == consts.ApiOptionalReload.on_empty
    # Action
    api_module.change_module(type_id=eve_module3_id)
    # Verification - same when the module has no room for charges
    api_module.update()
    with check_no_field():
        api_module.optional_reload_effective  # ruff:ignore[useless-expression]
    assert api_module.optional_reload_override == consts.ApiOptionalReload.on_empty
    # Action
    api_module.change_module(type_id=eve_module1_id)
    # Verification
    api_module.update()
    assert api_module.optional_reload_effective == consts.ApiOptionalReload.on_empty
    assert api_module.optional_reload_override == consts.ApiOptionalReload.on_empty
