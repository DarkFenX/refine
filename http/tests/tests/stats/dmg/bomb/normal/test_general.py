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
from tests.stats.dmg import make_eve_bomb, make_eve_launcher, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_fleet_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_fit_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_charge_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [0, 0, 0, 0]
    assert api_fleet_stats.volley.one() == [0, 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_fit_stats.volley.one() == [0, 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(), StatsOptionItemDps(ignore_state=True)]),
        volley=(True, [StatsOptionItemVolley(), StatsOptionItemVolley(ignore_state=True)])))
    api_charge_dps_normal, api_charge_dps_ignored = api_charge_stats.dps
    assert api_charge_dps_normal == [0, 0, 0, 0]
    assert api_charge_dps_ignored == [approx(93.548387), 0, 0, 0]
    api_charge_volley_normal, api_charge_volley_ignored = api_charge_stats.volley
    assert api_charge_volley_normal == [0, 0, 0, 0]
    assert api_charge_volley_ignored == [approx(7250), 0, 0, 0]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dps=True, volley=True))
    assert api_fleet_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_fleet_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_fit_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(dps=True, volley=True))
    assert api_charge_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_stats.volley.one() == [approx(7250), 0, 0, 0]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75)
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
    assert api_fleet_stats.dps.one() == [approx(280.645161), 0, 0, 0]
    assert api_fleet_stats.volley.one() == [approx(21750), 0, 0, 0]
    api_fit1_stats = api_fit1.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit1_stats.dps.one() == [approx(187.096774), 0, 0, 0]
    assert api_fit1_stats.volley.one() == [approx(14500), 0, 0, 0]
    api_fit2_stats = api_fit2.get_stats(options=FitStatsOptions(dps=True, volley=True))
    assert api_fit2_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_fit2_stats.volley.one() == [approx(7250), 0, 0, 0]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75)
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
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=True, bomb=False)),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, bomb=True))]),
        volley=(True, [
            StatsOptionFitVolley(),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=True, bomb=False)),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, bomb=True))])))
    api_fleet_dps_default, api_fleet_dps_disabled, api_fleet_dps_enabled = api_fleet_stats.dps
    assert api_fleet_dps_default == [approx(93.548387), 0, 0, 0]
    assert api_fleet_dps_disabled == [0, 0, 0, 0]
    assert api_fleet_dps_enabled == [approx(93.548387), 0, 0, 0]
    api_fleet_volley_default, api_fleet_volley_disabled, api_fleet_volley_enabled = api_fleet_stats.volley
    assert api_fleet_volley_default == [approx(7250), 0, 0, 0]
    assert api_fleet_volley_disabled == [0, 0, 0, 0]
    assert api_fleet_volley_enabled == [approx(7250), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [
            StatsOptionFitDps(),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=True, bomb=False)),
            StatsOptionFitDps(item_kinds=StatDmgItemKinds(default=False, bomb=True))]),
        volley=(True, [
            StatsOptionFitVolley(),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=True, bomb=False)),
            StatsOptionFitVolley(item_kinds=StatDmgItemKinds(default=False, bomb=True))])))
    api_fit_dps_default, api_fit_dps_disabled, api_fit_dps_enabled = api_fit_stats.dps
    assert api_fit_dps_default == [approx(93.548387), 0, 0, 0]
    assert api_fit_dps_disabled == [0, 0, 0, 0]
    assert api_fit_dps_enabled == [approx(93.548387), 0, 0, 0]
    api_fit_volley_default, api_fit_volley_disabled, api_fit_volley_enabled = api_fit_stats.volley
    assert api_fit_volley_default == [approx(7250), 0, 0, 0]
    assert api_fit_volley_disabled == [0, 0, 0, 0]
    assert api_fit_volley_enabled == [approx(7250), 0, 0, 0]


