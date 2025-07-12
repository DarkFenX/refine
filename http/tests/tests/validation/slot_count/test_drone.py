from tests import approx, check_no_field
from tests.fw.api import FitStatsOptions, ValOptions


def test_fail_single(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (1, 0)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is False
    assert api_val.details.launched_drone_count.used == 1
    assert api_val.details.launched_drone_count.max == 0
    assert api_val.details.launched_drone_count.users == [api_drone.id]


def test_fail_multiple(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    api_drone2 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (2, 1)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is False
    assert api_val.details.launched_drone_count.used == 2
    assert api_val.details.launched_drone_count.max == 1
    assert api_val.details.launched_drone_count.users == sorted([api_drone1.id, api_drone2.id])


def test_equal(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (1, 1)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 1})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.set_character(type_id=eve_char_id)
    api_drone1 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    api_drone2 = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (2, 1)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=(True, [api_drone1.id])))
    assert api_val.passed is False
    assert api_val.details.launched_drone_count.used == 2
    assert api_val.details.launched_drone_count.max == 1
    assert api_val.details.launched_drone_count.users == [api_drone2.id]
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=(True, [api_drone2.id])))
    assert api_val.passed is False
    assert api_val.details.launched_drone_count.used == 2
    assert api_val.details.launched_drone_count.max == 1
    assert api_val.details.launched_drone_count.users == [api_drone1.id]
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=(True, [api_drone1.id, api_drone2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        launched_drone_count=(True, [api_drone1.id, api_other.id, api_drone2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified_max(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_drone_id = client.mk_eve_item()
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_char = api_fit.set_character(type_id=eve_char_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    assert api_char.update().attrs[eve_max_attr_id].extra == approx(0)
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (1, 0)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is False
    assert api_val.details.launched_drone_count.used == 1
    assert api_val.details.launched_drone_count.max == 0
    assert api_val.details.launched_drone_count.users == [api_drone.id]
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_char.update().attrs[eve_max_attr_id].extra == approx(1)
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (1, 1)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_fractional_max(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    eve_char1_id = client.mk_eve_item(attrs={eve_max_attr_id: 0.4})
    eve_char2_id = client.mk_eve_item(attrs={eve_max_attr_id: 0.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char1_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (1, 0)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is False
    assert api_val.details.launched_drone_count.used == 1
    assert api_val.details.launched_drone_count.max == 0
    assert api_val.details.launched_drone_count.users == [api_drone.id]
    # Action
    api_fit.set_character(type_id=eve_char2_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (1, 1)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_char(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (1, None)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is False
    assert api_val.details.launched_drone_count.used == 1
    assert api_val.details.launched_drone_count.max is None
    assert api_val.details.launched_drone_count.users == [api_drone.id]


def test_not_loaded_user(client, consts):
    # Not loaded drones still take slot
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 0})
    eve_drone1_id = client.alloc_item_id()
    eve_drone2_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_drone = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (1, 0)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is False
    assert api_val.details.launched_drone_count.used == 1
    assert api_val.details.launched_drone_count.max == 0
    assert api_val.details.launched_drone_count.users == [api_drone.id]
    # Action
    api_drone.change_drone(type_id=eve_drone2_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (1, 0)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is False
    assert api_val.details.launched_drone_count.used == 1
    assert api_val.details.launched_drone_count.max == 0
    assert api_val.details.launched_drone_count.users == [api_drone.id]
    # Action
    api_drone.change_drone(type_id=eve_drone1_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (1, 0)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is False
    assert api_val.details.launched_drone_count.used == 1
    assert api_val.details.launched_drone_count.max == 0
    assert api_val.details.launched_drone_count.users == [api_drone.id]


def test_not_loaded_char(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    eve_char_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (1, None)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is False
    assert api_val.details.launched_drone_count.used == 1
    assert api_val.details.launched_drone_count.max is None
    assert api_val.details.launched_drone_count.users == [api_drone.id]


def test_no_value_max(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    eve_char_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (1, 0)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is False
    assert api_val.details.launched_drone_count.used == 1
    assert api_val.details.launched_drone_count.max == 0
    assert api_val.details.launched_drone_count.users == [api_drone.id]


def test_criterion_drone_state(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_drone_id = client.mk_eve_item()
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.in_bay)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (0, 0)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (1, 0)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is False
    assert api_val.details.launched_drone_count.used == 1
    assert api_val.details.launched_drone_count.max == 0
    assert api_val.details.launched_drone_count.users == [api_drone.id]
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.in_bay)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (0, 0)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_kind(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_active_drones)
    eve_item_id = client.mk_eve_item()
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(attrs={eve_autocharge_attr_id: eve_item_id}, eff_ids=[eve_autocharge_effect_id])
    eve_char_id = client.mk_eve_item(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_item_id)
    api_fit.add_implant(type_id=eve_item_id)
    api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.overload, charge_type_id=eve_item_id)
    api_fit.add_rig(type_id=eve_item_id)
    api_fit.add_service(type_id=eve_item_id, state=consts.ApiServiceState.online)
    api_fit.set_ship(type_id=eve_item_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    api_fit.set_stance(type_id=eve_item_id)
    api_fit.add_subsystem(type_id=eve_item_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_stats = api_fit.get_stats(options=FitStatsOptions(launched_drones=True))
    assert api_stats.launched_drones == (0, 0)
    api_val = api_fit.validate(options=ValOptions(launched_drone_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
