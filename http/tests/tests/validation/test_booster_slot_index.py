from tests import approx, check_no_field
from tests.fw.api import ValOptions


def test_multiple(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_booster_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster1 = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_booster2 = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.booster_slot_index == {1: sorted([api_booster1.id, api_booster2.id])}
    # Action
    api_booster1.remove()
    # Verification
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_different_slots(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_booster1_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    eve_booster2_id = client.mk_eve_item(attrs={eve_slot_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster1 = api_fit.add_booster(type_id=eve_booster1_id)
    api_fit.add_booster(type_id=eve_booster2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_booster3 = api_fit.add_booster(type_id=eve_booster1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.booster_slot_index == {1: sorted([api_booster1.id, api_booster3.id])}


def test_known_failures(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_booster1_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    eve_booster2_id = client.mk_eve_item(attrs={eve_slot_attr_id: 2})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_booster(type_id=eve_other_id)
    api_booster1 = api_fit.add_booster(type_id=eve_booster1_id)
    api_booster2 = api_fit.add_booster(type_id=eve_booster2_id)
    api_booster3 = api_fit.add_booster(type_id=eve_booster1_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=(True, [api_booster1.id])))
    assert api_val.passed is False
    assert api_val.details.booster_slot_index == {1: [api_booster3.id]}
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=(True, [api_booster3.id])))
    assert api_val.passed is False
    assert api_val.details.booster_slot_index == {1: [api_booster1.id]}
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=(True, [api_booster1.id, api_booster3.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        booster_slot_index=(True, [api_booster1.id, api_other.id, api_booster3.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_booster4 = api_fit.add_booster(type_id=eve_booster2_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=(True, [api_booster1.id, api_booster3.id])))
    assert api_val.passed is False
    assert api_val.details.booster_slot_index == {2: sorted([api_booster2.id, api_booster4.id])}


def test_rounding(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_booster1_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1.2})
    eve_booster2_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1.4})
    eve_booster3_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster1 = api_fit.add_booster(type_id=eve_booster1_id)
    api_booster2 = api_fit.add_booster(type_id=eve_booster2_id)
    api_fit.add_booster(type_id=eve_booster3_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.booster_slot_index == {1: sorted([api_booster1.id, api_booster2.id])}


def test_modified_index(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_booster_id = client.mk_eve_item(attrs={eve_slot_attr_id: 2})
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.post_assign,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_slot_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 3}, eff_ids=[eve_mod_effect_id])
    eve_char_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_booster1 = api_fit.add_booster(type_id=eve_booster_id)
    api_booster2 = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    assert api_booster1.update().attrs[eve_slot_attr_id].extra == approx(2)
    assert api_booster2.update().attrs[eve_slot_attr_id].extra == approx(2)
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.booster_slot_index == {2: sorted([api_booster1.id, api_booster2.id])}
    # Action
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification - attribute is modified, but not for purposes of validation
    assert api_booster1.update().attrs[eve_slot_attr_id].extra == approx(3)
    assert api_booster2.update().attrs[eve_slot_attr_id].extra == approx(3)
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.booster_slot_index == {2: sorted([api_booster1.id, api_booster2.id])}


def test_no_attr(client, consts):
    eve_slot_attr_id = consts.EveAttr.boosterness
    eve_booster_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster1 = api_fit.add_booster(type_id=eve_booster_id)
    api_booster2 = api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.booster_slot_index == {1: sorted([api_booster1.id, api_booster2.id])}


def test_no_value(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_booster_id = client.mk_eve_item()
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_slot_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_booster_id)
    api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_booster_id = client.alloc_item_id()
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_slot_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_booster_id)
    api_fit.add_booster(type_id=eve_booster_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_state(client, consts):
    # Disabled boosters still compete for slots
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_booster_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_booster1 = api_fit.add_booster(type_id=eve_booster_id, state=False)
    api_booster2 = api_fit.add_booster(type_id=eve_booster_id, state=False)
    # Verification
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=True))
    assert api_val.passed is False
    assert api_val.details.booster_slot_index == {1: sorted([api_booster1.id, api_booster2.id])}


def test_criterion_item_kind(client, consts):
    eve_slot_attr_id = client.mk_eve_attr(id_=consts.EveAttr.boosterness)
    eve_implant_id = client.mk_eve_item(attrs={eve_slot_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_implant(type_id=eve_implant_id)
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_val = api_fit.validate(options=ValOptions(booster_slot_index=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
