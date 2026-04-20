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
from tests.stats.dmg import make_eve_breacher, make_eve_launcher, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=25, cycle_time=10000, reload_time=30000)
    eve_charge_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=1000, dmg_rel=1, dmg_duration=75000, volume=0.5)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification - module is active, charge is not force-disabled, so stats are exposed without
    # extra flags
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps.breacher == [approx(1000), approx(0.01)]
    assert api_fleet_dmg_stats.volley.breacher == [approx(1000), approx(0.01)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps.breacher == [approx(1000), approx(0.01)]
    assert api_fit_dmg_stats.volley.breacher == [approx(1000), approx(0.01)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=True)]))).dmg.one()
    assert api_module_dmg_stats.dps.breacher == [approx(1000), approx(0.01)]
    assert api_module_dmg_stats.volley.breacher == [approx(1000), approx(0.01)]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_charge_dmg_stats.dps.breacher == [approx(1000), approx(0.01)]
    assert api_charge_dmg_stats.volley.breacher == [approx(1000), approx(0.01)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification - module not active, charge is not force-disabled - stats are exposed only when
    # state is ignored
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps.breacher == [0, 0]
    assert api_fleet_dmg_stats.volley.breacher == [0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps.breacher == [0, 0]
    assert api_fit_dmg_stats.volley.breacher == [0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(include_charges=True),
        StatsOptionItemDmg(include_charges=True, ignore_state=True)])))
    api_module_dmg_normal, api_module_dmg_ignored = api_module_stats.dmg
    assert api_module_dmg_normal.dps.breacher == [0, 0]
    assert api_module_dmg_normal.volley.breacher == [0, 0]
    assert api_module_dmg_ignored.dps.breacher == [approx(1000), approx(0.01)]
    assert api_module_dmg_ignored.volley.breacher == [approx(1000), approx(0.01)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_charge_dmg_normal, api_charge_dmg_ignored = api_charge_stats.dmg
    assert api_charge_dmg_normal.dps.breacher == [0, 0]
    assert api_charge_dmg_normal.volley.breacher == [0, 0]
    assert api_charge_dmg_ignored.dps.breacher == [approx(1000), approx(0.01)]
    assert api_charge_dmg_ignored.volley.breacher == [approx(1000), approx(0.01)]
    # Action
    api_module.charge.change_charge(state=False)
    # Verification - module not active, charge is force-disabled - stats are exposed only when state
    # is ignored
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps.breacher == [0, 0]
    assert api_fleet_dmg_stats.volley.breacher == [0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps.breacher == [0, 0]
    assert api_fit_dmg_stats.volley.breacher == [0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(include_charges=True),
        StatsOptionItemDmg(include_charges=True, ignore_state=True)])))
    api_module_dmg_normal, api_module_dmg_ignored = api_module_stats.dmg
    assert api_module_dmg_normal.dps.breacher == [0, 0]
    assert api_module_dmg_normal.volley.breacher == [0, 0]
    assert api_module_dmg_ignored.dps.breacher == [approx(1000), approx(0.01)]
    assert api_module_dmg_ignored.volley.breacher == [approx(1000), approx(0.01)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_charge_dmg_normal, api_charge_dmg_ignored = api_charge_stats.dmg
    assert api_charge_dmg_normal.dps.breacher == [0, 0]
    assert api_charge_dmg_normal.volley.breacher == [0, 0]
    assert api_charge_dmg_ignored.dps.breacher == [approx(1000), approx(0.01)]
    assert api_charge_dmg_ignored.volley.breacher == [approx(1000), approx(0.01)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification - module active, but charge is force-disabled - stats are exposed only when state
    # is ignored
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps.breacher == [0, 0]
    assert api_fleet_dmg_stats.volley.breacher == [0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps.breacher == [0, 0]
    assert api_fit_dmg_stats.volley.breacher == [0, 0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(include_charges=True),
        StatsOptionItemDmg(include_charges=True, ignore_state=True)])))
    api_module_dmg_normal, api_module_dmg_ignored = api_module_stats.dmg
    assert api_module_dmg_normal.dps.breacher == [0, 0]
    assert api_module_dmg_normal.volley.breacher == [0, 0]
    assert api_module_dmg_ignored.dps.breacher == [approx(1000), approx(0.01)]
    assert api_module_dmg_ignored.volley.breacher == [approx(1000), approx(0.01)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_charge_dmg_normal, api_charge_dmg_ignored = api_charge_stats.dmg
    assert api_charge_dmg_normal.dps.breacher == [0, 0]
    assert api_charge_dmg_normal.volley.breacher == [0, 0]
    assert api_charge_dmg_ignored.dps.breacher == [approx(1000), approx(0.01)]
    assert api_charge_dmg_ignored.volley.breacher == [approx(1000), approx(0.01)]
    # Action
    api_module.charge.change_charge(state=True)
    # Verification - module is active, charge is not force-disabled, so stats are exposed without
    # extra flags
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps.breacher == [approx(1000), approx(0.01)]
    assert api_fleet_dmg_stats.volley.breacher == [approx(1000), approx(0.01)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps.breacher == [approx(1000), approx(0.01)]
    assert api_fit_dmg_stats.volley.breacher == [approx(1000), approx(0.01)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=True)]))).dmg.one()
    assert api_module_dmg_stats.dps.breacher == [approx(1000), approx(0.01)]
    assert api_module_dmg_stats.volley.breacher == [approx(1000), approx(0.01)]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_charge_dmg_stats.dps.breacher == [approx(1000), approx(0.01)]
    assert api_charge_dmg_stats.volley.breacher == [approx(1000), approx(0.01)]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=25, cycle_time=10000, reload_time=30000)
    eve_charge_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=1000, dmg_rel=1, dmg_duration=75000, volume=0.5)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, breacher=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, breacher=True))])))
    api_fleet_dmg_default, api_fleet_dmg_disabled, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps.breacher == [approx(1000), approx(0.01)]
    assert api_fleet_dmg_default.volley.breacher == [approx(1000), approx(0.01)]
    assert api_fleet_dmg_disabled.dps.breacher == [0, 0]
    assert api_fleet_dmg_disabled.volley.breacher == [0, 0]
    assert api_fleet_dmg_enabled.dps.breacher == [approx(1000), approx(0.01)]
    assert api_fleet_dmg_enabled.volley.breacher == [approx(1000), approx(0.01)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, breacher=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, breacher=True))])))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps.breacher == [approx(1000), approx(0.01)]
    assert api_fit_dmg_default.volley.breacher == [approx(1000), approx(0.01)]
    assert api_fit_dmg_disabled.dps.breacher == [0, 0]
    assert api_fit_dmg_disabled.volley.breacher == [0, 0]
    assert api_fit_dmg_enabled.dps.breacher == [approx(1000), approx(0.01)]
    assert api_fit_dmg_enabled.volley.breacher == [approx(1000), approx(0.01)]


