from tests import approx
from tests.fw.api import (
    FitStatsOptions,
    ItemStatsOptions,
    StatDmgItemKinds,
    StatsOptionFitDps,
    StatsOptionFitVolley,
    StatsOptionItemDps,
    StatsOptionItemVolley,
)
from tests.tests.stats.dmg import make_eve_smartbomb, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_smartbomb(client=client, basic_info=eve_basic_info, dmgs=(45, 45, 45, 45), cycle_time=7500)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(6), approx(6), approx(6), approx(6)]
    assert api_fit_stats.volley.one() == [approx(45), approx(45), approx(45), approx(45)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [approx(6), approx(6), approx(6), approx(6)]
    assert api_module_stats.volley.one() == [approx(45), approx(45), approx(45), approx(45)]
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
    assert api_module_dps_ignored == [approx(6), approx(6), approx(6), approx(6)]
    api_module_volley_normal, api_module_volley_ignored = api_module_stats.volley
    assert api_module_volley_normal == [0, 0, 0, 0]
    assert api_module_volley_ignored == [approx(45), approx(45), approx(45), approx(45)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(6), approx(6), approx(6), approx(6)]
    assert api_fit_stats.volley.one() == [approx(45), approx(45), approx(45), approx(45)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [approx(6), approx(6), approx(6), approx(6)]
    assert api_module_stats.volley.one() == [approx(45), approx(45), approx(45), approx(45)]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_smartbomb(client=client, basic_info=eve_basic_info, dmgs=(45, 45, 45, 45), cycle_time=7500)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(12), approx(12), approx(12), approx(12)]
    assert api_fit_stats.volley.one() == [approx(90), approx(90), approx(90), approx(90)]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_smartbomb(client=client, basic_info=eve_basic_info, dmgs=(45, 45, 45, 45), cycle_time=7500)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [
            StatsOptionFitDps(),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=True, smartbomb=False)),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, smartbomb=True))]),
        volley=(True, [
            StatsOptionFitVolley(),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=True, smartbomb=False)),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, smartbomb=True))])))
    api_fit_dps_default, api_fit_dps_disabled, api_fit_dps_enabled = api_fit_stats.dps
    assert api_fit_dps_default == [approx(6), approx(6), approx(6), approx(6)]
    assert api_fit_dps_disabled == [0, 0, 0, 0]
    assert api_fit_dps_enabled == [approx(6), approx(6), approx(6), approx(6)]
    api_fit_volley_default, api_fit_volley_disabled, api_fit_volley_enabled = api_fit_stats.volley
    assert api_fit_volley_default == [approx(45), approx(45), approx(45), approx(45)]
    assert api_fit_volley_disabled == [0, 0, 0, 0]
    assert api_fit_volley_enabled == [approx(45), approx(45), approx(45), approx(45)]
