from fw import approx
from fw.api import FitStatsOptions, ItemStatsOptions, StatCapSrcKinds, StatsOptionCapBalance


def test_state(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_boost_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.power_booster,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 15, eve_cycle_time_attr_id: 12000, eve_reload_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_boost_amount_attr_id: 150, eve_volume_attr_id: 4.5})
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(9.782609)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(9.782609)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == 0
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(9.782609)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(9.782609)


def test_modified_and_limit(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_boost_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_mod_attr_id = client.mk_eve_attr()
    eve_inject_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.power_booster,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 640, eve_cycle_time_attr_id: 12000, eve_reload_attr_id: 10000},
        eff_ids=[eve_inject_effect_id],
        defeff_id=eve_inject_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_boost_amount_attr_id: 3200, eve_volume_attr_id: 96})
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_boost_amount_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: 37.5}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 4000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(234.146341)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(234.146341)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(292.682927)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(292.682927)
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(234.146341)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(234.146341)


def test_charge_switch(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_boost_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.power_booster,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 15, eve_cycle_time_attr_id: 12000, eve_reload_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_charge1_id = client.mk_eve_item(attrs={eve_boost_amount_attr_id: 150, eve_volume_attr_id: 4.5})
    eve_charge2_id = client.mk_eve_item(attrs={eve_boost_amount_attr_id: 400, eve_volume_attr_id: 12})
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(9.782609)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(9.782609)
    # Action
    api_module.change_module(charge_type_id=None)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == 0
    # Action
    api_module.change_module(charge_type_id=eve_charge2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(18.181818)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(18.181818)


def test_src_kind(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_boost_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.power_booster,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 15, eve_cycle_time_attr_id: 12000, eve_reload_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_boost_amount_attr_id: 150, eve_volume_attr_id: 4.5})
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification
    api_options = [
        StatsOptionCapBalance(),
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(default=False, cap_injectors=True)),
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(default=True, cap_injectors=False))]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=(True, api_options)))
    assert api_fit_stats.cap_balance == [approx(9.782609), approx(9.782609), 0]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=(True, api_options)))
    assert api_ship_stats.cap_balance == [approx(9.782609), approx(9.782609), 0]


def test_charge_no_value(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.power_booster,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 15, eve_cycle_time_attr_id: 12000, eve_reload_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 4.5})
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == 0


def test_charge_not_loaded(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.power_booster,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_capacity_attr_id: 15, eve_cycle_time_attr_id: 12000, eve_reload_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_charge_id = client.alloc_item_id()
    eve_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == 0
