from fw import Spool, approx
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
from tests.stats.dmg import make_eve_charge_normal, make_eve_turret_spool, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=4.28, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=2180, reload_time=0.01)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 63, 0, 33), volume=0.01)
    client.create_sources()
    api_sol = client.create_sol(default_spool=Spool.spool_scale_to_api(val=1))
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fleet_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fit_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_module_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
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
    assert api_module_dmg_ignored.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_module_dmg_ignored.volley == [0, approx(842.625), 0, approx(441.375)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fleet_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fit_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_module_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=4.28, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=2180, reload_time=0.01)
    eve_charge1_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 63, 0, 33), volume=0.01)
    eve_charge2_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 72, 0, 63), volume=0.01)
    client.create_sources()
    api_sol = client.create_sol(default_spool=Spool.spool_scale_to_api(val=1))
    api_fit1 = api_sol.create_fit()
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge1_id)
    api_fit1.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge2_id)
    api_fit2 = api_sol.create_fit()
    api_fit2.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge1_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(1214.793578), 0, approx(791.456422)]
    assert api_fleet_dmg_stats.volley == [0, approx(2648.25), 0, approx(1725.375)]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [0, approx(828.268349), 0, approx(588.990826)]
    assert api_fit1_dmg_stats.volley == [0, approx(1805.625), 0, approx(1284)]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fit2_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=4.28, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=2180, reload_time=0.01)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 63, 0, 33), volume=0.01)
    client.create_sources()
    api_sol = client.create_sol(default_spool=Spool.spool_scale_to_api(val=1))
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, turret=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, turret=True))])))
    api_fleet_dmg_default, api_fleet_dmg_disabled, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fleet_dmg_default.volley == [0, approx(842.625), 0, approx(441.375)]
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fleet_dmg_enabled.volley == [0, approx(842.625), 0, approx(441.375)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, turret=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, turret=True))])))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fit_dmg_default.volley == [0, approx(842.625), 0, approx(441.375)]
    assert api_fit_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fit_dmg_enabled.volley == [0, approx(842.625), 0, approx(441.375)]


