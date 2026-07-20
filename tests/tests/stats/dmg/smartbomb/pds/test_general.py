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
from tests.stats.dmg import make_eve_charge_normal, make_eve_pds, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_pds(
        client=client, basic_info=eve_basic_info, dmg_mult=1.25, cycle_time=12000, capacity=1000, reload_time=180000)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(250, 250, 250, 250), volume=1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fleet_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_module_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_module_dmg_normal, api_module_dmg_ignored = api_module_stats.dmg
    assert api_module_dmg_normal.dps == [0, 0, 0, 0]
    assert api_module_dmg_normal.volley == [0, 0, 0, 0]
    assert api_module_dmg_ignored.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_module_dmg_ignored.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fleet_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_module_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_pds(
        client=client, basic_info=eve_basic_info, dmg_mult=1.25, cycle_time=12000, capacity=1000, reload_time=180000)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(250, 250, 250, 250), volume=1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fit2 = api_sol.create_fit()
    api_fit2.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(78.125), approx(78.125), approx(78.125), approx(78.125)]
    assert api_fleet_dmg_stats.volley == [approx(937.5), approx(937.5), approx(937.5), approx(937.5)]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [approx(52.083333), approx(52.083333), approx(52.083333), approx(52.083333)]
    assert api_fit1_dmg_stats.volley == [approx(625), approx(625), approx(625), approx(625)]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit2_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_pds(
        client=client, basic_info=eve_basic_info, dmg_mult=1.25, cycle_time=12000, capacity=1000, reload_time=180000)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(250, 250, 250, 250), volume=1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, smartbomb=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, smartbomb=True))])))
    api_fleet_dmg_default, api_fleet_dmg_disabled, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fleet_dmg_default.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fleet_dmg_enabled.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, smartbomb=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, smartbomb=True))])))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit_dmg_default.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    assert api_fit_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit_dmg_enabled.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]


def test_time(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_pds(
        client=client, basic_info=eve_basic_info, dmg_mult=1.25, cycle_time=12000, capacity=1000, reload_time=180000)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(250, 250, 250, 250), volume=1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification - burst stats (reload is ignored)
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fleet_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_module_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Verification - sim without time means stats with reload time are considered
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(6.510417), approx(6.510417), approx(6.510417), approx(6.510417)]
    assert api_fleet_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(6.510417), approx(6.510417), approx(6.510417), approx(6.510417)]
    assert api_fit_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(6.510417), approx(6.510417), approx(6.510417), approx(6.510417)]
    assert api_module_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Verification - just after first hit landed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    assert api_fleet_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    assert api_fit_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    assert api_module_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Verification - just before second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=11))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(28.409091), approx(28.409091), approx(28.409091), approx(28.409091)]
    assert api_fleet_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=11))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(28.409091), approx(28.409091), approx(28.409091), approx(28.409091)]
    assert api_fit_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=11))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(28.409091), approx(28.409091), approx(28.409091), approx(28.409091)]
    assert api_module_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Verification - just after second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=13))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(48.076923), approx(48.076923), approx(48.076923), approx(48.076923)]
    assert api_fleet_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=13))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(48.076923), approx(48.076923), approx(48.076923), approx(48.076923)]
    assert api_fit_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=13))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(48.076923), approx(48.076923), approx(48.076923), approx(48.076923)]
    assert api_module_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Verification - just before reload starts
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=59))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(26.483051), approx(26.483051), approx(26.483051), approx(26.483051)]
    assert api_fleet_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=59))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(26.483051), approx(26.483051), approx(26.483051), approx(26.483051)]
    assert api_fit_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=59))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(26.483051), approx(26.483051), approx(26.483051), approx(26.483051)]
    assert api_module_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Verification - just before reload completes
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=239))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(6.537657), approx(6.537657), approx(6.537657), approx(6.537657)]
    assert api_fleet_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=239))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(6.537657), approx(6.537657), approx(6.537657), approx(6.537657)]
    assert api_fit_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=239))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(6.537657), approx(6.537657), approx(6.537657), approx(6.537657)]
    assert api_module_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Verification - after reload is done and another bomb hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=195))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(8.012821), approx(8.012821), approx(8.012821), approx(8.012821)]
    assert api_fleet_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=195))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(8.012821), approx(8.012821), approx(8.012821), approx(8.012821)]
    assert api_fit_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=195))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(8.012821), approx(8.012821), approx(8.012821), approx(8.012821)]
    assert api_module_dmg_stats.volley == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]


def test_partial_chargedness(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module1_id = make_eve_pds(
        client=client, basic_info=eve_basic_info, dmg_mult=1.25, cycle_time=12000, capacity=1000, reload_time=180000)
    eve_module2_id = make_eve_pds(
        client=client, basic_info=eve_basic_info, dmg_mult=1.25, cycle_time=12000, capacity=999, reload_time=180000)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(250, 250, 250, 250), volume=1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module1_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None))])))
    api_fleet_dmg_burst, api_fleet_dmg_reload = api_fleet_stats.dmg
    assert api_fleet_dmg_burst.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fleet_dmg_reload.dps == [approx(6.510417), approx(6.510417), approx(6.510417), approx(6.510417)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None))])))
    api_fit_dmg_burst, api_fit_dmg_reload = api_fit_stats.dmg
    assert api_fit_dmg_burst.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit_dmg_reload.dps == [approx(6.510417), approx(6.510417), approx(6.510417), approx(6.510417)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(time_options=StatTimeBurst()),
        StatsOptionItemDmg(time_options=StatTimeSim(time=None))])))
    api_module_dmg_burst, api_module_dmg_reload = api_module_stats.dmg
    assert api_module_dmg_burst.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_module_dmg_reload.dps == [approx(6.510417), approx(6.510417), approx(6.510417), approx(6.510417)]
    # Action
    api_module.change_module(type_id=eve_module2_id)
    # Verification - PDS refuses to cycle when count of loaded charges is lower than it needs.
    # Tested on Tranquility on 2025-12-19
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None))])))
    api_fleet_dmg_burst, api_fleet_dmg_reload = api_fleet_stats.dmg
    assert api_fleet_dmg_burst.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fleet_dmg_reload.dps == [approx(5.482456), approx(5.482456), approx(5.482456), approx(5.482456)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None))])))
    api_fit_dmg_burst, api_fit_dmg_reload = api_fit_stats.dmg
    assert api_fit_dmg_burst.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit_dmg_reload.dps == [approx(5.482456), approx(5.482456), approx(5.482456), approx(5.482456)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(time_options=StatTimeBurst()),
        StatsOptionItemDmg(time_options=StatTimeSim(time=None))])))
    api_module_dmg_burst, api_module_dmg_reload = api_module_stats.dmg
    assert api_module_dmg_burst.dps == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_module_dmg_reload.dps == [approx(5.482456), approx(5.482456), approx(5.482456), approx(5.482456)]
