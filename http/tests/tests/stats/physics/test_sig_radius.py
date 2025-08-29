from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions


def test_ship_modified(client, consts):
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_sig_radius_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 25}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_sig_radius_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sig_radius=True))
    assert api_fit_stats.sig_radius == approx(100)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_ship_stats.sig_radius == approx(100)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sig_radius=True))
    assert api_fit_stats.sig_radius == approx(125)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_ship_stats.sig_radius == approx(125)
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sig_radius=True))
    assert api_fit_stats.sig_radius == approx(100)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_ship_stats.sig_radius == approx(100)


def test_ship_no_value(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sig_radius=True))
    assert api_fit_stats.sig_radius == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_ship_stats.sig_radius == 0


def test_ship_absent(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sig_radius=True))
    assert api_fit_stats.sig_radius is None


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sig_radius=True))
    assert api_fit_stats.sig_radius is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_ship_stats.sig_radius is None


def test_struct_modified(client, consts):
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_sig_radius_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 25}, eff_ids=[eve_mod_effect_id])
    eve_struct_id = client.mk_eve_struct(attrs={eve_sig_radius_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sig_radius=True))
    assert api_fit_stats.sig_radius == approx(100)
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_ship_stats.sig_radius == approx(100)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sig_radius=True))
    assert api_fit_stats.sig_radius == approx(125)
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_ship_stats.sig_radius == approx(125)
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sig_radius=True))
    assert api_fit_stats.sig_radius == approx(100)
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_ship_stats.sig_radius == approx(100)


def test_drone_modified(client, consts):
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_prop_blow_attr_id = client.mk_eve_attr(id_=consts.EveAttr.entity_max_velocity_sig_radius_mult)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_sig_radius_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 300},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_item(attrs={eve_sig_radius_attr_id: 1350, eve_prop_blow_attr_id: 6})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, prop_mode=consts.ApiNpcPropMode.cruise)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_drone_stats.sig_radius == approx(1350)
    # Action
    api_drone.change_drone(prop_mode=consts.ApiNpcPropMode.chase)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_drone_stats.sig_radius == approx(8100)
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_drone_stats.sig_radius == approx(32400)
    # Action
    api_drone.change_drone(prop_mode=consts.ApiNpcPropMode.cruise)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_drone_stats.sig_radius == approx(5400)
    # Action
    api_fw_effect.remove()
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_drone_stats.sig_radius == approx(1350)


def test_fighter_modified(client, consts):
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_sig_radius_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 300},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_fighter_id = client.mk_eve_item(attrs={eve_sig_radius_attr_id: 873, eve_max_count_attr_id: 9})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_fighter_stats.sig_radius == approx(873)
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_fighter_stats.sig_radius == approx(3492)
    # Action
    api_fw_effect.remove()
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_fighter_stats.sig_radius == approx(873)


def test_other(client, consts):
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_module_id = client.mk_eve_item(attrs={eve_sig_radius_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(sig_radius=True))
    assert api_module_stats.sig_radius is None
