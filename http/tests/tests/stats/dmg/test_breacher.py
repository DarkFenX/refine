from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionItemDps, StatsOptionItemVolley
from tests.tests.stats.dmg import make_eve_breacher, make_eve_launcher, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=25, cycle_time=10000, reload_time=30000)
    eve_charge_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=1000, dmg_rel=1, dmg_duration=75000, volume=0.5)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one().breacher == [approx(1000), approx(0.01)]
    assert api_fit_stats.volley.one().breacher == [approx(1000), approx(0.01)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_charge_stats.dps.one().breacher == [approx(1000), approx(0.01)]
    assert api_charge_stats.volley.one().breacher == [approx(1000), approx(0.01)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one().breacher is None
    assert api_fit_stats.volley.one().breacher is None
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(), StatsOptionItemDps(ignore_state=True)]),
        volley=(True, [StatsOptionItemVolley(), StatsOptionItemVolley(ignore_state=True)])))
    api_charge_dps_normal, api_charge_dps_ignored = api_charge_stats.dps
    assert api_charge_dps_normal.breacher is None
    assert api_charge_dps_ignored.breacher == [approx(1000), approx(0.01)]
    api_charge_volley_normal, api_charge_volley_ignored = api_charge_stats.volley
    assert api_charge_volley_normal.breacher is None
    assert api_charge_volley_ignored.breacher == [approx(1000), approx(0.01)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one().breacher == [approx(1000), approx(0.01)]
    assert api_fit_stats.volley.one().breacher == [approx(1000), approx(0.01)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_charge_stats.dps.one().breacher == [approx(1000), approx(0.01)]
    assert api_charge_stats.volley.one().breacher == [approx(1000), approx(0.01)]