def test_include_charges(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=25, cycle_time=10000, reload_time=30000)
    eve_charge_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=1000, dmg_rel=1, dmg_duration=75000, volume=0.5)
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
    assert api_module_dmg_without.dps.breacher == [0, 0]
    assert api_module_dmg_without.volley.breacher == [0, 0]
    assert api_module_dmg_with.dps.breacher == [approx(1000), approx(0.01)]
    assert api_module_dmg_with.volley.breacher == [approx(1000), approx(0.01)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=False), StatsOptionItemDmg(include_charges=True)])))
    api_charge_dmg_without, api_charge_dmg_with = api_charge_stats.dmg
    assert api_charge_dmg_without.dps.breacher == [approx(1000), approx(0.01)]
    assert api_charge_dmg_without.volley.breacher == [approx(1000), approx(0.01)]
    assert api_charge_dmg_with.dps.breacher == [approx(1000), approx(0.01)]
    assert api_charge_dmg_with.volley.breacher == [approx(1000), approx(0.01)]


def test_time_reload(client, consts):
    # Realistic case of Tholos with poor breacher duration/reload skills - when it has to reload,
    # there is downtime, so overall dps changes
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=4, cycle_time=24000, reload_time=30000)
    eve_charge_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=200, dmg_rel=0.75, dmg_duration=50000, volume=0.1)
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
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None))])))
    api_fleet_dmg_burst, api_fleet_dmg_reload = api_fleet_stats.dmg
    assert api_fleet_dmg_burst.dps.breacher == [approx(200), approx(0.0075)]
    assert api_fleet_dmg_reload.dps.breacher == [approx(199.191919), approx(0.007469697)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None))])))
    api_fit_dmg_burst, api_fit_dmg_reload = api_fit_stats.dmg
    assert api_fit_dmg_burst.dps.breacher == [approx(200), approx(0.0075)]
    assert api_fit_dmg_reload.dps.breacher == [approx(199.191919), approx(0.007469697)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(time_options=StatTimeBurst()),
        StatsOptionItemDmg(time_options=StatTimeSim(time=None))])))
    api_charge_dmg_burst, api_charge_dmg_reload = api_charge_stats.dmg
    assert api_charge_dmg_burst.dps.breacher == [approx(200), approx(0.0075)]
    assert api_charge_dmg_reload.dps.breacher == [approx(199.191919), approx(0.007469697)]
