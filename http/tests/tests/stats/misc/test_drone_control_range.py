from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions


def test_char_modified(client, consts):
    eve_dcr_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_control_distance)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.char,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_dcr_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 30000}, eff_ids=[eve_mod_effect_id])
    eve_char_id = client.mk_eve_item(attrs={eve_dcr_attr_id: 20000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_char = api_fit.set_character(type_id=eve_char_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(drone_control_range=True))
    assert api_fit_stats.drone_control_range == approx(20000)
    api_char_stats = api_char.get_stats(options=ItemStatsOptions(drone_control_range=True))
    assert api_char_stats.drone_control_range == approx(20000)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(drone_control_range=True))
    assert api_fit_stats.drone_control_range == approx(50000)
    api_char_stats = api_char.get_stats(options=ItemStatsOptions(drone_control_range=True))
    assert api_char_stats.drone_control_range == approx(50000)
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(drone_control_range=True))
    assert api_fit_stats.drone_control_range == approx(20000)
    api_char_stats = api_char.get_stats(options=ItemStatsOptions(drone_control_range=True))
    assert api_char_stats.drone_control_range == approx(20000)


def test_char_no_value(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.drone_control_distance)
    eve_char_id = client.mk_eve_item()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_char = api_fit.set_character(type_id=eve_char_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(drone_control_range=True))
    assert api_fit_stats.drone_control_range == 0
    api_char_stats = api_char.get_stats(options=ItemStatsOptions(drone_control_range=True))
    assert api_char_stats.drone_control_range == 0


def test_char_absent(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.drone_control_distance)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(drone_control_range=True))
    assert api_fit_stats.drone_control_range is None


def test_char_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.drone_control_distance)
    eve_char_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_char = api_fit.set_character(type_id=eve_char_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(drone_control_range=True))
    assert api_fit_stats.drone_control_range is None
    api_char_stats = api_char.get_stats(options=ItemStatsOptions(drone_control_range=True))
    assert api_char_stats.drone_control_range is None


def test_other(client, consts):
    eve_dcr_attr_id = client.mk_eve_attr(id_=consts.EveAttr.drone_control_distance)
    eve_ship_id = client.mk_eve_ship(attrs={eve_dcr_attr_id: 20000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(drone_control_range=True))
    assert api_ship_stats.drone_control_range is None
