from tests import approx, check_no_field


def test_fail_single(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_subsystems)
    eve_subsystem_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_count.used == 1
    assert api_val.details.subsystem_slot_count.total == 0
    assert api_val.details.subsystem_slot_count.users == [api_subsystem.id]


def test_fail_multiple_ship(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_subsystems)
    eve_subsystem_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem_id)
    api_subsystem2 = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_count.used == 2
    assert api_val.details.subsystem_slot_count.total == 1
    assert api_val.details.subsystem_slot_count.users == sorted([api_subsystem1.id, api_subsystem2.id])


def test_fail_multiple_struct(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_subsystems)
    eve_subsystem_id = client.mk_eve_item()
    eve_struct_id = client.mk_eve_struct(attrs={eve_total_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_subsystem1 = api_fit.add_subsystem(type_id=eve_subsystem_id)
    api_subsystem2 = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_count.used == 2
    assert api_val.details.subsystem_slot_count.total == 1
    assert api_val.details.subsystem_slot_count.users == sorted([api_subsystem1.id, api_subsystem2.id])


def test_equal(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_subsystems)
    eve_subsystem_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified_total(client, consts):
    # Unrealistic scenario, but modification of total count is supported
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_subsystems)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_total_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_subsystem_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    assert api_ship.update().attrs[eve_total_attr_id].extra == approx(0)
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_count.used == 1
    assert api_val.details.subsystem_slot_count.total == 0
    assert api_val.details.subsystem_slot_count.users == [api_subsystem.id]
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_total_attr_id].extra == approx(1)
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_fractional_total(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_subsystems)
    eve_subsystem_id = client.mk_eve_item()
    eve_ship1_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0.4})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_count.used == 1
    assert api_val.details.subsystem_slot_count.total == 0
    assert api_val.details.subsystem_slot_count.users == [api_subsystem.id]
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_ship(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_subsystems)
    eve_subsystem_id = client.mk_eve_item()
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_total_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_subsystem = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_count.used == 1
    assert api_val.details.subsystem_slot_count.total is None
    assert api_val.details.subsystem_slot_count.users == [api_subsystem.id]


def test_not_loaded_user(client, consts):
    # Not loaded subsystems still take slot
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_subsystems)
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    eve_subsystem_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_count.used == 1
    assert api_val.details.subsystem_slot_count.total == 0
    assert api_val.details.subsystem_slot_count.users == [api_subsystem.id]


def test_not_loaded_ship(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_subsystems)
    eve_subsystem_id = client.mk_eve_item()
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_total_attr_id: 5})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_count.used == 1
    assert api_val.details.subsystem_slot_count.total is None
    assert api_val.details.subsystem_slot_count.users == [api_subsystem.id]


def test_no_value_total(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_subsystems)
    eve_subsystem_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    # Make an item to ensure that total attribute is not cleaned up
    client.mk_eve_item(attrs={eve_total_attr_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_count.used == 1
    assert api_val.details.subsystem_slot_count.total == 0
    assert api_val.details.subsystem_slot_count.users == [api_subsystem.id]


def test_no_attr_total(client, consts):
    # Invalid situation which shouldn't happen; just check that nothing crashes, behavior is
    # irrelevant
    eve_total_attr_id = consts.EveAttr.max_subsystems
    eve_subsystem_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_count.used == 1
    assert api_val.details.subsystem_slot_count.total is None
    assert api_val.details.subsystem_slot_count.users == [api_subsystem.id]


def test_criterion_state(client, consts):
    # Slot is taken even when subsystem is disabled
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_subsystems)
    eve_subsystem_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_subsystem = api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_count.used == 1
    assert api_val.details.subsystem_slot_count.total == 0
    assert api_val.details.subsystem_slot_count.users == [api_subsystem.id]
    # Action
    api_subsystem.change_subsystem(state=False)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_count.used == 1
    assert api_val.details.subsystem_slot_count.total == 0
    assert api_val.details.subsystem_slot_count.users == [api_subsystem.id]


def test_criterion_item_kind(client, consts):
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_subsystems)
    eve_rig_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_total_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_t3c_slot_override(client, consts):
    # T3Cs have 5 subsystem slots in EVE data, check that it's changed to 4
    eve_total_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_subsystems)
    eve_subsystem_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(grp_id=consts.EveItemGrp.strategic_cruiser, attrs={eve_total_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    for _ in range(5):
        api_fit.add_subsystem(type_id=eve_subsystem_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.subsystem_slot_count])
    assert api_val.passed is False
    assert api_val.details.subsystem_slot_count.used == 5
    assert api_val.details.subsystem_slot_count.total == 4
    assert len(api_val.details.subsystem_slot_count.users) == 5
