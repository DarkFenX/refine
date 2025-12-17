from fw import approx
from fw.api import (
    FitStatsOptions,
    ItemStatsOptions,
    StatCapConsumerOptions,
    StatCapSrcKinds,
    StatsOptionCapBalance,
)


def test_state(client, consts):
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 30, eve_cycle_time_attr_id: 15000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(-2)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(-2)
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
    assert api_fit_stats.cap_balance.one() == approx(-2)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(-2)


def test_modified(client, consts):
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_mod_attr_id = client.mk_eve_attr()
    eve_consume_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 30, eve_cycle_time_attr_id: 15000},
        eff_ids=[eve_consume_effect_id],
        defeff_id=eve_consume_effect_id)
    eve_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.loc,
        loc=consts.EveModLoc.ship,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_mod_attr_id,
        affectee_attr_id=eve_use_attr_id)
    eve_mod_effect_id = client.mk_eve_effect(mod_info=[eve_mod])
    eve_rig_id = client.mk_eve_item(attrs={eve_mod_attr_id: -20}, eff_ids=[eve_mod_effect_id])
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(-2)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(-2)
    # Action
    api_rig = api_fit.add_rig(type_id=eve_rig_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(-1.6)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(-1.6)
    # Action
    api_rig.remove()
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(-2)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(-2)


def test_src_kind(client, consts):
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 30, eve_cycle_time_attr_id: 15000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_options = [
        StatsOptionCapBalance(),
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(default=False, consumers=True)),
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(default=True, consumers=False))]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=(True, api_options)))
    assert api_fit_stats.cap_balance == [approx(-2), approx(-2), 0]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=(True, api_options)))
    assert api_ship_stats.cap_balance == [approx(-2), approx(-2), 0]


def test_reload(client, consts):
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.projectile_fired,
        cat_id=consts.EveEffCat.target,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 3, eve_cycle_time_attr_id: 2500, eve_reload_attr_id: 5000, eve_capacity_attr_id: 1.5},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.0125})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    # Verification
    api_options = [
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(consumers=True)),
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(consumers=(True, StatCapConsumerOptions()))),
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(consumers=(True, StatCapConsumerOptions(reload=False)))),
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(consumers=(True, StatCapConsumerOptions(reload=True))))]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=(True, api_options)))
    assert api_fit_stats.cap_balance == [approx(-1.2), approx(-1.2), approx(-1.2), approx(-1.180328)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=(True, api_options)))
    assert api_ship_stats.cap_balance == [approx(-1.2), approx(-1.2), approx(-1.2), approx(-1.180328)]


def test_ancil(client, consts):
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reload_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fueled_armor_repair,
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_use_attr_id: 40,
            eve_cycle_time_attr_id: 4500,
            eve_reload_attr_id: 60000,
            eve_capacity_attr_id: 0.08},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_charge_id = client.mk_eve_item(attrs={eve_volume_attr_id: 0.01})
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_options = [
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(consumers=(True, StatCapConsumerOptions(reload=False)))),
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(consumers=(True, StatCapConsumerOptions(reload=True))))]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=(True, api_options)))
    assert api_fit_stats.cap_balance == [approx(-8.888889), approx(-3.333333)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=(True, api_options)))
    assert api_ship_stats.cap_balance == [approx(-8.888889), approx(-3.333333)]
    # Action
    api_module.change_module(charge_type_id=None)
    # Verification
    api_options = [
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(consumers=(True, StatCapConsumerOptions(reload=False)))),
        StatsOptionCapBalance(src_kinds=StatCapSrcKinds(consumers=(True, StatCapConsumerOptions(reload=True))))]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=(True, api_options)))
    assert api_fit_stats.cap_balance == [approx(-8.888889), approx(-8.888889)]
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=(True, api_options)))
    assert api_ship_stats.cap_balance == [approx(-8.888889), approx(-8.888889)]


def test_reactivation_delay(client, consts):
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_reactivation_attr_id = client.mk_eve_attr(id_=consts.EveAttr.module_reactivation_delay)
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 30, eve_cycle_time_attr_id: 15000, eve_reactivation_attr_id: 45000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == approx(-0.5)
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == approx(-0.5)


def test_effect_no_discharge(client, consts):
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 30, eve_cycle_time_attr_id: 15000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == 0


def test_effect_no_duration(client, consts):
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(cat_id=consts.EveEffCat.active, discharge_attr_id=eve_use_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 30, eve_cycle_time_attr_id: 15000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == 0


def test_attr_zero_cycle_time(client, consts):
    eve_use_attr_id = client.mk_eve_attr()
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        cat_id=consts.EveEffCat.active,
        discharge_attr_id=eve_use_attr_id,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_use_attr_id: 30, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_ship_id = client.mk_eve_ship()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_ship = api_fit.set_ship(type_id=eve_ship_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(cap_balance=True))
    assert api_fit_stats.cap_balance.one() == 0
    api_ship_stats = api_ship.get_stats(options=ItemStatsOptions(cap_balance=True))
    assert api_ship_stats.cap_balance.one() == 0
