from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionItemDps, StatsOptionItemVolley
from tests.tests.stats.dmg import make_eve_turret_proj, make_eve_turret_proj_charge, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=45, capacity=0.25, cycle_time=8000, reload_time=10000)
    eve_charge_id = make_eve_turret_proj_charge(
        client=client, basic_info=eve_basic_info, dmgs=(0, 23, 4.6, 0), volume=0.0125)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, approx(129.375), approx(25.875), 0]
    assert api_fit_stats.volley.one() == [0, approx(1035), approx(207), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [0, approx(129.375), approx(25.875), 0]
    assert api_module_stats.volley.one() == [0, approx(1035), approx(207), 0]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(), StatsOptionItemDps(ignore_state=True)]),
        volley=(True, [StatsOptionItemVolley(), StatsOptionItemVolley(ignore_state=True)])))
    api_module_dps_normal, api_module_dps_ignored = api_module_stats.dps
    assert api_module_dps_normal == [0, 0, 0, 0]
    assert api_module_dps_ignored == [0, approx(129.375), approx(25.875), 0]
    api_module_volley_normal, api_module_volley_ignored = api_module_stats.volley
    assert api_module_volley_normal == [0, 0, 0, 0]
    assert api_module_volley_ignored == [0, approx(1035), approx(207), 0]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, approx(129.375), approx(25.875), 0]
    assert api_fit_stats.volley.one() == [0, approx(1035), approx(207), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [0, approx(129.375), approx(25.875), 0]
    assert api_module_stats.volley.one() == [0, approx(1035), approx(207), 0]
