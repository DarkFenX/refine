from fw import approx, check_no_field
from fw.api import FitStatsOptions, ItemStatsOptions


def test_ship_modified(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_cap_need_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_capacitor_need)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_cap_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_cap_attr_id)
    eve_cap_mod_effect_id = client.mk_eve_effect(mod_info=[eve_cap_mod])
    eve_cap_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 25}, eff_ids=[eve_cap_mod_effect_id])
    eve_mass_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.mod_add,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_mass_attr_id)
    eve_mass_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mass_mod])
    eve_mass_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 500000}, eff_ids=[eve_mass_mod_effect_id])
    eve_cap_need_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_cap_need_attr_id)
    eve_cap_need_mod_effect_id = client.mk_eve_effect(mod_info=[eve_cap_need_mod])
    eve_cap_need_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: -25}, eff_ids=[eve_cap_need_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_cap_attr_id: 500, eve_mass_attr_id: 1050000, eve_cap_need_attr_id: 0.000000835})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range == approx(570.287995)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range == approx(570.287995)
    # Action
    api_cap_rig = api_fit.add_rig(type_id=eve_cap_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range == approx(712.859994)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range == approx(712.859994)
    # Action
    api_mass_rig = api_fit.add_rig(type_id=eve_mass_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range == approx(482.905157)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range == approx(482.905157)
    # Action
    api_cap_need_rig = api_fit.add_rig(type_id=eve_cap_need_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range == approx(643.873543)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range == approx(643.873543)
    # Action
    api_cap_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range == approx(515.098835)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range == approx(515.098835)
    # Action
    api_mass_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range == approx(760.383994)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range == approx(760.383994)
    # Action
    api_cap_need_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range == approx(570.287995)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range == approx(570.287995)


def test_ship_cap_value(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_cap_need_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_capacitor_need)
    eve_ship1_id = client.mk_eve_ship(
        attrs={eve_cap_attr_id: 0, eve_mass_attr_id: 1050000, eve_cap_need_attr_id: 0.000000835})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_mass_attr_id: 1050000, eve_cap_need_attr_id: 0.000000835})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range is None
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range is None


def test_ship_mass_value(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_cap_need_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_capacitor_need)
    eve_ship1_id = client.mk_eve_ship(
        attrs={eve_cap_attr_id: 500, eve_mass_attr_id: 0, eve_cap_need_attr_id: 0.000000835})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_cap_attr_id: 500, eve_cap_need_attr_id: 0.000000835})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range is None
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range is None


def test_ship_cap_need_value(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_cap_need_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_capacitor_need)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_cap_attr_id: 500, eve_mass_attr_id: 1050000, eve_cap_need_attr_id: 0})
    eve_ship2_id = client.mk_eve_ship(attrs={eve_cap_attr_id: 500, eve_mass_attr_id: 1050000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range is None
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range is None


def test_ship_all_values(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_cap_need_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_capacitor_need)
    eve_ship1_id = client.mk_eve_ship(attrs={eve_cap_attr_id: 0, eve_mass_attr_id: 0, eve_cap_need_attr_id: 0})
    eve_ship2_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range is None
    # Action
    api_ship.change_ship(type_id=eve_ship2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range is None


def test_ship_absent(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    client.mk_eve_attr(id_=consts.EveAttr.mass)
    client.mk_eve_attr(id_=consts.EveAttr.warp_capacitor_need)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range is None


def test_ship_not_loaded(client, consts):
    client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    client.mk_eve_attr(id_=consts.EveAttr.mass)
    client.mk_eve_attr(id_=consts.EveAttr.warp_capacitor_need)
    eve_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range is None


def test_struct(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_cap_need_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_capacitor_need)
    eve_struct_id = client.mk_eve_struct(
        attrs={eve_cap_attr_id: 500, eve_mass_attr_id: 1050000, eve_cap_need_attr_id: 0.000000835})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=True))
    assert api_fit_stats.max_warp_range is None
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_ship_stats.max_warp_range is None


def test_drone_modified(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_cap_need_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_capacitor_need)
    eve_struct_id = client.mk_eve_struct(
        attrs={eve_cap_attr_id: 500, eve_mass_attr_id: 1050000, eve_cap_need_attr_id: 0.000000835})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_struct = api_fit.set_ship(type_id=eve_struct_id)
    # Verification
    api_drone_stats = api_struct.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_drone_stats.max_warp_range is None


def test_fighter(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_cap_need_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_capacitor_need)
    eve_max_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    eve_fighter_id = client.mk_eve_fighter(attrs={
        eve_cap_attr_id: 500, eve_mass_attr_id: 1050000, eve_cap_need_attr_id: 0.000000835, eve_max_count_attr_id: 9})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id)
    # Verification
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_fighter_stats.max_warp_range is None


def test_other(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_cap_need_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_capacitor_need)
    eve_module_id = client.mk_eve_item(
        attrs={eve_cap_attr_id: 500, eve_mass_attr_id: 1050000, eve_cap_need_attr_id: 0.000000835})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id)
    # Verification
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(max_warp_range=True))
    assert api_module_stats.max_warp_range is None


def test_not_requested(client, consts):
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_cap_need_attr_id = client.mk_eve_attr(id_=consts.EveAttr.warp_capacitor_need)
    eve_ship_id = client.mk_eve_ship(
        attrs={eve_cap_attr_id: 500, eve_mass_attr_id: 1050000, eve_cap_need_attr_id: 0.000000835})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(max_warp_range=False))
    with check_no_field():
        api_fit_stats.max_warp_range  # noqa: B018
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(max_warp_range=False))
    with check_no_field():
        api_ship_stats.max_warp_range  # noqa: B018
