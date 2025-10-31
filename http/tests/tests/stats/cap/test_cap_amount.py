from tests import approx, check_no_field
from tests.fw.api import FitStatsOptions, ItemStatsOptions


def test_ship_modified(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_cap_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 15}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_cap_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_amount=True))
    assert api_fit_stats.cap_amount == approx(500)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_amount=True))
    assert api_ship_stats.cap_amount == approx(500)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_amount=True))
    assert api_fit_stats.cap_amount == approx(575)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_amount=True))
    assert api_ship_stats.cap_amount == approx(575)
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_amount=True))
    assert api_fit_stats.cap_amount == approx(500)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_amount=True))
    assert api_ship_stats.cap_amount == approx(500)


def test_ship_no_value(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_amount=True))
    assert api_fit_stats.cap_amount == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_amount=True))
    assert api_ship_stats.cap_amount == 0


def test_ship_absent(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_amount=True))
    assert api_fit_stats.cap_amount is None


def test_struct(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_struct_id = client.mk_eve_struct(attrs={eve_cap_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_amount=True))
    assert api_fit_stats.cap_amount == approx(500)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_amount=True))
    assert api_ship_stats.cap_amount == approx(500)


def test_other(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_drone_id = client.mk_eve_item(attrs={eve_cap_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(cap_amount=True))
    assert api_drone_stats.cap_amount is None


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_amount=True))
    assert api_fit_stats.cap_amount is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_amount=True))
    assert api_ship_stats.cap_amount is None


def test_not_requested(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_ship_id = client.mk_eve_ship(attrs={eve_cap_attr_id: 100})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_amount=False))
    with check_no_field():
        api_fit_stats.cap_amount  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_amount=False))
    with check_no_field():
        api_ship_stats.cap_amount  # noqa: B018
