from tests import approx, check_no_field
from tests.fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionCapBalance


def test_ship_modified(client, consts):
    eve_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_amount_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: -20}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_amount_attr_id: 518.76, eve_regen_attr_id: 233437.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(5.555663)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(5.555663)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(4.44453)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(4.44453)
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(5.555663)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(5.555663)


def test_ship_no_value(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == 0


def test_ship_absent(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance is None


def test_struct(client, consts):
    eve_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_struct_id = client.mk_eve_struct(attrs={eve_amount_attr_id: 725000, eve_regen_attr_id: 4317806.592})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(419.773318)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(419.773318)


def test_other(client, consts):
    eve_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_drone_id = client.mk_eve_item(attrs={eve_amount_attr_id: 518.76, eve_regen_attr_id: 233437.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_drone_stats.cap_balance is None


def test_cap_perc(client, consts):
    eve_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_ship_id = client.mk_eve_ship(attrs={eve_amount_attr_id: 518.76, eve_regen_attr_id: 233437.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_cap_balance_options = [
        StatsOptionCapBalance(regen_perc=0.25),
        StatsOptionCapBalance(),
        StatsOptionCapBalance(regen_perc=0),
        StatsOptionCapBalance(regen_perc=0.7),
        StatsOptionCapBalance(regen_perc=0.9),
        StatsOptionCapBalance(regen_perc=1)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=(True, api_cap_balance_options)))
    assert api_fit_stats.cap_balance == [approx(5.555663), approx(5.555663), 0, approx(3.036948), approx(1.081871), 0]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=(True, api_cap_balance_options)))
    assert api_ship_stats.cap_balance == [approx(5.555663), approx(5.555663), 0, approx(3.036948), approx(1.081871), 0]


def test_attr_regen_zero(client, consts):
    eve_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_ship_id = client.mk_eve_ship(attrs={eve_amount_attr_id: 518.76, eve_regen_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == 0


def test_attr_both_zero(client, consts):
    eve_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_ship_id = client.mk_eve_ship(attrs={eve_amount_attr_id: 0, eve_regen_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == 0


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance is None


def test_not_requested(client, consts):
    eve_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_regen_attr_id = client.mk_eve_attr(id_=consts.EveAttr.recharge_rate)
    eve_ship_id = client.mk_eve_ship(attrs={eve_amount_attr_id: 518.76, eve_regen_attr_id: 233437.5})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=False))
    with check_no_field():
        api_fit_stats.cap_balance  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=False))
    with check_no_field():
        api_ship_stats.cap_balance  # noqa: B018
