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
from tests.stats.dmg import make_eve_bomb, make_eve_launcher, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, disallow_repeating_activation=1, reactivation_delay=67500, reload_time=10000)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification - module is active, charge is not force-disabled, so stats are exposed without
    # extra flags
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=True)]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_module_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_charge_dmg_stats.volley == [approx(7250), 0, 0, 0]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification - module not active, charge is not force-disabled - stats are exposed only when
    # state is ignored
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(include_charges=True),
        StatsOptionItemDmg(include_charges=True, ignore_state=True)])))
    api_module_dmg_normal, api_module_dmg_ignored = api_module_stats.dmg
    assert api_module_dmg_normal.dps == [0, 0, 0, 0]
    assert api_module_dmg_normal.volley == [0, 0, 0, 0]
    assert api_module_dmg_ignored.dps == [approx(93.548387), 0, 0, 0]
    assert api_module_dmg_ignored.volley == [approx(7250), 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_charge_dmg_normal, api_charge_dmg_ignored = api_charge_stats.dmg
    assert api_charge_dmg_normal.dps == [0, 0, 0, 0]
    assert api_charge_dmg_normal.volley == [0, 0, 0, 0]
    assert api_charge_dmg_ignored.dps == [approx(93.548387), 0, 0, 0]
    assert api_charge_dmg_ignored.volley == [approx(7250), 0, 0, 0]
    # Action
    api_module.charge.change_charge(state=False)
    # Verification - module not active, charge is force-disabled - stats are exposed only when state
    # is ignored
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(include_charges=True),
        StatsOptionItemDmg(include_charges=True, ignore_state=True)])))
    api_module_dmg_normal, api_module_dmg_ignored = api_module_stats.dmg
    assert api_module_dmg_normal.dps == [0, 0, 0, 0]
    assert api_module_dmg_normal.volley == [0, 0, 0, 0]
    assert api_module_dmg_ignored.dps == [approx(93.548387), 0, 0, 0]
    assert api_module_dmg_ignored.volley == [approx(7250), 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_charge_dmg_normal, api_charge_dmg_ignored = api_charge_stats.dmg
    assert api_charge_dmg_normal.dps == [0, 0, 0, 0]
    assert api_charge_dmg_normal.volley == [0, 0, 0, 0]
    assert api_charge_dmg_ignored.dps == [approx(93.548387), 0, 0, 0]
    assert api_charge_dmg_ignored.volley == [approx(7250), 0, 0, 0]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification - module active, but charge is force-disabled - stats are exposed only when state
    # is ignored
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(include_charges=True),
        StatsOptionItemDmg(include_charges=True, ignore_state=True)])))
    api_module_dmg_normal, api_module_dmg_ignored = api_module_stats.dmg
    assert api_module_dmg_normal.dps == [0, 0, 0, 0]
    assert api_module_dmg_normal.volley == [0, 0, 0, 0]
    assert api_module_dmg_ignored.dps == [approx(93.548387), 0, 0, 0]
    assert api_module_dmg_ignored.volley == [approx(7250), 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_charge_dmg_normal, api_charge_dmg_ignored = api_charge_stats.dmg
    assert api_charge_dmg_normal.dps == [0, 0, 0, 0]
    assert api_charge_dmg_normal.volley == [0, 0, 0, 0]
    assert api_charge_dmg_ignored.dps == [approx(93.548387), 0, 0, 0]
    assert api_charge_dmg_ignored.volley == [approx(7250), 0, 0, 0]
    # Action
    api_module.charge.change_charge(state=True)
    # Verification - module is active, charge is not force-disabled, so stats are exposed without
    # extra flags
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=True)]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_module_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_charge_dmg_stats.volley == [approx(7250), 0, 0, 0]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, disallow_repeating_activation=1, reactivation_delay=67500, reload_time=10000)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75)
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
    assert api_fleet_dmg_stats.dps == [approx(280.645161), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(21750), 0, 0, 0]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [approx(187.096774), 0, 0, 0]
    assert api_fit1_dmg_stats.volley == [approx(14500), 0, 0, 0]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_fit2_dmg_stats.volley == [approx(7250), 0, 0, 0]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, disallow_repeating_activation=1, reactivation_delay=67500, reload_time=10000)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, bomb=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, bomb=True))])))
    api_fleet_dmg_default, api_fleet_dmg_disabled, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [approx(93.548387), 0, 0, 0]
    assert api_fleet_dmg_default.volley == [approx(7250), 0, 0, 0]
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [approx(93.548387), 0, 0, 0]
    assert api_fleet_dmg_enabled.volley == [approx(7250), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, bomb=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, bomb=True))])))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [approx(93.548387), 0, 0, 0]
    assert api_fit_dmg_default.volley == [approx(7250), 0, 0, 0]
    assert api_fit_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [approx(93.548387), 0, 0, 0]
    assert api_fit_dmg_enabled.volley == [approx(7250), 0, 0, 0]


