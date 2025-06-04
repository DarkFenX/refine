from tests import approx, check_no_field
from tests.fw.api import FitValOptions


def test_fail_single(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.rig_slot_count.used == 1
    assert api_val.details.rig_slot_count.max == 0
    assert api_val.details.rig_slot_count.users == [api_rig.id]


def test_fail_multiple_ship(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.rig_slot_count.used == 2
    assert api_val.details.rig_slot_count.max == 1
    assert api_val.details.rig_slot_count.users == sorted([api_rig1.id, api_rig2.id])


def test_fail_multiple_struct(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_item()
    eve_struct_id = client.mk_eve_struct(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_struct_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.rig_slot_count.used == 2
    assert api_val.details.rig_slot_count.max == 1
    assert api_val.details.rig_slot_count.users == sorted([api_rig1.id, api_rig2.id])


def test_equal(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_known_failures(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 1})
    eve_other_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_other = api_fit.add_implant(type_id=eve_other_id)
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig1 = api_fit.add_rig(type_id=eve_rig_id)
    api_rig2 = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=(True, [api_rig1.id])))
    assert api_val.passed is False
    assert api_val.details.rig_slot_count.used == 2
    assert api_val.details.rig_slot_count.max == 1
    assert api_val.details.rig_slot_count.users == [api_rig2.id]
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=(True, [api_rig2.id])))
    assert api_val.passed is False
    assert api_val.details.rig_slot_count.used == 2
    assert api_val.details.rig_slot_count.max == 1
    assert api_val.details.rig_slot_count.users == [api_rig1.id]
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=(True, [api_rig1.id, api_rig2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=(True, [api_rig1.id, api_other.id, api_rig2.id])))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_modified_max(client, consts):
    # Unrealistic scenario, but modification of max count is supported
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_max_attr_id)
    eve_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_implant_id = client.mk_eve_item(attrs={eve_mod_attr_id: 1}, eff_ids=[eve_effect_id])
    eve_rig_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].extra == approx(0)
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.rig_slot_count.used == 1
    assert api_val.details.rig_slot_count.max == 0
    assert api_val.details.rig_slot_count.users == [api_rig.id]
    # Action
    api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    assert api_ship.update().attrs[eve_max_attr_id].extra == approx(1)
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_fractional_max(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_item()
    eve_ship1_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0.4})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0.6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship1_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.rig_slot_count.used == 1
    assert api_val.details.rig_slot_count.max == 0
    assert api_val.details.rig_slot_count.users == [api_rig.id]
    # Action
    api_fit.set_ship(type_id=eve_ship2_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018


def test_no_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.rig_slot_count.used == 1
    assert api_val.details.rig_slot_count.max is None
    assert api_val.details.rig_slot_count.users == [api_rig.id]


def test_not_loaded_user(client, consts):
    # Not loaded rigs still take slot
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    eve_rig_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.rig_slot_count.used == 1
    assert api_val.details.rig_slot_count.max == 0
    assert api_val.details.rig_slot_count.users == [api_rig.id]


def test_not_loaded_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_item()
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.rig_slot_count.used == 1
    assert api_val.details.rig_slot_count.max is None
    assert api_val.details.rig_slot_count.users == [api_rig.id]


def test_no_value_max(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.rig_slot_count.used == 1
    assert api_val.details.rig_slot_count.max == 0
    assert api_val.details.rig_slot_count.users == [api_rig.id]


def test_criterion_rig_state(client, consts):
    # Slot is taken even when rig is disabled
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_rig_id = client.mk_eve_item()
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.rig_slot_count.used == 1
    assert api_val.details.rig_slot_count.max == 0
    assert api_val.details.rig_slot_count.users == [api_rig.id]
    # Action
    api_rig.change_rig(state=False)
    # Verification
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is False
    assert api_val.details.rig_slot_count.used == 1
    assert api_val.details.rig_slot_count.max == 0
    assert api_val.details.rig_slot_count.users == [api_rig.id]


def test_criterion_item_kind(client, consts):
    eve_max_attr_id = client.mk_eve_attr(id_=consts.EveAttr.upgrade_slots_left)
    eve_item_id = client.mk_eve_item()
    eve_autocharge_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_abil_launch_bomb_type)
    eve_autocharge_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fighter_ability_launch_bomb,
        cat_id=consts.EveEffCat.active)
    eve_fighter_id = client.mk_eve_item(attrs={eve_autocharge_attr_id: eve_item_id}, eff_ids=[eve_autocharge_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_max_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_item_id)
    api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit.add_fw_effect(type_id=eve_item_id)
    api_fit.add_implant(type_id=eve_item_id)
    api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.overload, charge_type_id=eve_item_id)
    api_fit.add_service(type_id=eve_item_id, state=consts.ApiServiceState.online)
    api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_skill(type_id=eve_item_id, level=5)
    api_fit.set_stance(type_id=eve_item_id)
    api_fit.add_subsystem(type_id=eve_item_id)
    # Verification
    assert len(api_fighter.autocharges) == 1
    api_val = api_fit.validate(options=FitValOptions(rig_slot_count=True))
    assert api_val.passed is True
    with check_no_field():
        api_val.details  # noqa: B018
