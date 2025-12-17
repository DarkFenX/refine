from fw import approx, check_no_field
from fw.api import FitStatsOptions, ValOptions


def test_fail_single(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, rack=consts.ApiRack.mid, state=consts.ApiModuleState.offline)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (1, 0)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 1
    assert api_val.details.mid_slot_count.max == 0
    assert api_val.details.mid_slot_count.users == [api_module.id]


def test_fail_multiple_ship(client, consts):
    # Unlike other validations, here we expose only users which are outside of max count of slots
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, rack=consts.ApiRack.mid, state=consts.ApiModuleState.offline)
    api_module2 = api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.ApiRack.mid,
        state=consts.ApiModuleState.offline)
    api_module3 = api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.ApiRack.mid,
        state=consts.ApiModuleState.offline)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (3, 1)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 3
    assert api_val.details.mid_slot_count.max == 1
    assert api_val.details.mid_slot_count.users == sorted([api_module2.id, api_module3.id])


def test_fail_multiple_struct(client, consts):
    # Unlike other validations, here we expose only users which are outside of max count of slots
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_module_id = client.mk_eve_item()
    eve_struct_id = client.mk_eve_struct(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_fit.add_module(type_id=eve_module_id, rack=consts.ApiRack.mid, state=consts.ApiModuleState.offline)
    api_module2 = api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.ApiRack.mid,
        state=consts.ApiModuleState.offline)
    api_module3 = api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.ApiRack.mid,
        state=consts.ApiModuleState.offline)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (3, 1)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 3
    assert api_val.details.mid_slot_count.max == 1
    assert api_val.details.mid_slot_count.users == sorted([api_module2.id, api_module3.id])


def test_holes(client, consts):
    # Check what happens when not all slots are filled with modules
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.ApiRack.mid,
        state=consts.ApiModuleState.offline,
        mode={consts.ApiModAddMode.insert: 5})
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (6, 3)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 6
    assert api_val.details.mid_slot_count.max == 3
    assert api_val.details.mid_slot_count.users == [api_module1.id]
    # Action
    api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.ApiRack.mid,
        state=consts.ApiModuleState.offline,
        mode={consts.ApiModAddMode.insert: 2})
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (7, 3)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 7
    assert api_val.details.mid_slot_count.max == 3
    assert api_val.details.mid_slot_count.users == [api_module1.id]
    # Action
    api_module1.remove()
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (3, 3)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_equal(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, rack=consts.ApiRack.mid, state=consts.ApiModuleState.offline)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (1, 1)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 3})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_module1 = api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.ApiRack.mid,
        state=consts.ApiModuleState.offline,
        mode={consts.ApiModAddMode.replace: 0})
    # Verification - check case with KF specified, but used <= max being true
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (1, 3)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=(True, [api_module1.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module2 = api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.ApiRack.mid,
        state=consts.ApiModuleState.offline,
        mode={consts.ApiModAddMode.replace: 2})
    # Verification - check case with KF specified, but used <= max being true
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (3, 3)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=(True, [api_module2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module3 = api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.ApiRack.mid,
        state=consts.ApiModuleState.offline,
        mode={consts.ApiModAddMode.replace: 6})
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (7, 3)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=(True, [api_module2.id])))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 7
    assert api_val.details.mid_slot_count.max == 3
    assert api_val.details.mid_slot_count.users == [api_module3.id]
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=(True, [api_module3.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module4 = api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.ApiRack.mid,
        state=consts.ApiModuleState.offline,
        mode={consts.ApiModAddMode.replace: 4})
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (7, 3)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=(True, [api_module3.id])))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 7
    assert api_val.details.mid_slot_count.max == 3
    assert api_val.details.mid_slot_count.users == [api_module4.id]
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=(True, [api_module4.id])))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 7
    assert api_val.details.mid_slot_count.max == 3
    assert api_val.details.mid_slot_count.users == [api_module3.id]
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=(True, [api_module3.id, api_module4.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=ValOptions(
        mid_slot_count=(True, [api_module3.id, api_other.id, api_module4.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.ApiRack.mid,
        state=consts.ApiModuleState.offline,
        mode={consts.ApiModAddMode.replace: 1})
    # Verification - module has been added within slot limit, so it does not trigger anything
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (7, 3)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=(True, [api_module3.id, api_module4.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    # Action
    api_module6 = api_fit.add_module(
        type_id=eve_module_id,
        rack=consts.ApiRack.mid,
        state=consts.ApiModuleState.offline,
        mode={consts.ApiModAddMode.replace: 3})
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (7, 3)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=(True, [api_module3.id, api_module4.id])))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 7
    assert api_val.details.mid_slot_count.max == 3
    assert api_val.details.mid_slot_count.users == [api_module6.id]


def test_modified_max(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, rack=consts.ApiRack.mid, state=consts.ApiModuleState.offline)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].extra == approx(0)
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (1, 0)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 1
    assert api_val.details.mid_slot_count.max == 0
    assert api_val.details.mid_slot_count.users == [api_module.id]
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].extra == approx(1)
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (1, 1)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_fractional_max(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_module_id = client.mk_eve_item()
    eve_ship1_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0.4})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    api_module = api_fit.add_module(type_id=eve_module_id, rack=consts.ApiRack.mid, state=consts.ApiModuleState.offline)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (1, 0)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 1
    assert api_val.details.mid_slot_count.max == 0
    assert api_val.details.mid_slot_count.users == [api_module.id]
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (1, 1)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_module_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, rack=consts.ApiRack.mid, state=consts.ApiModuleState.offline)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (1, None)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 1
    assert api_val.details.mid_slot_count.max is None
    assert api_val.details.mid_slot_count.users == [api_module.id]