def test_include_charges(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
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
        dps=(True, [StatsOptionItemDps(include_charges=False), StatsOptionItemDps(include_charges=True)]),
        volley=(True, [StatsOptionItemVolley(include_charges=False), StatsOptionItemVolley(include_charges=True)])))
    api_module_dps_without, api_module_dps_with = api_module_stats.dps
    assert api_module_dps_without == [0, 0, 0, 0]
    assert api_module_dps_with == [approx(93.548387), 0, 0, 0]
    api_module_volley_without, api_module_volley_with = api_module_stats.volley
    assert api_module_volley_without == [0, 0, 0, 0]
    assert api_module_volley_with == [approx(7250), 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(include_charges=False), StatsOptionItemDps(include_charges=True)]),
        volley=(True, [StatsOptionItemVolley(include_charges=False), StatsOptionItemVolley(include_charges=True)])))
    api_charge_dps_without, api_charge_dps_with = api_charge_stats.dps
    assert api_charge_dps_without == [approx(93.548387), 0, 0, 0]
    assert api_charge_dps_with == [approx(93.548387), 0, 0, 0]
    api_charge_volley_without, api_charge_volley_with = api_charge_stats.volley
    assert api_charge_volley_without == [approx(7250), 0, 0, 0]
    assert api_charge_volley_with == [approx(7250), 0, 0, 0]


def test_time(client, consts):
    # Bomb launchers have reactivation delay which is longer than reload time, so burst/sustained
    # DPS is the same
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification - burst stats (reload is ignored)
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeBurst())]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeBurst())])))
    assert api_fleet_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_fleet_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeBurst())]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeBurst())])))
    assert api_fit_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_fit_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeBurst())]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeBurst())])))
    assert api_charge_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Verification - sim without time means stats with reload time are considered. Since bomb
    # launchers have reactivation delay which is longer than reload time, burst and sustained DPS
    # are the same
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=None))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=None))])))
    assert api_fleet_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_fleet_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=None))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=None))])))
    assert api_fit_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_fit_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=None))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=None))])))
    assert api_charge_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Verification - just after first hit landed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=1))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=1))])))
    assert api_fleet_stats.dps.one() == [approx(7250), 0, 0, 0]
    assert api_fleet_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=1))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=1))])))
    assert api_fit_stats.dps.one() == [approx(7250), 0, 0, 0]
    assert api_fit_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=1))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=1))])))
    assert api_charge_stats.dps.one() == [approx(7250), 0, 0, 0]
    assert api_charge_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Verification - just before second hit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=77))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=77))])))
    assert api_fleet_stats.dps.one() == [approx(94.155844), 0, 0, 0]
    assert api_fleet_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=77))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=77))])))
    assert api_fit_stats.dps.one() == [approx(94.155844), 0, 0, 0]
    assert api_fit_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=77))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=77))])))
    assert api_charge_stats.dps.one() == [approx(94.155844), 0, 0, 0]
    assert api_charge_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Verification - just after second hit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=78))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=78))])))
    assert api_fleet_stats.dps.one() == [approx(185.897436), 0, 0, 0]
    assert api_fleet_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=78))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=78))])))
    assert api_fit_stats.dps.one() == [approx(185.897436), 0, 0, 0]
    assert api_fit_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=78))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=78))])))
    assert api_charge_stats.dps.one() == [approx(185.897436), 0, 0, 0]
    assert api_charge_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Verification - before 5th hit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=309))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=309))])))
    assert api_fleet_stats.dps.one() == [approx(93.851133), 0, 0, 0]
    assert api_fleet_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=309))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=309))])))
    assert api_fit_stats.dps.one() == [approx(93.851133), 0, 0, 0]
    assert api_fit_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=309))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=309))])))
    assert api_charge_stats.dps.one() == [approx(93.851133), 0, 0, 0]
    assert api_charge_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Verification - after 5th hit. Here we check that reload happens during reactivation, not on
    # top of it.
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=311))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=311))])))
    assert api_fleet_stats.dps.one() == [approx(116.559486), 0, 0, 0]
    assert api_fleet_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(time_options=StatTimeSim(time=311))]),
        volley=(True, [StatsOptionFitVolley(time_options=StatTimeSim(time=311))])))
    assert api_fit_stats.dps.one() == [approx(116.559486), 0, 0, 0]
    assert api_fit_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(time_options=StatTimeSim(time=311))]),
        volley=(True, [StatsOptionItemVolley(time_options=StatTimeSim(time=311))])))
    assert api_charge_stats.dps.one() == [approx(116.559486), 0, 0, 0]
    assert api_charge_stats.volley.one() == [approx(7250), 0, 0, 0]
