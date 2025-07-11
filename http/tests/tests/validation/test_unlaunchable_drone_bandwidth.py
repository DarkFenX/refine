from tests import Muta, approx, check_no_field
from tests.fw.api import ValOptions


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 25})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max == approx(25)
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone.id: 50}


def test_fail_multiple_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 25})
    eve_drone3_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_bay)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_bay)
    api_drone3 = api_fit.add_drone(type_id=eve_drone3_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max == approx(20)
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone2.id: 25, api_drone3.id: 50}


def test_fail_multiple_struct(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 25})
    eve_drone3_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_struct_id = client.mk_eve_struct(attrs={eve_max_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_bay)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_bay)
    api_drone3 = api_fit.add_drone(type_id=eve_drone3_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max == approx(20)
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone2.id: 25, api_drone3.id: 50}


def test_eqial_below_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 25})
    eve_drone3_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 50})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_bay)
    api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_bay)
    api_fit.add_drone(type_id=eve_drone3_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 25})
    eve_drone3_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 20})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_bay)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=(True, [api_drone2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone3 = api_fit.add_drone(type_id=eve_drone3_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=(True, [api_drone2.id])))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max == approx(20)
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone3.id: 50}
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=(True, [api_drone3.id])))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max == approx(20)
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone2.id: 25}
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=(True, [api_drone2.id, api_drone3.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        unlaunchable_drone_bandwidth=(True, [api_drone2.id, api_other.id, api_drone3.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified_use(client, consts):
    # Drone bandwidth use is never modified, so the lib just uses unmodified attributes for faster
    # access to the attr value
    eve_skill_id = client.mk_eve_item()
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.own_srq,
        loc=consts.EveModLoc.char,
        srq=eve_skill_id,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 40}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 25})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(20)
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max == approx(25)
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone.id: 40}
    # Action
    api_implant.remove()
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(40)
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max == approx(25)
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone.id: 40}


def test_modified_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 40})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].extra == approx(40)
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max == approx(40)
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone.id: 50}
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].extra == approx(60)
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_mutation_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_base_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 25})
    eve_mutated_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_drone_id], eve_mutated_drone_id)],
        attrs={eve_use_attr_id: (0.8, 1.2)})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 25})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_base_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(25)
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone.change_drone(mutation=(eve_mutator_id, {eve_use_attr_id: Muta.roll_to_api(val=0.8)}))
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(56)
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max == approx(25)
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone.id: approx(50)}
    # Action
    api_drone.change_drone(mutation={eve_use_attr_id: None})
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].extra == approx(50)
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max == approx(25)
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone.id: approx(50)}


def test_rounding(client, consts):
    # Bandwidth shouldn't have its sum or individual values rounded
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.002})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 5.227})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 5.226})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_bay)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max == approx(5.226)
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone2.id: 5.227}


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max is None
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone.id: 5}


def test_not_loaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max is None
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone.id: 5}


def test_not_loaded_user(client, consts):
    # Just check that nothing crashes, not loaded items are not supposed to even be registered
    client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    eve_drone_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_non_positive(client, consts):
    # Invalid situation which shouldn't happen; just check that nothing crashes, behavior is
    # irrelevant
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    eve_drone3_id = client.mk_eve_item(attrs={eve_use_attr_id: -10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: -15})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_bay)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_bay)
    api_drone3 = api_fit.add_drone(type_id=eve_drone3_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max == approx(-15)
    assert api_val.details.unlaunchable_drone_bandwidth.users == {
        api_drone1.id: 0, api_drone2.id: 5, api_drone3.id: -10}


def test_no_value_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item()
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 25})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max == approx(25)
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone2.id: 50}


def test_no_value_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.unlaunchable_drone_bandwidth.max == approx(0)
    assert api_val.details.unlaunchable_drone_bandwidth.users == {api_drone.id: 150}


def test_criterion_item_kind(client, consts):
    # Validation applies only to drones
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_item_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125, eve_use_attr_id: 150})
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(
        attrs={eve_autocharge_attr_id: eve_item_id, eve_use_attr_id: 150},
        eff_ids=[eve_autocharge_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_item_id)
    api_fit.set_character(type_id=eve_item_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_item_id)
    api_fit.add_implant(type_id=eve_item_id)
    api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.overload, charge_type_id=eve_item_id)
    api_fit.add_rig(type_id=eve_item_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    api_fit.set_stance(type_id=eve_item_id)
    api_fit.add_subsystem(type_id=eve_item_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_val = api_fit.validate(options=ValOptions(unlaunchable_drone_bandwidth=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
