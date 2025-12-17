from fw import approx, check_no_field
from fw.api import FitStatsOptions, ItemStatsOptions


def test_ship_modified(client, consts):
    eve_dscan_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_directional_scan_range)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_dscan_range_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 25}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_dscan_range_attr_id: 2147483647000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dscan_range=True))
    assert api_fit_stats.dscan_range == approx(14.355042)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(dscan_range=True))
    assert api_ship_stats.dscan_range == approx(14.355042)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dscan_range=True))
    assert api_fit_stats.dscan_range == approx(17.943802)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(dscan_range=True))
    assert api_ship_stats.dscan_range == approx(17.943802)
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dscan_range=True))
    assert api_fit_stats.dscan_range == approx(14.355042)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(dscan_range=True))
    assert api_ship_stats.dscan_range == approx(14.355042)


def test_ship_no_value(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.max_directional_scan_range)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dscan_range=True))
    assert api_fit_stats.dscan_range == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(dscan_range=True))
    assert api_ship_stats.dscan_range == 0


def test_ship_absent(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.max_directional_scan_range)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dscan_range=True))
    assert api_fit_stats.dscan_range is None


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.max_directional_scan_range)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dscan_range=True))
    assert api_fit_stats.dscan_range is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(dscan_range=True))
    assert api_ship_stats.dscan_range is None


def test_struct_modified(client, consts):
    eve_dscan_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_directional_scan_range)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.struct,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_dscan_range_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 25}, eff_ids=[eve_mod_effect_id])
    eve_struct_id = client.mk_eve_struct(attrs={eve_dscan_range_attr_id: 2147483647000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dscan_range=True))
    assert api_fit_stats.dscan_range == approx(14.355042)
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(dscan_range=True))
    assert api_ship_stats.dscan_range == approx(14.355042)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dscan_range=True))
    assert api_fit_stats.dscan_range == approx(17.943802)
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(dscan_range=True))
    assert api_ship_stats.dscan_range == approx(17.943802)
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dscan_range=True))
    assert api_fit_stats.dscan_range == approx(14.355042)
    api_ship_stats = api_struct.get_stats(options=ItemStatsOptions(dscan_range=True))
    assert api_ship_stats.dscan_range == approx(14.355042)


def test_other(client, consts):
    eve_dscan_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_directional_scan_range)
    eve_drone_id = client.mk_eve_drone(attrs={eve_dscan_range_attr_id: 2147483647000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(dscan_range=True))
    assert api_drone_stats.dscan_range is None


def test_not_requested(client, consts):
    eve_dscan_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_directional_scan_range)
    eve_ship_id = client.mk_eve_ship(attrs={eve_dscan_range_attr_id: 2147483647000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dscan_range=False))
    with check_no_field():
        api_fit_stats.dscan_range  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(dscan_range=False))
    with check_no_field():
        api_ship_stats.dscan_range  # noqa: B018
