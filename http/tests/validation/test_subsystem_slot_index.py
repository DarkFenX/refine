from tests import check_no_field


def test_multiple(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_subsystem2 = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_index])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {1: sorted([api_subsystem1.id, api_subsystem2.id])}
    # Action
    api_subsystem1.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_different_slots(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem1_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem, attrs={eve_slot_attr_id: 1})
    eve_subsystem2_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem, attrs={eve_slot_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem1_id)
    api_fit.add_subsystem(type_id=eve_subsystem2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_subsystem3 = api_fit.add_subsystem(type_id=eve_subsystem1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_index])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {1: sorted([api_subsystem1.id, api_subsystem3.id])}


def test_rounding(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem1_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem, attrs={eve_slot_attr_id: 1.2})
    eve_subsystem2_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem, attrs={eve_slot_attr_id: 1.4})
    eve_subsystem3_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem, attrs={eve_slot_attr_id: 1.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem1_id)
    api_subsystem2 = api_fit.add_subsystem(type_id=eve_subsystem2_id)
    api_fit.add_subsystem(type_id=eve_subsystem3_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_index])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {1: sorted([api_subsystem1.id, api_subsystem2.id])}


def test_no_attr(client, consts):
    eve_slot_attr_id = consts.EveAttr.subsystem_slot
    eve_subsystem_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem_id)
    api_subsystem2 = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_index])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {1: sorted([api_subsystem1.id, api_subsystem2.id])}


def test_no_value(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem)
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_slot_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_user(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem_id = client.alloc_item_id()
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_slot_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_state(client, consts):
    # Disabled subsystems still compete for slots
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem_id, state=False)
    api_subsystem2 = api_fit.add_subsystem(type_id=eve_subsystem_id, state=False)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_index])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_index == {1: sorted([api_subsystem1.id, api_subsystem2.id])}


def test_criterion_item_category(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_subsystem_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_type(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.subsystem_slot)
    eve_rig_id = client.mk_eve_item(cat_id=consts.EveItemCat.subsystem, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_rig(type_id=eve_rig_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
