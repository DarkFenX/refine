from tests import approx, check_no_field


def test_fail_single(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_fighter_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is False
    assert api_val.details.launched_support_fighters.used == 1
    assert api_val.details.launched_support_fighters.total == 0
    assert api_val.details.launched_support_fighters.users == [api_fighter.id]


def test_fail_multiple_ship(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_fighter_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is False
    assert api_val.details.launched_support_fighters.used == 2
    assert api_val.details.launched_support_fighters.total == 1
    assert api_val.details.launched_support_fighters.users == sorted([api_fighter1.id, api_fighter2.id])


def test_fail_multiple_struct(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_fighter_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 1})
    eve_struct_id = client.mk_eve_struct(attrs={eve_total_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is False
    assert api_val.details.launched_support_fighters.used == 2
    assert api_val.details.launched_support_fighters.total == 1
    assert api_val.details.launched_support_fighters.users == sorted([api_fighter1.id, api_fighter2.id])


def test_equal(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_fighter_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified_fighter_type(client, consts):
    # Fighter type is never modified, so the lib just uses unmodified attributes for access to the
    # attr value
    eve_skill_id = client.mk_eve_item()
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_ftr_type_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_fighter_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 1}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -1}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    # Verification
    assert api_fighter.update().attrs[eve_ftr_type_attr_id].extra == approx(0)
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is False
    assert api_val.details.launched_support_fighters.used == 1
    assert api_val.details.launched_support_fighters.total == 0
    assert api_val.details.launched_support_fighters.users == [api_fighter.id]
    # Action
    api_implant.remove()
    # Verification
    assert api_fighter.update().attrs[eve_ftr_type_attr_id].extra == approx(1)
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is False
    assert api_val.details.launched_support_fighters.used == 1
    assert api_val.details.launched_support_fighters.total == 0
    assert api_val.details.launched_support_fighters.users == [api_fighter.id]


def test_modified_total(client, consts):
    # Unrealistic scenario, but modification of total count is supported
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_total_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_fighter_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    # Verification
    assert api_ship.update().attrs[eve_total_attr_id].extra == approx(0)
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is False
    assert api_val.details.launched_support_fighters.used == 1
    assert api_val.details.launched_support_fighters.total == 0
    assert api_val.details.launched_support_fighters.users == [api_fighter.id]
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_total_attr_id].extra == approx(1)
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_fractional_fighter_type(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_fighter1_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 0.4})
    eve_fighter2_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 0.6})
    eve_fighter3_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: -0.1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter1 = api_fit.add_fighter(type_id=eve_fighter1_id, state=consts.ApiMinionState.in_space)
    api_fighter2 = api_fit.add_fighter(type_id=eve_fighter2_id, state=consts.ApiMinionState.in_space)
    api_fighter3 = api_fit.add_fighter(type_id=eve_fighter3_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is False
    assert api_val.details.launched_support_fighters.used == 3
    assert api_val.details.launched_support_fighters.total == 2
    assert api_val.details.launched_support_fighters.users == sorted([
        api_fighter1.id, api_fighter2.id, api_fighter3.id])


def test_fractional_total(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_fighter_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 1})
    eve_ship1_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0.4})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is False
    assert api_val.details.launched_support_fighters.used == 1
    assert api_val.details.launched_support_fighters.total == 0
    assert api_val.details.launched_support_fighters.users == [api_fighter.id]
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification - value is rounded up to int
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_ship(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_fighter_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 1})
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_total_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is False
    assert api_val.details.launched_support_fighters.used == 1
    assert api_val.details.launched_support_fighters.total is None
    assert api_val.details.launched_support_fighters.users == [api_fighter.id]


def test_not_loaded_user(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    eve_fighter_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_not_loaded_ship(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_fighter_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 1})
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_total_attr_id: 5})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is False
    assert api_val.details.launched_support_fighters.used == 1
    assert api_val.details.launched_support_fighters.total is None
    assert api_val.details.launched_support_fighters.users == [api_fighter.id]


def test_no_value_total(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_fighter_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship()
    # Make an item to ensure that total attribute is not cleaned up
    client.mk_eve_item(attrs={eve_total_attr_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is False
    assert api_val.details.launched_support_fighters.used == 1
    assert api_val.details.launched_support_fighters.total == 0
    assert api_val.details.launched_support_fighters.users == [api_fighter.id]


def test_no_attr_fighter_type(client, consts):
    # Invalid situation which shouldn't happen; just check that nothing crashes, behavior is
    # irrelevant
    eve_ftr_type_attr_id = consts.EveAttr.ftr_sq_is_support
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_fighter_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is False
    assert api_val.details.launched_support_fighters.used == 1
    assert api_val.details.launched_support_fighters.total == 0
    assert api_val.details.launched_support_fighters.users == [api_fighter.id]


def test_no_attr_total(client, consts):
    # Invalid situation which shouldn't happen; just check that nothing crashes, behavior is
    # irrelevant
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_total_attr_id = consts.EveAttr.ftr_support_slots
    eve_fighter_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is False
    assert api_val.details.launched_support_fighters.used == 1
    assert api_val.details.launched_support_fighters.total is None
    assert api_val.details.launched_support_fighters.users == [api_fighter.id]


def test_criterion_state(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_fighter_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is False
    assert api_val.details.launched_support_fighters.used == 1
    assert api_val.details.launched_support_fighters.total == 0
    assert api_val.details.launched_support_fighters.users == [api_fighter.id]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_fighter_type(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_other_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_light)
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_fighter1_id = client.mk_eve_item()
    eve_fighter2_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 0})
    eve_fighter3_id = client.mk_eve_item(attrs={eve_other_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_fighter1_id, state=consts.ApiMinionState.in_space)
    api_fit.add_fighter(type_id=eve_fighter2_id, state=consts.ApiMinionState.in_space)
    api_fit.add_fighter(type_id=eve_fighter3_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_kind(client, consts):
    eve_ftr_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_is_support)
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_support_slots)
    eve_drone_id = client.mk_eve_item(attrs={eve_ftr_type_attr_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.launched_support_fighters])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
