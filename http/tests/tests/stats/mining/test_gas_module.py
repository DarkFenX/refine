from tests import approx
from tests.fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatMiningItemKinds,
    StatsOptionFitMining,
    StatsOptionItemMining,
)


def test_state(client, consts):
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mining_clouds,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 40,
            eve_cycle_time_attr_id: 30000,
            eve_waste_chance_attr_id: 34,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().gas == [approx(1.333333), approx(1.786667)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().gas == [approx(1.333333), approx(1.786667)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(mps=True))
    assert api_module_stats.mps.one().gas == [approx(1.333333), approx(1.786667)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().gas is None
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().gas is None
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        mps=(True, [StatsOptionItemMining(), StatsOptionItemMining(ignore_state=True)])))
    api_module_mps_normal, api_module_mps_ignored = api_module_stats.mps
    assert api_module_mps_normal.gas is None
    assert api_module_mps_ignored.gas == [approx(1.333333), approx(1.786667)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().gas == [approx(1.333333), approx(1.786667)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().gas == [approx(1.333333), approx(1.786667)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(mps=True))
    assert api_module_stats.mps.one().gas == [approx(1.333333), approx(1.786667)]


def test_stacking(client, consts):
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mining_clouds,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 40,
            eve_cycle_time_attr_id: 30000,
            eve_waste_chance_attr_id: 34,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit2 = api_sol.create_fit()
    api_fit2.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().gas == [approx(4), approx(5.36)]
    api_fit1_stats = api_fit1.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit1_stats.mps.one().gas == [approx(2.666667), approx(3.573333)]
    api_fit2_stats = api_fit2.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit2_stats.mps.one().gas == [approx(1.333333), approx(1.786667)]


def test_waste_chance(client, consts):
    # Test that waste chance of >100% is properly processed
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mining_clouds,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 40,
            eve_cycle_time_attr_id: 30000,
            eve_waste_chance_attr_id: 250,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().gas == [approx(1.333333), approx(2.666667)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().gas == [approx(1.333333), approx(2.666667)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(mps=True))
    assert api_module_stats.mps.one().gas == [approx(1.333333), approx(2.666667)]


def test_no_waste(client, consts):
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mining_clouds,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module1_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 40,
            eve_cycle_time_attr_id: 30000,
            eve_waste_chance_attr_id: 0,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 40,
            eve_cycle_time_attr_id: 30000,
            eve_waste_chance_attr_id: 34,
            eve_waste_mult_attr_id: 0},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active)
    api_module2 = api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().gas == [approx(2.666667), approx(2.666667)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().gas == [approx(2.666667), approx(2.666667)]
    api_module1_stats = api_module1.get_stats(options=ItemStatsOptions(mps=True))
    assert api_module1_stats.mps.one().gas == [approx(1.333333), approx(1.333333)]
    api_module2_stats = api_module2.get_stats(options=ItemStatsOptions(mps=True))
    assert api_module2_stats.mps.one().gas == [approx(1.333333), approx(1.333333)]


def test_item_kind(client, consts):
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mining_clouds,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 40,
            eve_cycle_time_attr_id: 30000,
            eve_waste_chance_attr_id: 34,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=(True, [
        StatsOptionFitMining(),
        StatsOptionFitMining(item_kinds=StatMiningItemKinds()),
        StatsOptionFitMining(item_kinds=StatMiningItemKinds(default=False, module=True)),
        StatsOptionFitMining(item_kinds=StatMiningItemKinds(default=True, module=False))])))
    assert api_fleet_stats.mps.map(lambda i: i.gas) == [
        [approx(1.333333), approx(1.786667)],
        [approx(1.333333), approx(1.786667)],
        [approx(1.333333), approx(1.786667)],
        None]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=(True, [
        StatsOptionFitMining(),
        StatsOptionFitMining(item_kinds=StatMiningItemKinds()),
        StatsOptionFitMining(item_kinds=StatMiningItemKinds(default=False, module=True)),
        StatsOptionFitMining(item_kinds=StatMiningItemKinds(default=True, module=False))])))
    assert api_fit_stats.mps.map(lambda i: i.gas) == [
        [approx(1.333333), approx(1.786667)],
        [approx(1.333333), approx(1.786667)],
        [approx(1.333333), approx(1.786667)],
        None]


def test_other_mining_kinds(client, consts):
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mining_clouds,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 40,
            eve_cycle_time_attr_id: 30000,
            eve_waste_chance_attr_id: 34,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().ore is None
    assert api_fleet_stats.mps.one().ice is None
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().ore is None
    assert api_fit_stats.mps.one().ice is None
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(mps=True))
    assert api_module_stats.mps.one().ore is None
    assert api_module_stats.mps.one().ice is None


def test_cycle_time_zero(client, consts):
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mining_clouds,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 40,
            eve_cycle_time_attr_id: 0,
            eve_waste_chance_attr_id: 34,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().gas is None
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().gas is None
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(mps=True))
    assert api_module_stats.mps.one().gas is None


def test_cycle_time_absent(client, consts):
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.mining_clouds, cat_id=consts.EveEffCat.target)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 40,
            eve_cycle_time_attr_id: 30000,
            eve_waste_chance_attr_id: 34,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().gas is None
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().gas is None
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(mps=True))
    assert api_module_stats.mps.one().gas is None


def test_item_not_loaded(client, consts):
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().gas is None
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().gas is None
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(mps=True))
    assert api_module_stats.mps is None
