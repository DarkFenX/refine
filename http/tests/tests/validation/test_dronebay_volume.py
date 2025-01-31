from tests import approx, check_no_field


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(150)
    assert api_val.details.dronebay_volume.output == approx(125)
    assert api_val.details.dronebay_volume.users == {api_drone.id: approx(150)}


def test_fail_multiple_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(150)
    assert api_val.details.dronebay_volume.output == approx(125)
    assert api_val.details.dronebay_volume.users == {api_drone1.id: approx(50), api_drone2.id: approx(100)}


def test_fail_multiple_struct(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_struct_id = client.mk_eve_struct(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(150)
    assert api_val.details.dronebay_volume.output == approx(125)
    assert api_val.details.dronebay_volume.users == {api_drone1.id: approx(50), api_drone2.id: approx(100)}


def test_equal(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified_use(client, consts):
    # Drone volume is never modified, so the lib just uses unmodified attributes for faster access
    # to the attr value
    eve_skill_id = client.mk_eve_item()
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(75)
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(150)
    assert api_val.details.dronebay_volume.output == approx(125)
    assert api_val.details.dronebay_volume.users == {api_drone.id: approx(150)}
    # Action
    api_implant.remove()
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(150)
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(150)
    assert api_val.details.dronebay_volume.output == approx(125)
    assert api_val.details.dronebay_volume.users == {api_drone.id: approx(150)}


def test_modified_output(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_output_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 120})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    assert api_ship.update().attrs[eve_output_attr_id].extra == approx(120)
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(150)
    assert api_val.details.dronebay_volume.output == approx(120)
    assert api_val.details.dronebay_volume.users == {api_drone.id: approx(150)}
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_output_attr_id].extra == approx(180)
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_mutation_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_base_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 120})
    eve_mutated_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 130})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_drone_id], eve_mutated_drone_id)],
        attrs={eve_use_attr_id: (0.8, 1.2)})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_base_drone_id)
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(120)
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone.change_drone(mutation=(eve_mutator_id, {eve_use_attr_id: {consts.ApiAttrMutation.roll: 0.8}}))
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(145.6)
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(130)
    assert api_val.details.dronebay_volume.output == approx(125)
    assert api_val.details.dronebay_volume.users == {api_drone.id: approx(130)}
    # Action
    api_drone.change_drone(mutation={eve_use_attr_id: None})
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(130)
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(130)
    assert api_val.details.dronebay_volume.output == approx(125)
    assert api_val.details.dronebay_volume.users == {api_drone.id: approx(130)}


def test_rounding(client, consts):
    # Volume/capacity shouldn't have its sum or individual values rounded
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.002})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 5.227})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 5.223})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(5.229)
    assert api_val.details.dronebay_volume.output == approx(5.223)
    assert api_val.details.dronebay_volume.users == {api_drone1.id: approx(0.002), api_drone2.id: approx(5.227)}


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(5)
    assert api_val.details.dronebay_volume.output is None
    assert api_val.details.dronebay_volume.users == {api_drone.id: approx(5)}


def test_not_loaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 5})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(5)
    assert api_val.details.dronebay_volume.output is None
    assert api_val.details.dronebay_volume.users == {api_drone.id: approx(5)}


def test_not_loaded_user(client, consts):
    # Just check that nothing crashes, not loaded items are not supposed to even be registered
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    # Create an item which has the attribute, just to prevent the attribute from being cleaned up
    client.mk_eve_item(attrs={eve_use_attr_id: 5})
    eve_drone_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_non_positive(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_drone3_id = client.mk_eve_item(attrs={eve_use_attr_id: -10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone1_id)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id)
    api_fit.add_drone(type_id=eve_drone3_id)
    # Verification - items with negative and 0 use are not exposed
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(140)
    assert api_val.details.dronebay_volume.output == approx(125)
    assert api_val.details.dronebay_volume.users == {api_drone2.id: approx(150)}


def test_no_value_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_drone2_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id)
    api_fit.add_drone(type_id=eve_drone2_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(150)
    assert api_val.details.dronebay_volume.output == approx(125)
    assert api_val.details.dronebay_volume.users == {api_drone1.id: approx(150)}


def test_no_value_output(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship()
    # Make an item to ensure that output attribute is not cleaned up
    client.mk_eve_item(attrs={eve_output_attr_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(150)
    assert api_val.details.dronebay_volume.output == approx(0)
    assert api_val.details.dronebay_volume.users == {api_drone.id: approx(150)}


def test_no_attr_use(client, consts):
    # Invalid situation which shouldn't happen; just check that nothing crashes, behavior is
    # irrelevant
    eve_use_attr_id = consts.EveAttr.volume
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(150)
    assert api_val.details.dronebay_volume.output == approx(125)
    assert api_val.details.dronebay_volume.users == {api_drone.id: approx(150)}


def test_no_attr_output(client, consts):
    # Invalid situation which shouldn't happen; just check that nothing crashes, behavior is
    # irrelevant
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = consts.EveAttr.drone_capacity
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(150)
    assert api_val.details.dronebay_volume.output is None
    assert api_val.details.dronebay_volume.users == {api_drone.id: approx(150)}


def test_criterion_state(client, consts):
    # Drones take dronebay volume in any state, even ghost
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiState.offline)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(150)
    assert api_val.details.dronebay_volume.output == approx(125)
    assert api_val.details.dronebay_volume.users == {api_drone.id: approx(150)}
    # Action
    api_drone.change_drone(state=consts.ApiState.ghost)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is False
    assert api_val.details.dronebay_volume.used == approx(150)
    assert api_val.details.dronebay_volume.output == approx(125)
    assert api_val.details.dronebay_volume.users == {api_drone.id: approx(150)}


def test_criterion_item_type(client, consts):
    # Validation applies only to drones
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_output_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_capacity)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_output_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_fighter(type_id=eve_drone_id)
    # Verification
    api_val = api_fit.validate(include=[consts.ApiValType.dronebay_volume])
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