def test_no_value_max(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, rack=consts.ApiRack.mid, state=consts.ApiModuleState.offline)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (1, 0)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 1
    assert api_val.details.mid_slot_count.max == 0
    assert api_val.details.mid_slot_count.users == [api_module.id]


def test_no_attr_max(client, consts):
    eve_max_attr_id = consts.EveAttr.med_slots
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, rack=consts.ApiRack.mid, state=consts.ApiModuleState.offline)
    # Verification - when output attr does not exist, it is assumed to be 0
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 1
    assert api_val.details.mid_slot_count.max == 0
    assert api_val.details.mid_slot_count.users == [api_module.id]


def test_not_loaded_user(client, consts):
    # Not loaded modules still take slot
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, rack=consts.ApiRack.mid, state=consts.ApiModuleState.offline)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (1, 0)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 1
    assert api_val.details.mid_slot_count.max == 0
    assert api_val.details.mid_slot_count.users == [api_module.id]


def test_not_loaded_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, rack=consts.ApiRack.mid, state=consts.ApiModuleState.offline)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (1, None)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 1
    assert api_val.details.mid_slot_count.max is None
    assert api_val.details.mid_slot_count.users == [api_module.id]


def test_criterion_module_state(client, consts):
    # Slot is taken even when module is disabled
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, rack=consts.ApiRack.mid, state=consts.ApiModuleState.offline)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (1, 0)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 1
    assert api_val.details.mid_slot_count.max == 0
    assert api_val.details.mid_slot_count.users == [api_module.id]
    # Action
    api_module.change_module(state=consts.ApiModuleState.disabled)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (1, 0)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.mid_slot_count.used == 1
    assert api_val.details.mid_slot_count.max == 0
    assert api_val.details.mid_slot_count.users == [api_module.id]


def test_criterion_rack(client, consts):
    # Modules from other racks are ignored
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_module_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, rack=consts.ApiRack.high, state=consts.ApiModuleState.offline)
    api_fit.add_module(type_id=eve_module_id, rack=consts.ApiRack.low, state=consts.ApiModuleState.offline)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (0, 0)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_criterion_item_kind(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.med_slots)
    eve_item_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ftr_abil_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(attrs={eve_autocharge_attr_id: eve_item_id}, eff_ids=[eve_autocharge_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_booster(type_id=eve_item_id)
    api_fit.set_character(type_id=eve_item_id)
    api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_item_id)
    api_fit.add_implant(type_id=eve_item_id)
    api_module = api_fit.add_module(
        type_id=eve_item_id,
        rack=consts.ApiRack.mid,
        state=consts.ApiModuleState.overload,
        charge_type_id=eve_item_id)
    api_fit.add_rig(type_id=eve_item_id)
    api_fit.add_service(type_id=eve_item_id, state=consts.ApiServiceState.online)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    api_fit.set_stance(type_id=eve_item_id)
    api_fit.add_subsystem(type_id=eve_item_id)
    # Verification - KF module itself, we still check its charge
    assert len(api_fighter.autocharges) == 1
    api_stats = api_fit.get_stats(options=FitStatsOptions(mid_slots=True))
    assert api_stats.mid_slots == (1, 0)
    api_val = api_fit.validate(options=ValOptions(mid_slot_count=(True, [api_module.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
