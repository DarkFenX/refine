from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionFitDps, StatsOptionItemDps, StatsOptionItemVolley
from tests.tests.stats.dmg import make_eve_turret_charge_normal, make_eve_vorton, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_vorton(
        client=client, basic_info=eve_basic_info, dmg_mult=2.62, capacity=25, cycle_time=6000, reload_time=5000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(402, 0, 379, 0), volume=0.0125)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_fit_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_module_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
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
    assert api_module_dps_ignored == [approx(175.54), 0, approx(165.496667), 0]
    api_module_volley_normal, api_module_volley_ignored = api_module_stats.volley
    assert api_module_volley_normal == [0, 0, 0, 0]
    assert api_module_volley_ignored == [approx(1053.24), 0, approx(992.98), 0]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_fit_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [approx(175.54), 0, approx(165.496667), 0]
    assert api_module_stats.volley.one() == [approx(1053.24), 0, approx(992.98), 0]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_vorton(
        client=client, basic_info=eve_basic_info, dmg_mult=2.62, capacity=25, cycle_time=6000, reload_time=5000)
    eve_charge1_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(402, 0, 379, 0), volume=0.0125)
    eve_charge2_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(218, 0, 212, 0), volume=0.0125)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge1_id)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge2_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(270.733333), 0, approx(258.07), 0]
    assert api_fit_stats.volley.one() == [approx(1624.4), 0, approx(1548.42), 0]


def test_reload(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_vorton(
        client=client, basic_info=eve_basic_info, dmg_mult=2.62, capacity=25, cycle_time=6000, reload_time=5000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(402, 0, 379, 0), volume=0.0125)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(), StatsOptionFitDps(reload=True)])))
    api_fit_dps_burst, api_fit_dps_reload = api_fit_stats.dps
    assert api_fit_dps_burst == [approx(175.54), 0, approx(165.496667), 0]
    assert api_fit_dps_reload == [approx(175.466889), 0, approx(165.427738), 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(), StatsOptionItemDps(reload=True)])))
    api_module_dps_burst, api_module_dps_reload = api_module_stats.dps
    assert api_module_dps_burst == [approx(175.54), 0, approx(165.496667), 0]
    assert api_module_dps_reload == [approx(175.466889), 0, approx(165.427738), 0]


def test_charge_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_vorton(
        client=client, basic_info=eve_basic_info, dmg_mult=2.62, capacity=25, cycle_time=6000, reload_time=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_stats.volley.one() == [0, 0, 0, 0]


def test_charge_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_vorton(
        client=client, basic_info=eve_basic_info, dmg_mult=2.62, capacity=25, cycle_time=6000, reload_time=5000)
    eve_charge_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_stats.volley.one() == [0, 0, 0, 0]
