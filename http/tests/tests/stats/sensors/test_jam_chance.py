
from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions


def test_module_regular_ship(client, consts):
    eve_sensor_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength)
    eve_sensor_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength)
    eve_jam_radar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_radar_strength_bonus)
    eve_jam_magnet_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_magnetometric_strength_bonus)
    eve_jam_grav_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_gravimetric_strength_bonus)
    eve_jam_ladar_attr_id = client.mk_eve_attr(id_=consts.EveAttr.scan_ladar_strength_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_jam_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.remote_ecm_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_jam_radar_attr_id: 12, eve_jam_magnet_attr_id: 3, eve_jam_grav_attr_id: 3, eve_jam_ladar_attr_id: 3,
            eve_cycle_time_attr_id: 20000, eve_optimal_attr_id: 50000, eve_falloff_attr_id: 25000},
        eff_ids=[eve_jam_effect_id],
        defeff_id=eve_jam_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 300})
    eve_tgt_ship1_id = client.mk_eve_ship(attrs={eve_sensor_grav_attr_id: 25, eve_radius_attr_id: 150})
    eve_tgt_ship2_id = client.mk_eve_ship(attrs={eve_sensor_radar_attr_id: 22, eve_radius_attr_id: 150})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 0, 0))
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(jam_chance=True))
    assert api_tgt_fit_stats.jam_chance == approx(0.12)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(jam_chance=True))
    assert api_tgt_ship_stats.jam_chance == approx(0.12)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(jam_chance=True))
    assert api_tgt_fit_stats.jam_chance == approx(0.5454545)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(jam_chance=True))
    assert api_tgt_ship_stats.jam_chance == approx(0.5454545)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 50450, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(jam_chance=True))
    assert api_tgt_fit_stats.jam_chance == approx(0.5454545)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(jam_chance=True))
    assert api_tgt_ship_stats.jam_chance == approx(0.5454545)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 75450, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(jam_chance=True))
    assert api_tgt_fit_stats.jam_chance == approx(0.2727273)
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(jam_chance=True))
    assert api_tgt_ship_stats.jam_chance == approx(0.2727273)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 125451, 0))
    # Verification
    api_tgt_fit_stats = api_tgt_fit.get_stats(options=FitStatsOptions(jam_chance=True))
    assert api_tgt_fit_stats.jam_chance == 0
    api_tgt_ship_stats = api_tgt_ship.get_stats(options=ItemStatsOptions(jam_chance=True))
    assert api_tgt_ship_stats.jam_chance == 0
