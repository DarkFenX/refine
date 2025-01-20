from tests import check_no_field


def test_amount(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_booster_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster1 = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.booster_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_booster2 = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.booster_slot_index])
    assert api_val.passed is False
    assert api_val.details.booster_slot_index == {1: sorted([api_booster1.id, api_booster2.id])}
    # Action
    api_booster1.remove()
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.booster_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_different_slots(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_booster1_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1})
    eve_booster2_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster1 = api_fit.add_booster(type_id=eve_booster1_id)
    api_fit.add_booster(type_id=eve_booster2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.booster_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
    # Action
    api_booster3 = api_fit.add_booster(type_id=eve_booster1_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.booster_slot_index])
    assert api_val.passed is False
    assert api_val.details.booster_slot_index == {1: sorted([api_booster1.id, api_booster3.id])}


def test_no_attr(client, consts):
    eve_slot_attr_id = consts.EveAttr.boosterness
    eve_booster_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster1 = api_fit.add_booster(type_id=eve_booster_id)
    api_booster2 = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.booster_slot_index])
    assert api_val.passed is False
    assert api_val.details.booster_slot_index == {1: sorted([api_booster1.id, api_booster2.id])}


def test_criterion_state(client, consts):
    # Disabled boosters still compete for slots
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_booster_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster1 = api_fit.add_booster(type_id=eve_booster_id, state=False)
    api_booster2 = api_fit.add_booster(type_id=eve_booster_id, state=False)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.booster_slot_index])
    assert api_val.passed is False
    assert api_val.details.booster_slot_index == {1: sorted([api_booster1.id, api_booster2.id])}


def test_criterion_item_category(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_booster_id = client.mk_eve_item(cat_id=consts.EveItemCat.module, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_booster_id)
    api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.booster_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104


def test_criterion_item_type(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_implant_id = client.mk_eve_item(cat_id=consts.EveItemCat.implant, attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.booster_slot_index])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # pylint: disable=W0104
