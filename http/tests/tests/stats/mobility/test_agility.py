from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions


def test_ship_modified_mass(client, consts):
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_mass_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 500000}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_agility_attr_id: 3.2, eve_mass_attr_id: 1050000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(agility=True, align_time=True))
    assert api_fit_stats.agility == approx(4.657949)
    assert api_fit_stats.align_time == 5
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(agility=True, align_time=True))
    assert api_ship_stats.agility == approx(4.657949)
    assert api_ship_stats.align_time == 5
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(agility=True, align_time=True))
    assert api_fit_stats.agility == approx(6.87602)
    assert api_fit_stats.align_time == 7
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(agility=True, align_time=True))
    assert api_ship_stats.agility == approx(6.87602)
    assert api_ship_stats.align_time == 7
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(agility=True, align_time=True))
    assert api_fit_stats.agility == approx(4.657949)
    assert api_fit_stats.align_time == 5
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(agility=True, align_time=True))
    assert api_ship_stats.agility == approx(4.657949)
    assert api_ship_stats.align_time == 5


def test_ship_modified_agility(client, consts):
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_mass_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: -20}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_agility_attr_id: 3.2, eve_mass_attr_id: 1050000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(agility=True, align_time=True))
    assert api_fit_stats.agility == approx(4.657949)
    assert api_fit_stats.align_time == 5
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(agility=True, align_time=True))
    assert api_ship_stats.agility == approx(4.657949)
    assert api_ship_stats.align_time == 5
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(agility=True, align_time=True))
    assert api_fit_stats.agility == approx(3.726359)
    assert api_fit_stats.align_time == 4
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(agility=True, align_time=True))
    assert api_ship_stats.agility == approx(3.726359)
    assert api_ship_stats.align_time == 4
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(agility=True, align_time=True))
    assert api_fit_stats.agility == approx(4.657949)
    assert api_fit_stats.align_time == 5
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(agility=True, align_time=True))
    assert api_ship_stats.agility == approx(4.657949)
    assert api_ship_stats.align_time == 5


def test_ship_zero_value_mass(client, consts):
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.mk_eve_ship(attrs={eve_agility_attr_id: 3.2, eve_mass_attr_id: 0})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(agility=True, align_time=True))
    assert api_fit_stats.agility is None
    assert api_fit_stats.align_time is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(agility=True, align_time=True))
    assert api_ship_stats.agility is None
    assert api_ship_stats.align_time is None


def test_ship_zero_value_agility(client, consts):
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.mk_eve_ship(attrs={eve_agility_attr_id: 0, eve_mass_attr_id: 1050000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(agility=True, align_time=True))
    assert api_fit_stats.agility is None
    assert api_fit_stats.align_time is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(agility=True, align_time=True))
    assert api_ship_stats.agility is None
    assert api_ship_stats.align_time is None


def test_ship_no_value_mass(client, consts):
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.mk_eve_ship(attrs={eve_agility_attr_id: 3.2})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(agility=True, align_time=True))
    assert api_fit_stats.agility is None
    assert api_fit_stats.align_time is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(agility=True, align_time=True))
    assert api_ship_stats.agility is None
    assert api_ship_stats.align_time is None


def test_ship_no_value_agility(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1050000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(agility=True, align_time=True))
    assert api_fit_stats.agility is None
    assert api_fit_stats.align_time is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(agility=True, align_time=True))
    assert api_ship_stats.agility is None
    assert api_ship_stats.align_time is None


def test_ship_no_ship(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.agility)
    client.mk_eve_attr(id_=consts.EveAttr.mass)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(agility=True, align_time=True))
    assert api_fit_stats.agility is None
    assert api_fit_stats.align_time is None


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.agility)
    client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(agility=True, align_time=True))
    assert api_fit_stats.agility is None
    assert api_fit_stats.align_time is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(agility=True, align_time=True))
    assert api_ship_stats.agility is None
    assert api_ship_stats.align_time is None


def test_drone(client, consts):
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_drone_id = client.mk_eve_ship(attrs={eve_agility_attr_id: 100, eve_mass_attr_id: 10000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id)
    # Verification
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(agility=True, align_time=True))
    assert api_drone_stats.agility == approx(1.386294)
    assert api_drone_stats.align_time == 2


def test_fighter(client, consts):
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_fighter_id = client.mk_eve_ship(attrs={eve_agility_attr_id: 700, eve_mass_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_drone(type_id=eve_fighter_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(agility=True, align_time=True))
    assert api_fighter_stats.agility == approx(0.970406)
    assert api_fighter_stats.align_time == 1


def test_other(client, consts):
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_module_id = client.mk_eve_ship(attrs={eve_agility_attr_id: 3.2, eve_mass_attr_id: 1050000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(agility=True, align_time=True))
    assert api_module_stats.agility is None
    assert api_module_stats.align_time is None
