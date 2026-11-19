from fw import Muta, approx, check_no_field
from fw.api import FitStatsOptions, ValOptions


def test_fail_single(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(150)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: 150}


def test_fail_multiple_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(150)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone1.id: 50, api_drone2.id: 100}


def test_fail_multiple_struct(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_struct_id = client.mk_eve_struct(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(150)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone1.id: 50, api_drone2.id: 100}


def test_equal(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(150), approx(150))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]


def test_decreasers(client, consts):
    # Potential bandwidth is checked with modules which decrease it switched off, e.g. warp core
    # stabilizers and interdiction nullifiers in EVE
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_online_effect_id = client.mk_eve_online_effect()
    eve_mod_online_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_mod_active_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, mod_info=[eve_mod])
    eve_online_penalty_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: -50},
        eff_ids=[eve_mod_online_effect_id, eve_online_effect_id])
    eve_active_penalty_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: -50},
        eff_ids=[eve_mod_active_effect_id],
        defeff_id=eve_mod_active_effect_id)
    eve_bonus_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: 50},
        eff_ids=[eve_mod_online_effect_id, eve_online_effect_id])
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 60})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_online_module = api_fit.add_module(type_id=eve_online_penalty_module_id, state=consts.ApiModuleState.online)
    api_active_module = api_fit.add_module(type_id=eve_active_penalty_module_id, state=consts.ApiModuleState.active)
    api_overload_module = api_fit.add_module(type_id=eve_active_penalty_module_id, state=consts.ApiModuleState.overload)
    api_bonus_module = api_fit.add_module(type_id=eve_bonus_module_id, state=consts.ApiModuleState.online)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    # Verification - bonus module stays online, so potential bandwidth is 150
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(100), approx(18.75))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]
    # Action
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(160), approx(18.75))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(160)
    assert api_val.details.drone_bandwidth_potential.max == approx(150)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone1.id: 100, api_drone2.id: 60}
    # Verification - module states are restored after validation
    assert api_online_module.update().state == consts.ApiModuleState.online
    assert api_active_module.update().state == consts.ApiModuleState.active
    assert api_overload_module.update().state == consts.ApiModuleState.overload
    assert api_bonus_module.update().state == consts.ApiModuleState.online
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(160), approx(18.75))


def test_decreasers_no_gain(client, consts):
    # When switching off a module which decreases bandwidth does not give the ship more of it,
    # current bandwidth is the potential one
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_penalty_attr_id = client.mk_eve_attr()
    eve_bonus_attr_id = client.mk_eve_attr()
    eve_penalty_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_penalty_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_bonus_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_bonus_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_online_effect_id = client.mk_eve_online_effect()
    eve_mod_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_penalty_mod, eve_bonus_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_penalty_attr_id: -50, eve_bonus_attr_id: 300},
        eff_ids=[eve_mod_effect_id, eve_online_effect_id])
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 250})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(250), approx(200))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(250)
    assert api_val.details.drone_bandwidth_potential.max == approx(200)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: 250}
    assert api_module.update().state == consts.ApiModuleState.online


def test_current_validation(client, consts):
    # In the current state potential bandwidth is the current one, so the lib skips this validation
    # there when the current bandwidth one passes
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_online_effect_id = client.mk_eve_online_effect()
    eve_mod_online_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: -50},
        eff_ids=[eve_mod_online_effect_id, eve_online_effect_id])
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 60})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.offline)
    api_drone1 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_current=True, drone_bandwidth_potential=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_current=True, drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_current.used == approx(60)
    assert api_val.details.drone_bandwidth_current.max == approx(50)
    assert api_val.details.drone_bandwidth_current.users == {api_drone1.id: 60}
    with check_no_field():
        api_val.details.drone_bandwidth_potential  # ruff:ignore[useless-expression]
    # Action
    api_drone2 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_current=True, drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_current.used == approx(120)
    assert api_val.details.drone_bandwidth_current.max == approx(50)
    assert api_val.details.drone_bandwidth_current.users == {api_drone1.id: 60, api_drone2.id: 60}
    assert api_val.details.drone_bandwidth_potential.used == approx(120)
    assert api_val.details.drone_bandwidth_potential.max == approx(100)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone1.id: 60, api_drone2.id: 60}
    assert api_module.update().state == consts.ApiModuleState.online


