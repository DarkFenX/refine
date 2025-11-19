from tests import check_no_field
from tests.fw.api import FitStatsOptions, ItemStatsOptions


def test_ship_modified_limit_ship(client, consts):
    eve_locks_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_locked_targets)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_locks_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 2}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_locks_attr_id: 7})
    eve_char_id = client.mk_eve_item(attrs={eve_locks_attr_id: 12})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 7
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 7
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 9
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 9
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 7
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 7


def test_ship_modified_limit_char(client, consts):
    eve_locks_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_locked_targets)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_locks_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 5}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_locks_attr_id: 10})
    eve_char_id = client.mk_eve_item(attrs={eve_locks_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 2
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 2
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 7
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 7
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 2
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 2


def test_ship_rounding(client, consts):
    # Count of locks is halved in pochven, and is rounded away from 0. Tested on 2025-08-29 using
    # loki with 7 -> 4 locks, and loki with 9 -> 5 locks.
    eve_locks_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_locked_targets)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_mul,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_locks_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.system, mod_info=[eve_mod])
    eve_sw_effect_id = client.mk_eve_item(attrs={eve_mod_attr_id: 0.5}, eff_ids=[eve_mod_effect_id])
    eve_ship1_id = client.mk_eve_ship(attrs={eve_locks_attr_id: 7})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_locks_attr_id: 9})
    eve_char_id = client.mk_eve_item(attrs={eve_locks_attr_id: 12})
    client.create_sources()
    api_sol = client.create_sol()
    api_sol.add_sw_effect(type_id=eve_sw_effect_id)
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 4
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 4
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 5
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 5


def test_ship_char_absent(client, consts):
    eve_locks_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_locked_targets)
    eve_ship_id = client.mk_eve_ship(attrs={eve_locks_attr_id: 7})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 7
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 7


def test_ship_char_not_loaded(client, consts):
    eve_locks_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_locked_targets)
    eve_ship_id = client.mk_eve_ship(attrs={eve_locks_attr_id: 7})
    eve_char_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 7
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 7


def test_ship_no_value(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.max_locked_targets)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 0


def test_ship_absent(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.max_locked_targets)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks is None


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.max_locked_targets)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks is None


def test_struct_modified_limit_struct(client, consts):
    eve_locks_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_locked_targets)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_locks_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 2}, eff_ids=[eve_mod_effect_id])
    eve_struct_id = client.mk_eve_struct(attrs={eve_locks_attr_id: 7})
    eve_char_id = client.mk_eve_item(attrs={eve_locks_attr_id: 12})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 7
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 7
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 9
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 9
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 7
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 7


def test_struct_modified_limit_char(client, consts):
    eve_locks_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_locked_targets)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_locks_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 5}, eff_ids=[eve_mod_effect_id])
    eve_struct_id = client.mk_eve_struct(attrs={eve_locks_attr_id: 10})
    eve_char_id = client.mk_eve_item(attrs={eve_locks_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 2
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 2
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 7
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 7
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=True))
    assert api_fit_stats.locks == 2
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(locks=True))
    assert api_ship_stats.locks == 2


def test_drone_modified(client, consts):
    eve_locks_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_locked_targets)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_locks_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 2},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_drone(attrs={eve_locks_attr_id: 8})
    eve_char_id = client.mk_eve_item(attrs={eve_locks_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(locks=True))
    assert api_drone_stats.locks == 8
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(locks=True))
    assert api_drone_stats.locks == 10
    # Action
    api_fw_effect.remove()
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(locks=True))
    assert api_drone_stats.locks == 8


def test_fighter_modified(client, consts):
    eve_locks_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_locked_targets)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.mod_add,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_locks_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 2},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_locks_attr_id: 8, eve_max_count_attr_id: 9})
    eve_char_id = client.mk_eve_item(attrs={eve_locks_attr_id: 2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(locks=True))
    assert api_fighter_stats.locks == 8
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(locks=True))
    assert api_fighter_stats.locks == 10
    # Action
    api_fw_effect.remove()
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(locks=True))
    assert api_fighter_stats.locks == 8


def test_other(client, consts):
    eve_locks_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_locked_targets)
    eve_module_id = client.mk_eve_item(attrs={eve_locks_attr_id: 8})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(locks=True))
    assert api_module_stats.locks is None


def test_not_requested(client, consts):
    eve_locks_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_locked_targets)
    eve_ship_id = client.mk_eve_ship(attrs={eve_locks_attr_id: 7})
    eve_char_id = client.mk_eve_item(attrs={eve_locks_attr_id: 12})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_character(type_id=eve_char_id)
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(locks=False))
    with check_no_field():
        api_fit_stats.locks  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(locks=False))
    with check_no_field():
        api_ship_stats.locks  # noqa: B018
