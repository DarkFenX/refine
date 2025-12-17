from fw import approx
from fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatDmgItemKinds,
    StatsOptionFitDps,
    StatsOptionFitVolley,
    StatsOptionItemDps,
    StatsOptionItemVolley,
)
from tests.stats.dmg import make_eve_dd_reaper, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_reaper(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(0, 0, 0, 206250),
        cycle_time=240000,
        delay=10000,
        dmg_interval=1000,
        dmg_duration=10000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [0, 0, 0, approx(859.375)]
    assert api_fleet_stats.volley.one() == [0, 0, 0, approx(206250)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, 0, 0, approx(859.375)]
    assert api_fit_stats.volley.one() == [0, 0, 0, approx(206250)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [0, 0, 0, approx(859.375)]
    assert api_module_stats.volley.one() == [0, 0, 0, approx(206250)]
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
    assert api_module_dps_ignored == [0, 0, 0, approx(859.375)]
    api_module_volley_normal, api_module_volley_ignored = api_module_stats.volley
    assert api_module_volley_normal == [0, 0, 0, 0]
    assert api_module_volley_ignored == [0, 0, 0, approx(206250)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, 0, 0, approx(859.375)]
    assert api_fit_stats.volley.one() == [0, 0, 0, approx(206250)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [0, 0, 0, approx(859.375)]
    assert api_module_stats.volley.one() == [0, 0, 0, approx(206250)]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_reaper(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(0, 0, 0, 206250),
        cycle_time=240000,
        delay=10000,
        dmg_interval=1000,
        dmg_duration=10000)
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
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [0, 0, 0, approx(2578.125)]
    assert api_fleet_stats.volley.one() == [0, 0, 0, approx(618750)]
    api_fit1_stats = api_fit1.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit1_stats.dps.one() == [0, 0, 0, approx(1718.75)]
    assert api_fit1_stats.volley.one() == [0, 0, 0, approx(412500)]
    api_fit2_stats = api_fit2.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit2_stats.dps.one() == [0, 0, 0, approx(859.375)]
    assert api_fit2_stats.volley.one() == [0, 0, 0, approx(206250)]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_dd_reaper(
        client=client,
        basic_info=eve_basic_info,
        dmgs=(0, 0, 0, 206250),
        cycle_time=240000,
        delay=10000,
        dmg_interval=1000,
        dmg_duration=10000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [
            StatsOptionFitDps(),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=True, superweapon=False)),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, superweapon=True))]),
        volley=(True, [
            StatsOptionFitVolley(),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=True, superweapon=False)),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, superweapon=True))])))
    api_fleet_dps_default, api_fleet_dps_disabled, api_fleet_dps_enabled = api_fleet_stats.dps
    assert api_fleet_dps_default == [0, 0, 0, approx(859.375)]
    assert api_fleet_dps_disabled == [0, 0, 0, 0]
    assert api_fleet_dps_enabled == [0, 0, 0, approx(859.375)]
    api_fleet_volley_default, api_fleet_volley_disabled, api_fleet_volley_enabled = api_fleet_stats.volley
    assert api_fleet_volley_default == [0, 0, 0, approx(206250)]
    assert api_fleet_volley_disabled == [0, 0, 0, 0]
    assert api_fleet_volley_enabled == [0, 0, 0, approx(206250)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [
            StatsOptionFitDps(),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=True, superweapon=False)),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, superweapon=True))]),
        volley=(True, [
            StatsOptionFitVolley(),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=True, superweapon=False)),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, superweapon=True))])))
    api_fit_dps_default, api_fit_dps_disabled, api_fit_dps_enabled = api_fit_stats.dps
    assert api_fit_dps_default == [0, 0, 0, approx(859.375)]
    assert api_fit_dps_disabled == [0, 0, 0, 0]
    assert api_fit_dps_enabled == [0, 0, 0, approx(859.375)]
    api_fit_volley_default, api_fit_volley_disabled, api_fit_volley_enabled = api_fit_stats.volley
    assert api_fit_volley_default == [0, 0, 0, approx(206250)]
    assert api_fit_volley_disabled == [0, 0, 0, 0]
    assert api_fit_volley_enabled == [0, 0, 0, approx(206250)]
