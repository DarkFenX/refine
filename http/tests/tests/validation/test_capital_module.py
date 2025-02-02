from tests import check_no_field


def test_main(client, consts):
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.capital_ships)
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_cap_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        attrs={eve_attr_id: 4000},
        eff_ids=[eve_effect_id])
    eve_subcap_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        attrs={eve_attr_id: 3500},
        eff_ids=[eve_effect_id])
    eve_subcap_ship_id = client.mk_eve_ship()
    eve_cap_ship_id = client.mk_eve_ship(srqs={eve_skill_id: 1})
    eve_struct_id = client.mk_eve_struct()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_subcap_ship_id)
    api_cap_module = api_fit.add_mod(type_id=eve_cap_module_id)
    api_fit.add_mod(type_id=eve_subcap_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is False
    assert api_val.details.capital_module == [api_cap_module.id]
    # Action
    api_fit.set_ship(type_id=eve_cap_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.set_ship(type_id=eve_subcap_ship_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is False
    assert api_val.details.capital_module == [api_cap_module.id]


def test_multiple(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        attrs={eve_attr_id: 4000},
        eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_mod(type_id=eve_module_id)
    api_module2 = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is False
    assert api_val.details.capital_module == sorted([api_module1.id, api_module2.id])


def test_no_ship(client, consts):
    eve_attr_id = consts.EveAttr.volume
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        attrs={eve_attr_id: 4000},
        eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_value(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_skill(client, consts):
    eve_skill_id = consts.EveItem.capital_ships
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        attrs={eve_attr_id: 4000},
        eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship(srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_attr(client, consts):
    eve_attr_id = consts.EveAttr.volume
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        attrs={eve_attr_id: 4000},
        eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is False
    assert api_val.details.capital_module == [api_module.id]


def test_not_loaded_ship(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        attrs={eve_attr_id: 4000},
        eff_ids=[eve_effect_id])
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_module(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_module_id = client.alloc_item_id()
    eve_ship_id = client.mk_eve_ship()
    # Create an item which has the effect, just to prevent the effect from being cleaned up
    client.mk_eve_item(eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_state(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_module_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        attrs={eve_attr_id: 4000},
        eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiState.ghost)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is False
    assert api_val.details.capital_module == [api_module.id]


def test_criterion_module_effect(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.hi_power)
    eve_module1_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_attr_id: 4000})
    eve_module2_id = client.mk_eve_item(
        cat_id=consts.EveItemCat.module,
        attrs={eve_attr_id: 4000},
        eff_ids=[eve_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_mod(type_id=eve_module1_id)
    api_module2 = api_fit.add_mod(type_id=eve_module2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is False
    assert api_val.details.capital_module == [api_module2.id]
    # Action
    api_module2.change_mod(effect_modes={eve_effect_id: consts.ApiEffMode.force_stop})
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is False
    assert api_val.details.capital_module == [api_module2.id]