def test_time_burst(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=4.28, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=2180, reload_time=0.01)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 63, 0, 33), volume=0.01)
    client.create_sources()
    api_sol = client.create_sol(default_spool=Spool.spool_scale_to_api(val=0.5))
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))])))
    api_fleet_dmg_pre, api_fleet_dmg_default, api_fleet_dmg_full = api_fleet_stats.dmg
    assert api_fleet_dmg_pre.dps == [0, approx(123.688073), 0, approx(64.788991)]
    assert api_fleet_dmg_pre.volley == [0, approx(269.64), 0, approx(141.24)]
    assert api_fleet_dmg_default.dps == [0, approx(262.218716), 0, approx(137.352661)]
    assert api_fleet_dmg_default.volley == [0, approx(571.6368), 0, approx(299.4288)]
    assert api_fleet_dmg_full.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fleet_dmg_full.volley == [0, approx(842.625), 0, approx(441.375)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))])))
    api_fit_dmg_pre, api_fit_dmg_default, api_fit_dmg_full = api_fit_stats.dmg
    assert api_fit_dmg_pre.dps == [0, approx(123.688073), 0, approx(64.788991)]
    assert api_fit_dmg_pre.volley == [0, approx(269.64), 0, approx(141.24)]
    assert api_fit_dmg_default.dps == [0, approx(262.218716), 0, approx(137.352661)]
    assert api_fit_dmg_default.volley == [0, approx(571.6368), 0, approx(299.4288)]
    assert api_fit_dmg_full.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fit_dmg_full.volley == [0, approx(842.625), 0, approx(441.375)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionItemDmg(time_options=StatTimeBurst()),
        StatsOptionItemDmg(time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))])))
    api_module_dmg_pre, api_module_dmg_default, api_module_dmg_full = api_module_stats.dmg
    assert api_module_dmg_pre.dps == [0, approx(123.688073), 0, approx(64.788991)]
    assert api_module_dmg_pre.volley == [0, approx(269.64), 0, approx(141.24)]
    assert api_module_dmg_default.dps == [0, approx(262.218716), 0, approx(137.352661)]
    assert api_module_dmg_default.volley == [0, approx(571.6368), 0, approx(299.4288)]
    assert api_module_dmg_full.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_module_dmg_full.volley == [0, approx(842.625), 0, approx(441.375)]
    # Action
    api_module.change_module(spool=Spool.spool_scale_to_api(val=0.7))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))])))
    api_fleet_dmg_pre, api_fleet_dmg_default, api_fleet_dmg_full = api_fleet_stats.dmg
    assert api_fleet_dmg_pre.dps == [0, approx(123.688073), 0, approx(64.788991)]
    assert api_fleet_dmg_pre.volley == [0, approx(269.64), 0, approx(141.24)]
    assert api_fleet_dmg_default.dps == [0, approx(314.167706), 0, approx(164.564037)]
    assert api_fleet_dmg_default.volley == [0, approx(684.8856), 0, approx(358.7496)]
    assert api_fleet_dmg_full.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fleet_dmg_full.volley == [0, approx(842.625), 0, approx(441.375)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))])))
    api_fit_dmg_pre, api_fit_dmg_default, api_fit_dmg_full = api_fit_stats.dmg
    assert api_fit_dmg_pre.dps == [0, approx(123.688073), 0, approx(64.788991)]
    assert api_fit_dmg_pre.volley == [0, approx(269.64), 0, approx(141.24)]
    assert api_fit_dmg_default.dps == [0, approx(314.167706), 0, approx(164.564037)]
    assert api_fit_dmg_default.volley == [0, approx(684.8856), 0, approx(358.7496)]
    assert api_fit_dmg_full.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_fit_dmg_full.volley == [0, approx(842.625), 0, approx(441.375)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionItemDmg(time_options=StatTimeBurst()),
        StatsOptionItemDmg(time_options=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))])))
    api_module_dmg_pre, api_module_dmg_default, api_module_dmg_full = api_module_stats.dmg
    assert api_module_dmg_pre.dps == [0, approx(123.688073), 0, approx(64.788991)]
    assert api_module_dmg_default.dps == [0, approx(314.167706), 0, approx(164.564037)]
    assert api_module_dmg_full.dps == [0, approx(386.525229), 0, approx(202.465596)]
    assert api_module_dmg_pre.volley == [0, approx(269.64), 0, approx(141.24)]
    assert api_module_dmg_default.volley == [0, approx(684.8856), 0, approx(358.7496)]
    assert api_module_dmg_full.volley == [0, approx(842.625), 0, approx(441.375)]


