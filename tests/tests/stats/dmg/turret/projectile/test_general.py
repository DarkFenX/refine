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
from tests.stats.dmg import make_eve_charge_normal, make_eve_turret_proj, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=45, capacity=0.25, cycle_time=8000, reload_time=10000)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 23, 4.6, 0), volume=0.0125)
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
    assert api_fleet_dmg_stats.dps == [0, approx(131.9625), approx(26.3925), 0]
    assert api_fleet_dmg_stats.volley == [0, approx(1055.7), approx(211.14), 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(131.9625), approx(26.3925), 0]
    assert api_fit_dmg_stats.volley == [0, approx(1055.7), approx(211.14), 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(131.9625), approx(26.3925), 0]
    assert api_module_dmg_stats.volley == [0, approx(1055.7), approx(211.14), 0]
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
        dmg=[StatsOptionItemDmg(), StatsOptionItemDmg(state=consts.ApiStatItemState.switch)]))
    api_module_dmg_normal, api_module_dmg_ignored = api_module_stats.dmg
    assert api_module_dmg_normal.dps == [0, 0, 0, 0]
    assert api_module_dmg_normal.volley == [0, 0, 0, 0]
    assert api_module_dmg_ignored.dps == [0, approx(131.9625), approx(26.3925), 0]
    assert api_module_dmg_ignored.volley == [0, approx(1055.7), approx(211.14), 0]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(131.9625), approx(26.3925), 0]
    assert api_fleet_dmg_stats.volley == [0, approx(1055.7), approx(211.14), 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(131.9625), approx(26.3925), 0]
    assert api_fit_dmg_stats.volley == [0, approx(1055.7), approx(211.14), 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(131.9625), approx(26.3925), 0]
    assert api_module_dmg_stats.volley == [0, approx(1055.7), approx(211.14), 0]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=45, capacity=0.25, cycle_time=8000, reload_time=10000)
    eve_charge1_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 23, 4.6, 0), volume=0.0125)
    eve_charge2_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(20.7, 0, 2.3, 4.6), volume=0.0125)
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
    assert api_fleet_dmg_stats.dps == [approx(118.76625), approx(263.925), approx(65.98125), approx(26.3925)]
    assert api_fleet_dmg_stats.volley == [approx(950.13), approx(2111.4), approx(527.85), approx(211.14)]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [approx(118.76625), approx(131.9625), approx(39.58875), approx(26.3925)]
    assert api_fit1_dmg_stats.volley == [approx(950.13), approx(1055.7), approx(316.71), approx(211.14)]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [0, approx(131.9625), approx(26.3925), 0]
    assert api_fit2_dmg_stats.volley == [0, approx(1055.7), approx(211.14), 0]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=45, capacity=0.25, cycle_time=8000, reload_time=10000)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 23, 4.6, 0), volume=0.0125)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=[
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, turret=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, turret=True))]))
    api_fleet_dmg_default, api_fleet_dmg_disabled, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [0, approx(131.9625), approx(26.3925), 0]
    assert api_fleet_dmg_default.volley == [0, approx(1055.7), approx(211.14), 0]
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [0, approx(131.9625), approx(26.3925), 0]
    assert api_fleet_dmg_enabled.volley == [0, approx(1055.7), approx(211.14), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=[
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, turret=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, turret=True))]))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [0, approx(131.9625), approx(26.3925), 0]
    assert api_fit_dmg_default.volley == [0, approx(1055.7), approx(211.14), 0]
    assert api_fit_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [0, approx(131.9625), approx(26.3925), 0]
    assert api_fit_dmg_enabled.volley == [0, approx(1055.7), approx(211.14), 0]


def test_time(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=45, capacity=0.25, cycle_time=8000, reload_time=10000)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(20.7, 0, 2.3, 4.6), volume=0.0125)
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
        dmg=[StatsOptionFitDmg(time=StatTimeBurst())])).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(118.76625), 0, approx(13.19625), approx(26.3925)]
    assert api_fleet_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeBurst())])).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(118.76625), 0, approx(13.19625), approx(26.3925)]
    assert api_fit_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeBurst())])).dmg.one()
    assert api_module_dmg_stats.dps == [approx(118.76625), 0, approx(13.19625), approx(26.3925)]
    assert api_module_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    # Verification - sim without time means stats with reload time are considered
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=None))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(111.78), 0, approx(12.42), approx(24.84)]
    assert api_fleet_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=None))])).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(111.78), 0, approx(12.42), approx(24.84)]
    assert api_fit_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=None))])).dmg.one()
    assert api_module_dmg_stats.dps == [approx(111.78), 0, approx(12.42), approx(24.84)]
    assert api_module_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    # Verification - just after first hit landed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(950.13), 0, approx(105.57), approx(211.14)]
    assert api_fleet_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1))])).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(950.13), 0, approx(105.57), approx(211.14)]
    assert api_fit_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=1))])).dmg.one()
    assert api_module_dmg_stats.dps == [approx(950.13), 0, approx(105.57), approx(211.14)]
    assert api_module_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    # Verification - just before second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=7))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(135.732857), 0, approx(15.081429), approx(30.162857)]
    assert api_fleet_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=7))])).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(135.732857), 0, approx(15.081429), approx(30.162857)]
    assert api_fit_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=7))])).dmg.one()
    assert api_module_dmg_stats.dps == [approx(135.732857), 0, approx(15.081429), approx(30.162857)]
    assert api_module_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    # Verification - just after second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=9))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(211.14), 0, approx(23.46), approx(46.92)]
    assert api_fleet_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=9))])).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(211.14), 0, approx(23.46), approx(46.92)]
    assert api_fit_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=9))])).dmg.one()
    assert api_module_dmg_stats.dps == [approx(211.14), 0, approx(23.46), approx(46.92)]
    assert api_module_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    # Verification - just before reload starts
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=159))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(119.513208), 0, approx(13.279245), approx(26.558491)]
    assert api_fleet_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=159))])).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(119.513208), 0, approx(13.279245), approx(26.558491)]
    assert api_fit_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=159))])).dmg.one()
    assert api_module_dmg_stats.dps == [approx(119.513208), 0, approx(13.279245), approx(26.558491)]
    assert api_module_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    # Verification - just before reload completes
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=169))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(112.44142), 0, approx(12.493491), approx(24.986982)]
    assert api_fleet_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=169))])).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(112.44142), 0, approx(12.493491), approx(24.986982)]
    assert api_fit_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=169))])).dmg.one()
    assert api_module_dmg_stats.dps == [approx(112.44142), 0, approx(12.493491), approx(24.986982)]
    assert api_module_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    # Verification - after reload is done and another arty hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=171))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(116.682632), 0, approx(12.964737), approx(25.929474)]
    assert api_fleet_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=171))])).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(116.682632), 0, approx(12.964737), approx(25.929474)]
    assert api_fit_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=171))])).dmg.one()
    assert api_module_dmg_stats.dps == [approx(116.682632), 0, approx(12.964737), approx(25.929474)]
    assert api_module_dmg_stats.volley == [approx(950.13), 0, approx(105.57), approx(211.14)]


def test_charge_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=45, capacity=0.25, cycle_time=8000, reload_time=10000)
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
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=45, capacity=0.25, cycle_time=8000, reload_time=10000)
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
