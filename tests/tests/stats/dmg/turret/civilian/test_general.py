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
from tests.stats.dmg import make_eve_charge_crystal, make_eve_turret_civilian, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_charge_id = make_eve_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 2, 3), volume=1,
        get_damaged=0, hp=1, vol_dmg=0.00025, vol_chance=1)
    eve_module_id = make_eve_turret_civilian(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 2, 3), dmg_mult=2.4,
        cycle_time=1200, reload_time=0.01, charge_type_id=eve_charge_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_fleet_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_fit_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_module_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
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
    assert api_module_dmg_ignored.dps == [0, 0, approx(4), approx(6)]
    assert api_module_dmg_ignored.volley == [0, 0, approx(4.8), approx(7.2)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_fleet_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_fit_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_module_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_charge_id = make_eve_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 2, 3), volume=1,
        get_damaged=0, hp=1, vol_dmg=0.00025, vol_chance=1)
    eve_module_id = make_eve_turret_civilian(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 2, 3), dmg_mult=2.4,
        cycle_time=1200, reload_time=0.01, charge_type_id=eve_charge_id)
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
    assert api_fleet_dmg_stats.dps == [0, 0, approx(12), approx(18)]
    assert api_fleet_dmg_stats.volley == [0, 0, approx(14.4), approx(21.6)]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [0, 0, approx(8), approx(12)]
    assert api_fit1_dmg_stats.volley == [0, 0, approx(9.6), approx(14.4)]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_fit2_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_charge_id = make_eve_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 2, 3), volume=1,
        get_damaged=0, hp=1, vol_dmg=0.00025, vol_chance=1)
    eve_module_id = make_eve_turret_civilian(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 2, 3), dmg_mult=2.4,
        cycle_time=1200, reload_time=0.01, charge_type_id=eve_charge_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, turret=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, turret=True))])))
    api_fleet_dmg_default, api_fleet_dmg_disabled, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [0, 0, approx(4), approx(6)]
    assert api_fleet_dmg_default.volley == [0, 0, approx(4.8), approx(7.2)]
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [0, 0, approx(4), approx(6)]
    assert api_fleet_dmg_enabled.volley == [0, 0, approx(4.8), approx(7.2)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, turret=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, turret=True))])))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [0, 0, approx(4), approx(6)]
    assert api_fit_dmg_default.volley == [0, 0, approx(4.8), approx(7.2)]
    assert api_fit_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [0, 0, approx(4), approx(6)]
    assert api_fit_dmg_enabled.volley == [0, 0, approx(4.8), approx(7.2)]


def test_time(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_charge_id = make_eve_charge_crystal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 2, 3), volume=1,
        get_damaged=0, hp=1, vol_dmg=0.00025, vol_chance=1)
    eve_module_id = make_eve_turret_civilian(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 2, 3), dmg_mult=2.4,
        cycle_time=1200, reload_time=0.01, charge_type_id=eve_charge_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification - burst stats (reload is ignored)
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_fleet_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_fit_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_module_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    # Verification - sim without time means stats with reload time are considered. But, since
    # civilian guns have no reload, dps stats are the same
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_fleet_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_fit_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_module_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    # Verification - just after first hit landed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=0.1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, approx(48), approx(72)]
    assert api_fleet_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=0.1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, approx(48), approx(72)]
    assert api_fit_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=0.1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, approx(48), approx(72)]
    assert api_module_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    # Verification - just before second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1.1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, approx(4.363636), approx(6.545455)]
    assert api_fleet_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1.1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, approx(4.363636), approx(6.545455)]
    assert api_fit_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=1.1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, approx(4.363636), approx(6.545455)]
    assert api_module_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    # Verification - just after second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1.3))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, approx(7.384615), approx(11.076923)]
    assert api_fleet_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1.3))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, approx(7.384615), approx(11.076923)]
    assert api_fit_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=1.3))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, approx(7.384615), approx(11.076923)]
    assert api_module_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    # Verification - just before reload would start if crystal was damaged
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=4799.9))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, approx(4.000083), approx(6.000125)]
    assert api_fleet_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=4799.9))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, approx(4.000083), approx(6.000125)]
    assert api_fit_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=4799.9))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, approx(4.000083), approx(6.000125)]
    assert api_module_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    # Verification - after reload would start, but instead there is another damage instance
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=4800.1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, approx(4.000917), approx(6.001375)]
    assert api_fleet_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=4800.1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, approx(4.000917), approx(6.001375)]
    assert api_fit_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=4800.1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, approx(4.000917), approx(6.001375)]
    assert api_module_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]


def test_charge_absent(client, consts):
    # Civilian guns use on-gun damage stats (confirmed by FC Kestrel on 2025-07-30), so if charge is
    # not there, it's not an issue for the damage dealing part
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_civilian(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 2, 3), dmg_mult=2.4,
        cycle_time=1200, reload_time=0.01)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_fleet_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_fit_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_module_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]


def test_charge_not_loaded(client, consts):
    # Civilian guns use on-gun damage stats (confirmed by FC Kestrel on 2025-07-30), so if charge is
    # not there, it's not an issue for the damage dealing part
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_charge_id = client.alloc_item_id()
    eve_module_id = make_eve_turret_civilian(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 2, 3), dmg_mult=2.4,
        cycle_time=1200, reload_time=0.01, charge_type_id=eve_charge_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_fleet_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_fit_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, approx(4), approx(6)]
    assert api_module_dmg_stats.volley == [0, 0, approx(4.8), approx(7.2)]
