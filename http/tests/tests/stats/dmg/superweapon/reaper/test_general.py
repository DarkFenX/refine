from fw import approx
from fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatDmgItemKinds,
    StatsOptionFitDmg,
    StatsOptionItemDmg,
    StatTimeBurst,
    StatTimeSim,
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
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(859.375)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(859.375)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, 0, approx(859.375)]
    assert api_module_dmg_stats.volley == [0, 0, 0, approx(206250)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_module_dmg_normal, api_module_dmg_ignored = api_module_stats.dmg
    assert api_module_dmg_normal.dps == [0, 0, 0, 0]
    assert api_module_dmg_normal.volley == [0, 0, 0, 0]
    assert api_module_dmg_ignored.dps == [0, 0, 0, approx(859.375)]
    assert api_module_dmg_ignored.volley == [0, 0, 0, approx(206250)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(859.375)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, 0, approx(859.375)]
    assert api_module_dmg_stats.volley == [0, 0, 0, approx(206250)]


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
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(2578.125)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(618750)]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [0, 0, 0, approx(1718.75)]
    assert api_fit1_dmg_stats.volley == [0, 0, 0, approx(412500)]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [0, 0, 0, approx(859.375)]
    assert api_fit2_dmg_stats.volley == [0, 0, 0, approx(206250)]


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
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, superweapon=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, superweapon=True))])))
    api_fleet_dmg_default, api_fleet_dmg_disabled, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [0, 0, 0, approx(859.375)]
    assert api_fleet_dmg_default.volley == [0, 0, 0, approx(206250)]
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [0, 0, 0, approx(859.375)]
    assert api_fleet_dmg_enabled.volley == [0, 0, 0, approx(206250)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, superweapon=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, superweapon=True))])))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [0, 0, 0, approx(859.375)]
    assert api_fit_dmg_default.volley == [0, 0, 0, approx(206250)]
    assert api_fit_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [0, 0, 0, approx(859.375)]
    assert api_fit_dmg_enabled.volley == [0, 0, 0, approx(206250)]


def test_time(client, consts):
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
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification - burst stats
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(859.375)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(859.375)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, 0, approx(859.375)]
    assert api_module_dmg_stats.volley == [0, 0, 0, approx(206250)]
    # Verification - sim without time
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(859.375)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(859.375)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, 0, approx(859.375)]
    assert api_module_dmg_stats.volley == [0, 0, 0, approx(206250)]
    # Verification - sim with time before damage hits
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=9.9))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=9.9))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=9.9))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_dmg_stats.volley == [0, 0, 0, 0]
    # Verification - sim with time after first hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=10.1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(20420.792079)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=10.1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(20420.792079)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=10.1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, 0, approx(20420.792079)]
    assert api_module_dmg_stats.volley == [0, 0, 0, approx(206250)]
    # Verification - sim with time to check that there are no other damage instances this cycle
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=20.1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(10261.19403)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=20.1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(10261.19403)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=20.1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, 0, approx(10261.19403)]
    assert api_module_dmg_stats.volley == [0, 0, 0, approx(206250)]
    # Verification - sim with time before damage of second cycle
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=249.9))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(825.330132)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=249.9))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(825.330132)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=249.9))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, 0, approx(825.330132)]
    assert api_module_dmg_stats.volley == [0, 0, 0, approx(206250)]
    # Verification - sim with time after damage of second cycle
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=250.1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(1649.340264)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=250.1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(1649.340264)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(206250)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=250.1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, 0, approx(1649.340264)]
    assert api_module_dmg_stats.volley == [0, 0, 0, approx(206250)]
