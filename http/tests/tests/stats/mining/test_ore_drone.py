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
        id_=consts.EveEffect.mining,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 990,
            eve_cycle_time_attr_id: 60000,
            eve_waste_chance_attr_id: 60,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().ore == [approx(16.5), approx(9.9)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().ore == [approx(16.5), approx(9.9)]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(mps=True))
    assert api_drone_stats.mps.one().ore == [approx(16.5), approx(9.9)]
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().ore is None
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().ore is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(
        mps=(True, [StatsOptionItemMining(), StatsOptionItemMining(ignore_state=True)])))
    api_drone_mps_normal, api_drone_mps_ignored = api_drone_stats.mps
    assert api_drone_mps_normal.ore is None
    assert api_drone_mps_ignored.ore == [approx(16.5), approx(9.9)]
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().ore == [approx(16.5), approx(9.9)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().ore == [approx(16.5), approx(9.9)]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(mps=True))
    assert api_drone_stats.mps.one().ore == [approx(16.5), approx(9.9)]


def test_stacking(client, consts):
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mining,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 990,
            eve_cycle_time_attr_id: 60000,
            eve_waste_chance_attr_id: 60,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fit1.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fit2 = api_sol.create_fit()
    api_fit2.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().ore == [approx(49.5), approx(29.7)]
    api_fit1_stats = api_fit1.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit1_stats.mps.one().ore == [approx(33), approx(19.8)]
    api_fit2_stats = api_fit2.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit2_stats.mps.one().ore == [approx(16.5), approx(9.9)]


def test_waste_chance(client, consts):
    # Test that waste chance of >100% is properly processed
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mining,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 990,
            eve_cycle_time_attr_id: 60000,
            eve_waste_chance_attr_id: 250,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().ore == [approx(16.5), approx(16.5)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().ore == [approx(16.5), approx(16.5)]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(mps=True))
    assert api_drone_stats.mps.one().ore == [approx(16.5), approx(16.5)]


def test_no_waste(client, consts):
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mining,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone1_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 990,
            eve_cycle_time_attr_id: 60000,
            eve_waste_chance_attr_id: 0,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_drone2_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 990,
            eve_cycle_time_attr_id: 60000,
            eve_waste_chance_attr_id: 60,
            eve_waste_mult_attr_id: 0},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone1 = api_fit.add_drone(type_id=eve_drone1_id, state=consts.ApiMinionState.engaging)
    api_drone2 = api_fit.add_drone(type_id=eve_drone2_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().ore == [approx(33), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().ore == [approx(33), 0]
    api_drone1_stats = api_drone1.get_stats(options=ItemStatsOptions(mps=True))
    assert api_drone1_stats.mps.one().ore == [approx(16.5), 0]
    api_drone2_stats = api_drone2.get_stats(options=ItemStatsOptions(mps=True))
    assert api_drone2_stats.mps.one().ore == [approx(16.5), 0]


def test_item_kind(client, consts):
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mining,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 990,
            eve_cycle_time_attr_id: 60000,
            eve_waste_chance_attr_id: 60,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=(True, [
        StatsOptionFitMining(),
        StatsOptionFitMining(item_kinds=StatMiningItemKinds()),
        StatsOptionFitMining(item_kinds=StatMiningItemKinds(default=False, minion=True)),
        StatsOptionFitMining(item_kinds=StatMiningItemKinds(default=True, minion=False))])))
    assert api_fleet_stats.mps.map(lambda i: i.ore) == [
        [approx(16.5), approx(9.9)],
        [approx(16.5), approx(9.9)],
        [approx(16.5), approx(9.9)],
        None]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=(True, [
        StatsOptionFitMining(),
        StatsOptionFitMining(item_kinds=StatMiningItemKinds()),
        StatsOptionFitMining(item_kinds=StatMiningItemKinds(default=False, minion=True)),
        StatsOptionFitMining(item_kinds=StatMiningItemKinds(default=True, minion=False))])))
    assert api_fit_stats.mps.map(lambda i: i.ore) == [
        [approx(16.5), approx(9.9)],
        [approx(16.5), approx(9.9)],
        [approx(16.5), approx(9.9)],
        None]


def test_other_mining_kinds(client, consts):
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mining,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 990,
            eve_cycle_time_attr_id: 60000,
            eve_waste_chance_attr_id: 60,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().ice is None
    assert api_fleet_stats.mps.one().gas is None
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().ice is None
    assert api_fit_stats.mps.one().gas is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(mps=True))
    assert api_drone_stats.mps.one().ice is None
    assert api_drone_stats.mps.one().gas is None


def test_cycle_time_zero(client, consts):
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mining,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 990,
            eve_cycle_time_attr_id: 0,
            eve_waste_chance_attr_id: 60,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().ore is None
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().ore is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(mps=True))
    assert api_drone_stats.mps.one().ore is None


def test_cycle_time_absent(client, consts):
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.mining, cat_id=consts.EveEffCat.target)
    eve_drone_id = client.mk_eve_item(
        attrs={
            eve_yield_attr_id: 990,
            eve_cycle_time_attr_id: 60000,
            eve_waste_chance_attr_id: 60,
            eve_waste_mult_attr_id: 1},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().ore is None
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().ore is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(mps=True))
    assert api_drone_stats.mps.one().ore is None
