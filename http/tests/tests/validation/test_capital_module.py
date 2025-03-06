from tests import approx, check_no_field


def test_main(client, consts):
    # Test regular module and capital volume on regular ship, capital ship and structure
    eve_skill_id = client.mk_eve_item(id_=consts.EveItem.capital_ships)
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_cap_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_attr_id: 4000})
    eve_subcap_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_attr_id: 50})
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
    assert api_val.details.capital_module == (3500, {api_cap_module.id: 4000})
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
    assert api_val.details.capital_module == (3500, {api_cap_module.id: 4000})


def test_multiple(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_attr_id: 4000})
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
    assert api_val.details.capital_module == (3500, {api_module1.id: 4000, api_module2.id: 4000})


def test_modified(client, consts):
    # Unrealistic scenario, but validation takes unmodified volume value
    eve_vol_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_vol_attr_id: 3000})
    eve_ship_id = client.mk_eve_ship()
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_vol_attr_id)
    eve_implant_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1000}, eff_ids=[eve_implant_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    assert api_module.update().attrs[eve_vol_attr_id].extra == approx(4000)
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_implant.remove()
    # Verification
    assert api_module.update().attrs[eve_vol_attr_id].extra == approx(3000)
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_mutation(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_base_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_attr_id: 3000})
    eve_mutated_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_attr_id: 4000})
    eve_mutator_id = client.mk_eve_mutator(items=[([eve_base_module_id], eve_mutated_module_id)])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_base_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module.change_mod(mutation=eve_mutator_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is False
    assert api_val.details.capital_module == (3500, {api_module.id: 4000})
    # Action
    api_module.change_mod(mutation=None)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_ship(client, consts):
    eve_attr_id = consts.EveAttr.volume
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_attr_id: 4000})
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
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module)
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
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_attr_id: 4000})
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
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_attr_id: 4000})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is False
    assert api_val.details.capital_module == (3500, {api_module.id: 4000})


def test_not_loaded_ship(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_attr_id: 4000})
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
    eve_module_id = client.alloc_item_id()
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


def test_criterion_state(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_attr_id: 4000})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_mod(type_id=eve_module_id, state=consts.ApiModuleState.ghost)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is False
    assert api_val.details.capital_module == (3500, {api_module.id: 4000})


def test_criterion_volume(client, consts):
    # Threshold for capital modules is >3500. The value has most likely been taken from EVE client
    # source, but right now can't say for sure if it's from there, and when it was
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_cap_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_attr_id: 3500.1})
    eve_subcap_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_attr_id: 3500})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_cap_module = api_fit.add_mod(type_id=eve_cap_module_id)
    api_fit.add_mod(type_id=eve_subcap_module_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is False
    assert api_val.details.capital_module == (3500, {api_cap_module.id: 3500.1})


def test_criterion_item_category(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_module_id = client.mk_eve_item(cat_id=consts.EveItemCat.structure_module, attrs={eve_attr_id: 4000})
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


def test_criterion_item_kind(client, consts):
    eve_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_rig_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_attr_id: 4000})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.capital_module])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
