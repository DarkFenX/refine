from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionItemDps, StatsOptionItemVolley
from tests.tests.stats.dmg import make_eve_drone, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, dmgs=(0, 13, 19, 0), dmg_mult=41, cycle_time=4000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_module_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, approx(133.25), approx(194.75), 0]
    assert api_fit_stats.volley.one() == [0, approx(533), approx(779), 0]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_drone_stats.dps.one() == [0, approx(133.25), approx(194.75), 0]
    assert api_drone_stats.volley.one() == [0, approx(533), approx(779), 0]
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(), StatsOptionItemDps(ignore_state=True)]),
        volley=(True, [StatsOptionItemVolley(), StatsOptionItemVolley(ignore_state=True)])))
    api_drone_dps_normal, api_drone_dps_ignored = api_drone_stats.dps
    assert api_drone_dps_normal == [0, 0, 0, 0]
    assert api_drone_dps_ignored == [0, approx(133.25), approx(194.75), 0]
    api_drone_volley_normal, api_drone_volley_ignored = api_drone_stats.volley
    assert api_drone_volley_normal == [0, 0, 0, 0]
    assert api_drone_volley_ignored == [0, approx(533), approx(779), 0]
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, approx(133.25), approx(194.75), 0]
    assert api_fit_stats.volley.one() == [0, approx(533), approx(779), 0]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_drone_stats.dps.one() == [0, approx(133.25), approx(194.75), 0]
    assert api_drone_stats.volley.one() == [0, approx(533), approx(779), 0]