def test_time_sim(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=4.28, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=2180, reload_time=0.01)
    eve_charge_id = make_eve_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 63, 0, 33), volume=0.01)
    client.create_sources()
    api_sol = client.create_sol(default_spool=Spool.spool_scale_to_api(val=1))
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    api_module.update()
    # Verification - sim without time means that reload is considered, and number is average over
    # whole period, which includes spool
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(377.934690), 0, approx(197.96579)]
    assert api_fleet_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(377.934690), 0, approx(197.96579)]
    assert api_fit_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=None))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(377.934690), 0, approx(197.96579)]
    assert api_module_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    # Verification - just after first hit landed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(269.64), 0, approx(141.24)]
    assert api_fleet_dmg_stats.volley == [0, approx(269.64), 0, approx(141.24)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(269.64), 0, approx(141.24)]
    assert api_fit_dmg_stats.volley == [0, approx(269.64), 0, approx(141.24)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(269.64), 0, approx(141.24)]
    assert api_module_dmg_stats.volley == [0, approx(269.64), 0, approx(141.24)]
    # Verification - just before second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=2.1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(128.4), 0, approx(67.257143)]
    assert api_fleet_dmg_stats.volley == [0, approx(269.64), 0, approx(141.24)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=2.1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(128.4), 0, approx(67.257143)]
    assert api_fit_dmg_stats.volley == [0, approx(269.64), 0, approx(141.24)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=2.1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(128.4), 0, approx(67.257143)]
    assert api_module_dmg_stats.volley == [0, approx(269.64), 0, approx(141.24)]
    # Verification - just after second hit (higher volley was recorded, higher dps)
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=2.2))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(253.706727), 0, approx(132.894)]
    assert api_fleet_dmg_stats.volley == [0, approx(288.5148), 0, approx(151.1268)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=2.2))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(253.706727), 0, approx(132.894)]
    assert api_fit_dmg_stats.volley == [0, approx(288.5148), 0, approx(151.1268)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=2.2))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(253.706727), 0, approx(132.894)]
    assert api_module_dmg_stats.volley == [0, approx(288.5148), 0, approx(151.1268)]
    # Verification - after 15th hit, which is about mid-spool
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=32.8))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(200.585854), 0, approx(105.06878)]
    assert api_fleet_dmg_stats.volley == [0, approx(552.762), 0, approx(289.542)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=32.8))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(200.585854), 0, approx(105.06878)]
    assert api_fit_dmg_stats.volley == [0, approx(552.762), 0, approx(289.542)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=32.8))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(200.585854), 0, approx(105.06878)]
    assert api_module_dmg_stats.volley == [0, approx(552.762), 0, approx(289.542)]
    # Verification - after 30th hit, which is almost full spool
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=65.5))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(261.61255), 0, approx(137.035145)]
    assert api_fleet_dmg_stats.volley == [0, approx(835.884), 0, approx(437.844)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=65.5))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(261.61255), 0, approx(137.035145)]
    assert api_fit_dmg_stats.volley == [0, approx(835.884), 0, approx(437.844)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=65.5))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(261.61255), 0, approx(137.035145)]
    assert api_module_dmg_stats.volley == [0, approx(835.884), 0, approx(437.844)]
    # Verification - after 31st hit, which is full spool
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=67.6))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(265.950399), 0, approx(139.307352)]
    assert api_fleet_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=67.6))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(265.950399), 0, approx(139.307352)]
    assert api_fit_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=67.6))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(265.950399), 0, approx(139.307352)]
    assert api_module_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    # Verification - just before reload starts
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1089.9))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(378.316127), 0, approx(198.16559)]
    assert api_fleet_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1089.9))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(378.316127), 0, approx(198.16559)]
    assert api_fit_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=1089.9))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(378.316127), 0, approx(198.16559)]
    assert api_module_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    # Verification - just before reload completes
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1090.9))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(377.969334), 0, approx(197.983937)]
    assert api_fleet_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1090.9))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(377.969334), 0, approx(197.983937)]
    assert api_fit_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=1090.9))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(377.969334), 0, approx(197.983937)]
    assert api_module_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    # Verification - after reload is done and another laser shot
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1091.1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(378.147179), 0, approx(198.077094)]
    assert api_fleet_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1091.1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(378.147179), 0, approx(198.077094)]
    assert api_fit_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=1091.1))]))).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(378.147179), 0, approx(198.077094)]
    assert api_module_dmg_stats.volley == [0, approx(842.625), 0, approx(441.375)]


def test_charge_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=4.28, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=2180, reload_time=0.01)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
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
    eve_module_id = make_eve_turret_spool(
        client=client, basic_info=eve_basic_info,
        dmg_mult=4.28, spool_step=0.07, spool_max=2.125,
        capacity=5, cycle_time=2180, reload_time=0.01)
    eve_charge_id = client.alloc_item_id()
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
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, 0, 0, 0]
    assert api_module_dmg_stats.volley == [0, 0, 0, 0]
