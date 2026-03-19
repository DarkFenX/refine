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
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fleet_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_module_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [0, 0, 0, 0]
    assert api_fleet_stats.volley.one() == [0, 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(), StatsOptionItemDps(ignore_state=True)]),
        volley=(True, [StatsOptionItemVolley(), StatsOptionItemVolley(ignore_state=True)])))
    api_module_dps_normal, api_module_dps_ignored = api_module_stats.dps
    assert api_module_dps_normal == [0, 0, 0, 0]
    assert api_module_dps_ignored == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    api_module_volley_normal, api_module_volley_ignored = api_module_stats.volley
    assert api_module_volley_normal == [0, 0, 0, 0]
    assert api_module_volley_ignored == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fleet_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_module_stats.dps.one() == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_module_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]


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
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [approx(78.125), approx(78.125), approx(78.125), approx(78.125)]
    assert api_fleet_stats.volley.one() == [approx(937.5), approx(937.5), approx(937.5), approx(937.5)]
    api_fit1_stats = api_fit1.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit1_stats.dps.one() == [approx(52.083333), approx(52.083333), approx(52.083333), approx(52.083333)]
    assert api_fit1_stats.volley.one() == [approx(625), approx(625), approx(625), approx(625)]
    api_fit2_stats = api_fit2.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit2_stats.dps.one() == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit2_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]


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
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [
            StatsOptionFitDps(),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=True, smartbomb=False)),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, smartbomb=True))]),
        volley=(True, [
            StatsOptionFitVolley(),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=True, smartbomb=False)),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, smartbomb=True))])))
    api_fleet_dps_default, api_fleet_dps_disabled, api_fleet_dps_enabled = api_fleet_stats.dps
    assert api_fleet_dps_default == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fleet_dps_disabled == [0, 0, 0, 0]
    assert api_fleet_dps_enabled == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    api_fleet_volley_default, api_fleet_volley_disabled, api_fleet_volley_enabled = api_fleet_stats.volley
    assert api_fleet_volley_default == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    assert api_fleet_volley_disabled == [0, 0, 0, 0]
    assert api_fleet_volley_enabled == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
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
    assert api_fit_dps_default == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit_dps_disabled == [0, 0, 0, 0]
    assert api_fit_dps_enabled == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    api_fit_volley_default, api_fit_volley_disabled, api_fit_volley_enabled = api_fit_stats.volley
    assert api_fit_volley_default == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    assert api_fit_volley_disabled == [0, 0, 0, 0]
    assert api_fit_volley_enabled == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]


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
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification - burst stats (reload is ignored)
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeBurst())]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeBurst())])))
    assert api_fleet_stats.dps.one() == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fleet_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeBurst())]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeBurst())])))
    assert api_fit_stats.dps.one() == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeBurst())]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeBurst())])))
    assert api_module_stats.dps.one() == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_module_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Verification - sim without time means stats with reload time are considered
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=None))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=None))])))
    assert api_fleet_stats.dps.one() == [approx(6.510417), approx(6.510417), approx(6.510417), approx(6.510417)]
    assert api_fleet_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=None))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=None))])))
    assert api_fit_stats.dps.one() == [approx(6.510417), approx(6.510417), approx(6.510417), approx(6.510417)]
    assert api_fit_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=None))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=None))])))
    assert api_module_stats.dps.one() == [approx(6.510417), approx(6.510417), approx(6.510417), approx(6.510417)]
    assert api_module_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Verification - just after first hit landed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=1))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=1))])))
    assert api_fleet_stats.dps.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    assert api_fleet_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=1))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=1))])))
    assert api_fit_stats.dps.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    assert api_fit_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=1))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=1))])))
    assert api_module_stats.dps.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    assert api_module_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Verification - just before second hit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=11))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=11))])))
    assert api_fleet_stats.dps.one() == [approx(28.409091), approx(28.409091), approx(28.409091), approx(28.409091)]
    assert api_fleet_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=11))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=11))])))
    assert api_fit_stats.dps.one() == [approx(28.409091), approx(28.409091), approx(28.409091), approx(28.409091)]
    assert api_fit_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=11))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=11))])))
    assert api_module_stats.dps.one() == [approx(28.409091), approx(28.409091), approx(28.409091), approx(28.409091)]
    assert api_module_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Verification - just after second hit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=13))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=13))])))
    assert api_fleet_stats.dps.one() == [approx(48.076923), approx(48.076923), approx(48.076923), approx(48.076923)]
    assert api_fleet_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=13))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=13))])))
    assert api_fit_stats.dps.one() == [approx(48.076923), approx(48.076923), approx(48.076923), approx(48.076923)]
    assert api_fit_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=13))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=13))])))
    assert api_module_stats.dps.one() == [approx(48.076923), approx(48.076923), approx(48.076923), approx(48.076923)]
    assert api_module_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Verification - just before reload starts
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=59))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=59))])))
    assert api_fleet_stats.dps.one() == [approx(26.483051), approx(26.483051), approx(26.483051), approx(26.483051)]
    assert api_fleet_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=59))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=59))])))
    assert api_fit_stats.dps.one() == [approx(26.483051), approx(26.483051), approx(26.483051), approx(26.483051)]
    assert api_fit_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=59))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=59))])))
    assert api_module_stats.dps.one() == [approx(26.483051), approx(26.483051), approx(26.483051), approx(26.483051)]
    assert api_module_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Verification - just before reload completes
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=239))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=239))])))
    assert api_fleet_stats.dps.one() == [approx(6.537657), approx(6.537657), approx(6.537657), approx(6.537657)]
    assert api_fleet_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=239))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=239))])))
    assert api_fit_stats.dps.one() == [approx(6.537657), approx(6.537657), approx(6.537657), approx(6.537657)]
    assert api_fit_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=239))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=239))])))
    assert api_module_stats.dps.one() == [approx(6.537657), approx(6.537657), approx(6.537657), approx(6.537657)]
    assert api_module_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    # Verification - after reload is done and another bomb hit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=195))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=195))])))
    assert api_fleet_stats.dps.one() == [approx(8.012821), approx(8.012821), approx(8.012821), approx(8.012821)]
    assert api_fleet_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=195))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=195))])))
    assert api_fit_stats.dps.one() == [approx(8.012821), approx(8.012821), approx(8.012821), approx(8.012821)]
    assert api_fit_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=195))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=195))])))
    assert api_module_stats.dps.one() == [approx(8.012821), approx(8.012821), approx(8.012821), approx(8.012821)]
    assert api_module_stats.volley.one() == [approx(312.5), approx(312.5), approx(312.5), approx(312.5)]


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
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=(True, [
        StatsOptionFitDps(time_options=StatTimeBurst()),
        StatsOptionFitDps(time_options=StatTimeSim(time=None))])))
    api_fleet_dps_burst, api_fleet_dps_reload = api_fleet_stats.dps
    assert api_fleet_dps_burst == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fleet_dps_reload == [approx(6.510417), approx(6.510417), approx(6.510417), approx(6.510417)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=(True, [
        StatsOptionFitDps(time_options=StatTimeBurst()),
        StatsOptionFitDps(time_options=StatTimeSim(time=None))])))
    api_fit_dps_burst, api_fit_dps_reload = api_fit_stats.dps
    assert api_fit_dps_burst == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit_dps_reload == [approx(6.510417), approx(6.510417), approx(6.510417), approx(6.510417)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=(True, [
        StatsOptionItemDps(time_options=StatTimeBurst()),
        StatsOptionItemDps(time_options=StatTimeSim(time=None))])))
    api_module_dps_burst, api_module_dps_reload = api_module_stats.dps
    assert api_module_dps_burst == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_module_dps_reload == [approx(6.510417), approx(6.510417), approx(6.510417), approx(6.510417)]
    # Action
    api_module.change_module(type_id=eve_module2_id)
    # Verification - PDS refuses to cycle when count of loaded charges is lower than it needs.
    # Tested on Tranquility on 2025-12-19
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=(True, [
        StatsOptionFitDps(time_options=StatTimeBurst()),
        StatsOptionFitDps(time_options=StatTimeSim(time=None))])))
    api_fleet_dps_burst, api_fleet_dps_reload = api_fleet_stats.dps
    assert api_fleet_dps_burst == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fleet_dps_reload == [approx(5.482456), approx(5.482456), approx(5.482456), approx(5.482456)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=(True, [
        StatsOptionFitDps(time_options=StatTimeBurst()),
        StatsOptionFitDps(time_options=StatTimeSim(time=None))])))
    api_fit_dps_burst, api_fit_dps_reload = api_fit_stats.dps
    assert api_fit_dps_burst == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_fit_dps_reload == [approx(5.482456), approx(5.482456), approx(5.482456), approx(5.482456)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dps=(True, [
        StatsOptionItemDps(time_options=StatTimeBurst()),
        StatsOptionItemDps(time_options=StatTimeSim(time=None))])))
    api_module_dps_burst, api_module_dps_reload = api_module_stats.dps
    assert api_module_dps_burst == [approx(26.041667), approx(26.041667), approx(26.041667), approx(26.041667)]
    assert api_module_dps_reload == [approx(5.482456), approx(5.482456), approx(5.482456), approx(5.482456)]
