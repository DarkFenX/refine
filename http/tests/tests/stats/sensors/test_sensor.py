"""
Sensors are a tricky subject in EVE, because ships can have multiple sensor strength more than 0 at
the same time. Some ships like council diplomatic shuttle have "omni" sensors. Others can just use
low-grade ECCM implant sets which increase sensor strength by some specific absolute value. But, as
testing has shown, EVE resolves ties in strength using similar way to RAH tie resolution, just by
sensor strength attribute ID, and considers ship having that sensor type with that sensor strength.
It is then used for everything, including jams. Fitting screen seems to consistently show the sensor
type EVE chose for a ship. Following is the tests which were conducted on Thunderdome:

Merlin with Legion jams targets a target with 2 sensor types. Target has 4 base of native sensor
strength, and 4 from LG ECCM set, both are boosted to 4.8 with skills. Jams have strength of 5.125
vs their respectable sensor type, and 1.625 vs the rest.

venture (4.8 green, 4.8 yellow, fitting yellow 4.8):
- green jams: sub-100%
- yellow jams: 100%
venture (4.8 green, 4.8 blue, fitting green 4.8):
- green jams: 100%
- blue jams: sub-100%
venture (4.8 green, 4.8 red, fitting red 4.8):
- green jams: sub-100%
- red jams: 100%

sigil (4.8 yellow, 4.8 blue, fitting yellow 4.8):
- yellow jams: 100%
- blue jams: sub-100%
sigil (4.8 yellow, 4.8 green, fitting yellow 4.8):
- yellow jams: 100%
- green jams: sub-100%
sigil (4.8 yellow, 4.8 red, fitting yellow 4.8):
- yellow jams: 100%
- red jams: sub-100%

Griffin with 4 rainbow Panola jams (~9.84 primary jam strength, ~3.28 secondary) vs council
diplomatic shuttle (9.6 strength of all types): only yellow is consistently jamming.
"""

from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions


def test_ship_modified_radar(client, consts):
    eve_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_radar_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 25}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_radar_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.radar, approx(25)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.radar, approx(25)]
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]


def test_ship_modified_gravimetric(client, consts):
    eve_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_grav_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 25}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_grav_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.gravimetric, approx(20)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.gravimetric, approx(20)]
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.gravimetric, approx(25)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.gravimetric, approx(25)]
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.gravimetric, approx(20)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.gravimetric, approx(20)]


def test_ship_modified_magnetometric(client, consts):
    eve_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_magnet_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 25}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_magnet_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.magnetometric, approx(20)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.magnetometric, approx(20)]
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.magnetometric, approx(25)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.magnetometric, approx(25)]
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.magnetometric, approx(20)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.magnetometric, approx(20)]


def test_ship_modified_ladar(client, consts):
    eve_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_ladar_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 25}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_ladar_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.ladar, approx(20)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.ladar, approx(20)]
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.ladar, approx(25)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.ladar, approx(25)]
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.ladar, approx(20)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.ladar, approx(20)]


def test_ship_no_value(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength)
    client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.radar, 0]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.radar, 0]


def test_ship_absent(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength)
    client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor is None


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength)
    client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor is None


def test_struct_modified(client, consts):
    eve_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_radar_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 25}, eff_ids=[eve_mod_effect_id])
    eve_struct_id = client.mk_eve_struct(attrs={eve_radar_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.radar, approx(25)]
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.radar, approx(25)]
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]


def test_drone_modified(client, consts):
    eve_skill_id = client.mk_eve_item()
    eve_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_radar_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 25},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_drone_id = client.mk_eve_item(attrs={eve_radar_attr_id: 20}, srqs={eve_skill_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_drone_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_drone_stats.sensor == [consts.ApiSensorKind.radar, approx(25)]
    # Action
    api_fw_effect.remove()
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_drone_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]


def test_fighter_modified(client, consts):
    eve_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_buff_type_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_id)
    eve_buff_val_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warfare_buff_1_value)
    eve_buff_id = client.mk_eve_buff(
        aggr_mode=consts.EveBuffAggrMode.max,
        op=consts.EveBuffOp.post_percent,
        item_mods=[client.mk_eve_buff_mod(attr_id=eve_radar_attr_id)])
    eve_effect_id = client.mk_eve_effect(id_=consts.UtilEffect.buff_everything, cat_id=consts.EveEffCat.active)
    eve_fw_effect_id = client.mk_eve_item(
        attrs={eve_buff_type_attr_id: eve_buff_id, eve_buff_val_attr_id: 25},
        eff_ids=[eve_effect_id], defeff_id=eve_effect_id)
    eve_fighter_id = client.mk_eve_item(attrs={eve_radar_attr_id: 20, eve_max_count_attr_id: 9})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_fighter_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]
    # Action
    api_fw_effect = api_fit.add_fw_effect(type_id=eve_fw_effect_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_fighter_stats.sensor == [consts.ApiSensorKind.radar, approx(25)]
    # Action
    api_fw_effect.remove()
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_fighter_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]


def test_other(client, consts):
    eve_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    eve_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    eve_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength)
    eve_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength)
    eve_module_id = client.mk_eve_item(
        attrs={eve_radar_attr_id: 20, eve_grav_attr_id: 20, eve_magnet_attr_id: 20, eve_ladar_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_module_stats.sensor is None


def test_tie_resolution_omni(client, consts):
    eve_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    eve_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    eve_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength)
    eve_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_radar_attr_id: 20, eve_grav_attr_id: 20, eve_magnet_attr_id: 20, eve_ladar_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]


def test_tie_resolution_radar_ladar(client, consts):
    eve_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    eve_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength)
    eve_ship_id = client.mk_eve_ship(attrs={eve_radar_attr_id: 20, eve_ladar_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.radar, approx(20)]


def test_tie_resolution_ladar_magnetometric(client, consts):
    eve_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength)
    eve_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength)
    eve_ship_id = client.mk_eve_ship(attrs={eve_magnet_attr_id: 20, eve_ladar_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.ladar, approx(20)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.ladar, approx(20)]


def test_tie_resolution_magnetometric_gravimetric(client, consts):
    eve_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    eve_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength)
    eve_ship_id = client.mk_eve_ship(attrs={eve_grav_attr_id: 20, eve_magnet_attr_id: 20})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(sensor=True))
    assert api_fit_stats.sensor == [consts.ApiSensorKind.magnetometric, approx(20)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(sensor=True))
    assert api_ship_stats.sensor == [consts.ApiSensorKind.magnetometric, approx(20)]