def test_current_validation_known_failures(client, consts):
    # Current bandwidth validation which passes only due to its known failures says nothing about
    # drones fitting, so this validation is not skipped
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 60})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    api_drone2 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_val = api_fit.validate(options=ValOptions(
        drone_bandwidth_current=[api_drone1.id, api_drone2.id],
        drone_bandwidth_potential=True))
    assert api_val.passed is False
    with check_no_field():
        api_val.details.drone_bandwidth_current  # ruff:ignore[useless-expression]
    assert api_val.details.drone_bandwidth_potential.used == approx(120)
    assert api_val.details.drone_bandwidth_potential.max == approx(100)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone1.id: 60, api_drone2.id: 60}
    api_val = api_fit.validate(options=ValOptions(
        drone_bandwidth_current=[api_drone1.id, api_drone2.id],
        drone_bandwidth_potential=[api_drone1.id, api_drone2.id]))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]


def test_unlaunchable_validation(client, consts):
    # Validations against potential bandwidth are checked together: when one of them fails in the
    # current state, both are checked with modules which decrease bandwidth switched off
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_online_effect_id = client.mk_eve_online_effect()
    eve_mod_online_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.online, mod_info=[eve_mod])
    eve_module_id = client.mk_eve_item(
        attrs={eve_mod_attr_id: -50},
        eff_ids=[eve_mod_online_effect_id, eve_online_effect_id])
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 40})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 80})
    eve_drone3_id = client.mk_eve_item(attrs={eve_use_attr_id: 70})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.online)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_bay)
    # Verification - launched drone fits into current bandwidth, the one in bay does not
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(40), approx(50))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True, unlaunchable_drone_bandwidth=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]
    # Action
    api_drone3 = api_fit.add_drone(type_id=eve_drone3_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(110), approx(50))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True, unlaunchable_drone_bandwidth=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(110)
    assert api_val.details.drone_bandwidth_potential.max == approx(100)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone1.id: 40, api_drone3.id: 70}
    with check_no_field():
        api_val.details.unlaunchable_drone_bandwidth  # ruff:ignore[useless-expression]
    assert api_module.update().state == consts.ApiModuleState.online


def test_known_failures(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_drone3_id = client.mk_eve_item(attrs={eve_use_attr_id: -10})
    eve_drone4_id = client.mk_eve_item(attrs={eve_use_attr_id: 0})
    eve_drone5_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.5})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=[api_drone1.id]))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]
    # Action
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(250), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=[api_drone1.id]))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(250)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone2.id: 100}
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=[api_drone2.id]))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(250)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone1.id: 150}
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=[api_drone1.id, api_drone2.id]))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]
    api_val = api_fit.validate(options=ValOptions(
        drone_bandwidth_potential=[api_drone1.id, api_other.id, api_drone2.id]))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]
    # Action
    api_drone3 = api_fit.add_drone(type_id=eve_drone3_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(240), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=[api_drone1.id, api_drone2.id]))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]
    # Action
    api_drone3.remove()
    api_drone4 = api_fit.add_drone(type_id=eve_drone4_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(250), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=[api_drone1.id, api_drone2.id]))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]
    # Action
    api_drone4.remove()
    api_drone5 = api_fit.add_drone(type_id=eve_drone5_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(250.5), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=[api_drone1.id, api_drone2.id]))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == 250.5
    assert api_val.details.drone_bandwidth_potential.max == 125
    assert api_val.details.drone_bandwidth_potential.users == {api_drone5.id: 0.5}


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
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150}, srqs={eve_skill_id: 1})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: -50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].modified == approx(75)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(150)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: 150}
    # Action
    api_implant.remove()
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].modified == approx(150)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(150)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: 150}


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
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 120})
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].modified == approx(120)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(150), approx(120))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(150)
    assert api_val.details.drone_bandwidth_potential.max == approx(120)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: 150}
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].modified == approx(180)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(150), approx(180))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]


def test_mutation_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_base_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 130})
    eve_mutated_drone_id = client.mk_eve_item(is_dynamic=True, attrs={eve_use_attr_id: 120})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_drone_id], eve_mutated_drone_id)],
        attrs={eve_use_attr_id: (0.8, 1.2)})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_base_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].modified == approx(130)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(130), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(130)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: approx(130)}
    # Action
    api_drone.change_drone(mutation=(eve_mutator_id, {eve_use_attr_id: Muta.roll_to_api(val=0.8)}))
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].modified == approx(145.6)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(130), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(130)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: approx(130)}
    # Action
    api_drone.change_drone(mutation={eve_use_attr_id: None})
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].modified == approx(130)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(130), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(130)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: approx(130)}


