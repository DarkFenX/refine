"""
Defenders are excluded from fit-level and fleet-level stats, and have no appropriate category
altogether. The only way to access stats is to get launcher/missile stats.
"""

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
from tests.stats.dmg import make_eve_launcher, make_eve_missile_defender, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=0.15,
        cycle_time=5000, disallow_repeating_activation=1, reactivation_delay=60000, reload_time=10000)
    eve_charge_id = make_eve_missile_defender(
        client=client, basic_info=eve_basic_info, dmgs=(200, 200, 200, 200), volume=0.015)
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
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=True)]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_module_dmg_stats.volley == [approx(200), approx(200), approx(200), approx(200)]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_charge_dmg_stats.volley == [approx(200), approx(200), approx(200), approx(200)]
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
    assert api_module_dmg_ignored.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_module_dmg_ignored.volley == [approx(200), approx(200), approx(200), approx(200)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_charge_dmg_normal, api_charge_dmg_ignored = api_charge_stats.dmg
    assert api_charge_dmg_normal.dps == [0, 0, 0, 0]
    assert api_charge_dmg_normal.volley == [0, 0, 0, 0]
    assert api_charge_dmg_ignored.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_charge_dmg_ignored.volley == [approx(200), approx(200), approx(200), approx(200)]
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
    assert api_module_dmg_ignored.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_module_dmg_ignored.volley == [approx(200), approx(200), approx(200), approx(200)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_charge_dmg_normal, api_charge_dmg_ignored = api_charge_stats.dmg
    assert api_charge_dmg_normal.dps == [0, 0, 0, 0]
    assert api_charge_dmg_normal.volley == [0, 0, 0, 0]
    assert api_charge_dmg_ignored.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_charge_dmg_ignored.volley == [approx(200), approx(200), approx(200), approx(200)]
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
    assert api_module_dmg_ignored.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_module_dmg_ignored.volley == [approx(200), approx(200), approx(200), approx(200)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_charge_dmg_normal, api_charge_dmg_ignored = api_charge_stats.dmg
    assert api_charge_dmg_normal.dps == [0, 0, 0, 0]
    assert api_charge_dmg_normal.volley == [0, 0, 0, 0]
    assert api_charge_dmg_ignored.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_charge_dmg_ignored.volley == [approx(200), approx(200), approx(200), approx(200)]
    # Action
    api_module.charge.change_charge(state=True)
    # Verification - module is active, charge is not force-disabled, so stats are exposed without
    # extra flags
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=True)]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_module_dmg_stats.volley == [approx(200), approx(200), approx(200), approx(200)]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_charge_dmg_stats.volley == [approx(200), approx(200), approx(200), approx(200)]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=0.15,
        cycle_time=5000, disallow_repeating_activation=1, reactivation_delay=60000, reload_time=10000)
    eve_charge_id = make_eve_missile_defender(
        client=client, basic_info=eve_basic_info, dmgs=(200, 200, 200, 200), volume=0.015)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(), StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True))])))
    api_fleet_dmg_default, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_default.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.volley == [0, 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(), StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True))])))
    api_fit_dmg_default, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [0, 0, 0, 0]
    assert api_fit_dmg_default.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.volley == [0, 0, 0, 0]


def test_include_charges(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=0.15,
        cycle_time=5000, disallow_repeating_activation=1, reactivation_delay=60000, reload_time=10000)
    eve_charge_id = make_eve_missile_defender(
        client=client, basic_info=eve_basic_info, dmgs=(200, 200, 200, 200), volume=0.015)
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
    assert api_module_dmg_with.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_module_dmg_with.volley == [approx(200), approx(200), approx(200), approx(200)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=False), StatsOptionItemDmg(include_charges=True)])))
    api_charge_dmg_without, api_charge_dmg_with = api_charge_stats.dmg
    assert api_charge_dmg_without.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_charge_dmg_without.volley == [approx(200), approx(200), approx(200), approx(200)]
    assert api_charge_dmg_with.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_charge_dmg_with.volley == [approx(200), approx(200), approx(200), approx(200)]


def test_time(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=0.15,
        cycle_time=5000, disallow_repeating_activation=1, reactivation_delay=60000, reload_time=10000)
    eve_charge_id = make_eve_missile_defender(
        client=client, basic_info=eve_basic_info, dmgs=(200, 200, 200, 200), volume=0.015)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_charge_dmg_stats.volley == [approx(200), approx(200), approx(200), approx(200)]
    # Verification - sim without time means stats with reload time are considered. Since defender
    # launchers have reactivation delay which is longer than reload time, burst and sustained DPS
    # are the same
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(3.076923), approx(3.076923), approx(3.076923), approx(3.076923)]
    assert api_charge_dmg_stats.volley == [approx(200), approx(200), approx(200), approx(200)]
    # Verification - just after first hit landed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(200), approx(200), approx(200), approx(200)]
    assert api_charge_dmg_stats.volley == [approx(200), approx(200), approx(200), approx(200)]
    # Verification - just before second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=64))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=64))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=64))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(3.125), approx(3.125), approx(3.125), approx(3.125)]
    assert api_charge_dmg_stats.volley == [approx(200), approx(200), approx(200), approx(200)]
    # Verification - just after second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=66))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=66))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=66))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(6.060606), approx(6.060606), approx(6.060606), approx(6.060606)]
    assert api_charge_dmg_stats.volley == [approx(200), approx(200), approx(200), approx(200)]
    # Verification - before 11th hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=714))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=714))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=714))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(3.081232), approx(3.081232), approx(3.081232), approx(3.081232)]
    assert api_charge_dmg_stats.volley == [approx(200), approx(200), approx(200), approx(200)]
    # Verification - after 11th hit. Here we check that reload happens during reactivation, not on
    # top of it.
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=716))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=716))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_charge_dmg_stats = api_module.charge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=716))]))).dmg.one()
    assert api_charge_dmg_stats.dps == [approx(3.351955), approx(3.351955), approx(3.351955), approx(3.351955)]
    assert api_charge_dmg_stats.volley == [approx(200), approx(200), approx(200), approx(200)]