def test_include_charges(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, disallow_repeating_activation=1, reactivation_delay=67500, reload_time=10000)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    # Verification - need to include charges for module to show dps, since it's on-charge effect
    # which deals damage. For charges, this option doesn't do anything
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=False), StatsOptionItemDmg(include_charges=True)])))
    api_module_dmg_without, api_module_dmg_with = api_module_stats.dmg
    assert api_module_dmg_without.dps == [0, 0, 0, 0]
    assert api_module_dmg_without.volley == [0, 0, 0, 0]
    assert api_module_dmg_with.dps == [approx(93.548387), 0, 0, 0]
    assert api_module_dmg_with.volley == [approx(7250), 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=False), StatsOptionItemDmg(include_charges=True)])))
    api_charge_dmg_without, api_charge_dmg_with = api_charge_stats.dmg
    assert api_charge_dmg_without.dps == [approx(93.548387), 0, 0, 0]
    assert api_charge_dmg_without.volley == [approx(7250), 0, 0, 0]
    assert api_charge_dmg_with.dps == [approx(93.548387), 0, 0, 0]
    assert api_charge_dmg_with.volley == [approx(7250), 0, 0, 0]


def test_time(client, consts):
    # Bomb launchers have reactivation delay which is longer than reload time, so burst/sustained
    # DPS is the same
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, disallow_repeating_activation=1, reactivation_delay=67500, reload_time=10000)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification - burst stats (reload is ignored)
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_charge_dmg_stats.volley == [approx(7250), 0, 0, 0]
    # Verification - sim without time means stats with reload time are considered. Since bomb
    # launchers have reactivation delay which is longer than reload time, burst and sustained DPS
    # are the same
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(93.548387), 0, 0, 0]
    assert api_charge_dmg_stats.volley == [approx(7250), 0, 0, 0]
    # Verification - just after first hit landed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(7250), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(7250), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(7250), 0, 0, 0]
    assert api_charge_dmg_stats.volley == [approx(7250), 0, 0, 0]
    # Verification - just before second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=77))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(94.155844), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=77))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(94.155844), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=77))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(94.155844), 0, 0, 0]
    assert api_charge_dmg_stats.volley == [approx(7250), 0, 0, 0]
    # Verification - just after second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=78))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(185.897436), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=78))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(185.897436), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=78))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(185.897436), 0, 0, 0]
    assert api_charge_dmg_stats.volley == [approx(7250), 0, 0, 0]
    # Verification - before 5th hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=309))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(93.851133), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=309))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(93.851133), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=309))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(93.851133), 0, 0, 0]
    assert api_charge_dmg_stats.volley == [approx(7250), 0, 0, 0]
    # Verification - after 5th hit. Here we check that reload happens during reactivation, not on
    # top of it.
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=311))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(116.559486), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=311))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(116.559486), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(7250), 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=311))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(116.559486), 0, 0, 0]
    assert api_charge_dmg_stats.volley == [approx(7250), 0, 0, 0]
