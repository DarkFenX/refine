from tests import approx, check_no_field
from tests.fw.api import FitStatsOptions, ItemStatsOptions


def test_ship_modified(client, consts):
    eve_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_speed_mult)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mult_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_mult_attr_id)
    eve_mult_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mult_mod])
    eve_mult_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 50}, eff_ids=[eve_mult_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_mult_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(warp_speed=True))
    assert api_fit_stats.warp_speed == approx(3)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(warp_speed=True))
    assert api_ship_stats.warp_speed == approx(3)
    # Action
    api_mult_rig = api_fit.add_rig(type_id=eve_mult_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(warp_speed=True))
    assert api_fit_stats.warp_speed == approx(4.5)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(warp_speed=True))
    assert api_ship_stats.warp_speed == approx(4.5)
    # Action
    api_mult_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(warp_speed=True))
    assert api_fit_stats.warp_speed == approx(3)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(warp_speed=True))
    assert api_ship_stats.warp_speed == approx(3)


def test_ship_value(client, consts):
    eve_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_speed_mult)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_mult_attr_id: 0})
    eve_ship2_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(warp_speed=True))
    assert api_fit_stats.warp_speed is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(warp_speed=True))
    assert api_ship_stats.warp_speed is None
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(warp_speed=True))
    assert api_fit_stats.warp_speed is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(warp_speed=True))
    assert api_ship_stats.warp_speed is None


def test_ship_absent(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.warp_speed_mult)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(warp_speed=True))
    assert api_fit_stats.warp_speed is None


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.warp_speed_mult)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(warp_speed=True))
    assert api_fit_stats.warp_speed is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(warp_speed=True))
    assert api_ship_stats.warp_speed is None


def test_struct(client, consts):
    eve_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_speed_mult)
    eve_struct_id = client.mk_eve_struct(attrs={eve_mult_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(warp_speed=True))
    assert api_fit_stats.warp_speed is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(warp_speed=True))
    assert api_ship_stats.warp_speed is None


def test_drone(client, consts):
    eve_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_speed_mult)
    eve_drone_id = client.mk_eve_drone(attrs={eve_mult_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(warp_speed=True))
    assert api_drone_stats.warp_speed is None


def test_fighter_modified(client, consts):
    eve_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_speed_mult)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_mult_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 20},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_fighter_id = client.mk_eve_fighter(attrs={eve_mult_attr_id: 1.5, eve_max_count_attr_id: 9})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(warp_speed=True))
    assert api_fighter_stats.warp_speed == approx(1.5)
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(warp_speed=True))
    assert api_fighter_stats.warp_speed == approx(1.8)
    # Action
    api_fw_effect.remove()
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(warp_speed=True))
    assert api_fighter_stats.warp_speed == approx(1.5)


def test_other(client, consts):
    eve_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_speed_mult)
    eve_module_id = client.mk_eve_item(attrs={eve_mult_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(warp_speed=True))
    assert api_module_stats.warp_speed is None


def test_not_requested(client, consts):
    eve_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_speed_mult)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mult_attr_id: 3})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(warp_speed=False))
    with check_no_field():
        api_fit_stats.warp_speed  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(warp_speed=False))
    with check_no_field():
        api_ship_stats.warp_speed  # noqa: B018
