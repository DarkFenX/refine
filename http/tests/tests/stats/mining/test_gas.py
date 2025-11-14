from tests import approx
from tests.fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatsOptionItemMining,
)


def test_state(client, consts):
    eve_yield_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_amount)
    eve_waste_chance_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_waste_probability)
    eve_waste_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mining_wasted_volume_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.mining_clouds,
        cat_id=consts.EveEffCat.active,
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
    assert api_fleet_stats.mps.one().gas == [approx(1.333333), approx(0.4533333)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().gas == [approx(1.333333), approx(0.4533333)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(mps=True))
    assert api_module_stats.mps.one().gas == [approx(1.333333), approx(0.4533333)]
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
    assert api_module_mps_ignored.gas == [approx(1.333333), approx(0.4533333)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(mps=True))
    assert api_fleet_stats.mps.one().gas == [approx(1.333333), approx(0.4533333)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(mps=True))
    assert api_fit_stats.mps.one().gas == [approx(1.333333), approx(0.4533333)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(mps=True))
    assert api_module_stats.mps.one().gas == [approx(1.333333), approx(0.4533333)]
