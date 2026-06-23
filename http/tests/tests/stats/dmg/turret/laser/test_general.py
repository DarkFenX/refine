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
from tests.stats.dmg import make_eve_charge_crystal, make_eve_turret_laser, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_laser(
        client=client, basic_info=eve_basic_info, dmg_mult=15, capacity=1, cycle_time=2050, reload_time=0.01)
    eve_charge_id = make_eve_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(9, 2, 0, 0), volume=1,
        get_damaged=1, hp=1, vol_dmg=0.01, vol_chance=0.1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_fit_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_module_dmg_stats.volley == [approx(135), approx(30), 0, 0]
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
    assert api_module_dmg_ignored.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_module_dmg_ignored.volley == [approx(135), approx(30), 0, 0]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_fit_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_module_dmg_stats.volley == [approx(135), approx(30), 0, 0]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_laser(
        client=client, basic_info=eve_basic_info, dmg_mult=15, capacity=1, cycle_time=2050, reload_time=0.01)
    eve_charge1_id = make_eve_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(9, 2, 0, 0), volume=1,
        get_damaged=1, hp=1, vol_dmg=0.01, vol_chance=0.1)
    eve_charge2_id = make_eve_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(8.9, 8.9, 0, 0), volume=1,
        get_damaged=1, hp=1, vol_dmg=0.01, vol_chance=0.1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge1_id)
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge2_id)
    api_fit2 = api_sol.create_fit()
    api_fit2.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge1_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(196.829268), approx(94.390244), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(403.5), approx(193.5), 0, 0]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [approx(130.97561), approx(79.756098), 0, 0]
    assert api_fit1_dmg_stats.volley == [approx(268.5), approx(163.5), 0, 0]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_fit2_dmg_stats.volley == [approx(135), approx(30), 0, 0]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_laser(
        client=client, basic_info=eve_basic_info, dmg_mult=15, capacity=1, cycle_time=2050, reload_time=0.01)
    eve_charge_id = make_eve_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(9, 2, 0, 0), volume=1,
        get_damaged=1, hp=1, vol_dmg=0.01, vol_chance=0.1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, turret=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, turret=True))])))
    api_fleet_dmg_default, api_fleet_dmg_disabled, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_fleet_dmg_default.volley == [approx(135), approx(30), 0, 0]
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_fleet_dmg_enabled.volley == [approx(135), approx(30), 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, turret=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, turret=True))])))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_fit_dmg_default.volley == [approx(135), approx(30), 0, 0]
    assert api_fit_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_fit_dmg_enabled.volley == [approx(135), approx(30), 0, 0]


def test_time(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_laser(
        client=client, basic_info=eve_basic_info, dmg_mult=15, capacity=1, cycle_time=2050, reload_time=0.01)
    eve_charge_id = make_eve_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(9, 2, 0, 0), volume=1,
        get_damaged=1, hp=1, vol_dmg=0.01, vol_chance=0.1)
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
    assert api_fleet_dmg_stats.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_fit_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_module_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    # Verification - sim without time means stats with reload time are considered
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(65.82155), approx(14.627011), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(65.82155), approx(14.627011), 0, 0]
    assert api_fit_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(65.82155), approx(14.627011), 0, 0]
    assert api_module_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    # Verification - just after first hit landed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(135), approx(30), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(135), approx(30), 0, 0]
    assert api_fit_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(135), approx(30), 0, 0]
    assert api_module_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    # Verification - just before second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=2))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(67.5), approx(15), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=2))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(67.5), approx(15), 0, 0]
    assert api_fit_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=2))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(67.5), approx(15), 0, 0]
    assert api_module_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    # Verification - just after second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=2.1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(128.571429), approx(28.571429), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=2.1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(128.571429), approx(28.571429), 0, 0]
    assert api_fit_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=2.1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(128.571429), approx(28.571429), 0, 0]
    assert api_module_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    # Verification - just before reload starts
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=2049.9))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(65.856871), approx(14.63486), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=2049.9))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(65.856871), approx(14.63486), 0, 0]
    assert api_fit_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=2049.9))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(65.856871), approx(14.63486), 0, 0]
    assert api_module_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    # Verification - just before reload completes
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=2050.9))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(65.82476), approx(14.627724), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=2050.9))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(65.82476), approx(14.627724), 0, 0]
    assert api_fit_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=2050.9))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(65.82476), approx(14.627724), 0, 0]
    assert api_module_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    # Verification - after reload is done and another laser shot
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=2051.1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(65.88416), approx(14.640924), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=2051.1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(65.88416), approx(14.640924), 0, 0]
    assert api_fit_dmg_stats.volley == [approx(135), approx(30), 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=2051.1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [approx(65.88416), approx(14.640924), 0, 0]
    assert api_module_dmg_stats.volley == [approx(135), approx(30), 0, 0]


def test_crystal_damage_flag(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_laser(
        client=client, basic_info=eve_basic_info, dmg_mult=15, capacity=1, cycle_time=2050, reload_time=0.01)
    eve_charge1_id = make_eve_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(9, 2, 0, 0), volume=1,
        get_damaged=1, hp=1, vol_dmg=0.01, vol_chance=0.1)
    eve_charge2_id = make_eve_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(9, 2, 0, 0), volume=1,
        get_damaged=0, hp=1, vol_dmg=0.01, vol_chance=0.1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id)
    api_module2 = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge2_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification - impact of reload is significant despite reload time being super low due to
    # library enforcing 1 second reload time
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None))])))
    api_fleet_dmg_burst, api_fleet_dmg_reload = api_fleet_stats.dmg
    assert api_fleet_dmg_burst.dps == [approx(131.707317), approx(29.268293), 0, 0]
    assert api_fleet_dmg_reload.dps == [approx(131.675209), approx(29.261158), 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None))])))
    api_fit_dmg_burst, api_fit_dmg_reload = api_fit_stats.dmg
    assert api_fit_dmg_burst.dps == [approx(131.707317), approx(29.268293), 0, 0]
    assert api_fit_dmg_reload.dps == [approx(131.675209), approx(29.261158), 0, 0]
    api_module1_stats = api_module1.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(time_options=StatTimeBurst()),
        StatsOptionItemDmg(time_options=StatTimeSim(time=None))])))
    api_module1_dmg_burst, api_module1_dmg_reload = api_module1_stats.dmg
    assert api_module1_dmg_burst.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_module1_dmg_reload.dps == [approx(65.82155), approx(14.627011), 0, 0]
    api_module2_stats = api_module2.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(time_options=StatTimeBurst()),
        StatsOptionItemDmg(time_options=StatTimeSim(time=None))])))
    api_module2_dmg_burst, api_module2_dmg_reload = api_module2_stats.dmg
    assert api_module2_dmg_burst.dps == [approx(65.853659), approx(14.634146), 0, 0]
    assert api_module2_dmg_reload.dps == [approx(65.853659), approx(14.634146), 0, 0]


def test_charge_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_laser(
        client=client, basic_info=eve_basic_info, dmg_mult=15, capacity=1, cycle_time=2050, reload_time=0.01)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_dmg_stats.volley == [0, 0, 0, 0]


def test_charge_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_laser(
        client=client, basic_info=eve_basic_info, dmg_mult=15, capacity=1, cycle_time=2050, reload_time=0.01)
    eve_charge_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_dmg_stats.volley == [0, 0, 0, 0]