def test_mutation_use_base_change(client, consts):
    # Bandwidth use is taken from base item, so changing base type can change it even when mutated
    # item stays the same
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_base_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 130})
    eve_base_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 100})
    eve_mutated_drone_id = client.mk_eve_item(is_dynamic=True, attrs={eve_use_attr_id: 120})
    eve_mutator_id = client.mk_eve_mutator(
        items=[([eve_base_drone1_id, eve_base_drone2_id], eve_mutated_drone_id)],
        attrs={eve_use_attr_id: (0.8, 1.2)})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(
        type_id=eve_base_drone1_id,
        state=consts.ApiMinionState.in_space,
        mutation=(eve_mutator_id, {eve_use_attr_id: Muta.roll_to_api(val=0.8)}))
    # Verification
    assert api_drone.update().attrs[eve_use_attr_id].modified == approx(145.6)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(130), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(130)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: approx(130)}
    # Action
    api_drone.change_drone(type_id=eve_base_drone2_id)
    # Verification
    api_drone.update()
    assert api_drone.type_id == eve_mutated_drone_id
    assert api_drone.attrs[eve_use_attr_id].modified == approx(112)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(100), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]
    # Action
    api_drone.change_drone(type_id=eve_base_drone1_id)
    # Verification
    api_drone.update()
    assert api_drone.type_id == eve_mutated_drone_id
    assert api_drone.attrs[eve_use_attr_id].modified == approx(145.6)
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(130), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(130)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: approx(130)}


def test_switch_type_id(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 140})
    eve_drone3_id = client.mk_eve_item(attrs={eve_use_attr_id: 120})
    eve_drone4_id = client.alloc_item_id()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(150)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: 150}
    # Action
    api_drone.change_drone(type_id=eve_drone2_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(140), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(140)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: 140}
    # Action
    api_drone.change_drone(type_id=eve_drone3_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(120), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]
    # Action
    api_drone.change_drone(type_id=eve_drone1_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(150)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: 150}
    # Action
    api_drone.change_drone(type_id=eve_drone4_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(0), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]
    # Action
    api_drone.change_drone(type_id=eve_drone2_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(140), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(140)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: 140}


def test_rounding(client, consts):
    # Bandwidth shouldn't have its sum or individual values rounded
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0.002})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 5.227})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 5.223})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(5.229), approx(5.223))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(5.229)
    assert api_val.details.drone_bandwidth_potential.max == approx(5.223)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone1.id: 0.002, api_drone2.id: 5.227}


def test_no_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(5), None)
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(5)
    assert api_val.details.drone_bandwidth_potential.max is None
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: 5}


def test_no_attr_use(client, consts):
    eve_use_attr_id = consts.EveAttr.drone_bandwidth_used
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification - users are assumed to take no resource when attribute does not exist
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]


def test_no_attr_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = consts.EveAttr.drone_bandwidth
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 50})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification - when output attr does not exist, it is assumed to be 0
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(50)
    assert api_val.details.drone_bandwidth_potential.max == approx(0)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: 50}


def test_not_loaded_ship(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 5})
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(5), None)
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(5)
    assert api_val.details.drone_bandwidth_potential.max is None
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: 5}


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
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(0), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]


def test_non_positive(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 0})
    eve_drone2_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_drone3_id = client.mk_eve_item(attrs={eve_use_attr_id: -10})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_space)
    api_fit.add_drone(type_id=eve_drone3_id, state=consts.ApiMinionState.in_space)
    # Verification - items with negative and 0 use are not exposed
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(140), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(140)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone2.id: 150}


def test_no_value_use(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone1_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_drone2_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(150)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone1.id: 150}


def test_no_value_max(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(150), approx(0))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(150)
    assert api_val.details.drone_bandwidth_potential.max == approx(0)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: 150}


def test_criterion_item_kind(client, consts):
    # Validation applies only to drones
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_item_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125, eve_use_attr_id: 150})
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
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
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(0), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]


def test_criterion_drone_state(client, consts):
    eve_use_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth_used)
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_bandwidth)
    eve_drone_id = client.mk_eve_item(attrs={eve_use_attr_id: 150})
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 125})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(0), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(150), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is False
    assert api_val.details.drone_bandwidth_potential.used == approx(150)
    assert api_val.details.drone_bandwidth_potential.max == approx(125)
    assert api_val.details.drone_bandwidth_potential.users == {api_drone.id: 150}
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.in_bay)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(drone_bandwidth=True))
    assert api_stats.drone_bandwidth.one() == (approx(0), approx(125))
    api_val = api_fit.validate(options=ValOptions(drone_bandwidth_potential=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # ruff:ignore[useless-expression]
