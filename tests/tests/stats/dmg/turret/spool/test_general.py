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
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_fleet_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_fit_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_module_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
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
    assert api_module_dmg_ignored.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_module_dmg_ignored.volley == [0, approx(855.306506), 0, approx(448.017694)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_fleet_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_fit_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_module_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]


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
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(1233.076221), 0, approx(803.367841)]
    assert api_fleet_dmg_stats.volley == [0, approx(2688.106163), 0, approx(1751.341894)]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [0, approx(840.733787), 0, approx(597.855138)]
    assert api_fit1_dmg_stats.volley == [0, approx(1832.799656), 0, approx(1303.3242)]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_fit2_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]


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
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=[
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, turret=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, turret=True))]))
    api_fleet_dmg_default, api_fleet_dmg_disabled, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_fleet_dmg_default.volley == [0, approx(855.306506), 0, approx(448.017694)]
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_fleet_dmg_enabled.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=[
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, turret=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, turret=True))]))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_fit_dmg_default.volley == [0, approx(855.306506), 0, approx(448.017694)]
    assert api_fit_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_fit_dmg_enabled.volley == [0, approx(855.306506), 0, approx(448.017694)]


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
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=[
        StatsOptionFitDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionFitDmg(time=StatTimeBurst()),
        StatsOptionFitDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))]))
    api_fleet_dmg_pre, api_fleet_dmg_default, api_fleet_dmg_full = api_fleet_stats.dmg
    assert api_fleet_dmg_pre.dps == [0, approx(125.549579), 0, approx(65.764065)]
    assert api_fleet_dmg_pre.volley == [0, approx(273.698082), 0, approx(143.365662)]
    assert api_fleet_dmg_default.dps == [0, approx(266.165107), 0, approx(139.419818)]
    assert api_fleet_dmg_default.volley == [0, approx(580.239934), 0, approx(303.935203)]
    assert api_fleet_dmg_full.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_fleet_dmg_full.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=[
        StatsOptionFitDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionFitDmg(time=StatTimeBurst()),
        StatsOptionFitDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))]))
    api_fit_dmg_pre, api_fit_dmg_default, api_fit_dmg_full = api_fit_stats.dmg
    assert api_fit_dmg_pre.dps == [0, approx(125.549579), 0, approx(65.764065)]
    assert api_fit_dmg_pre.volley == [0, approx(273.698082), 0, approx(143.365662)]
    assert api_fit_dmg_default.dps == [0, approx(266.165107), 0, approx(139.419818)]
    assert api_fit_dmg_default.volley == [0, approx(580.239934), 0, approx(303.935203)]
    assert api_fit_dmg_full.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_fit_dmg_full.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=[
        StatsOptionItemDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionItemDmg(time=StatTimeBurst()),
        StatsOptionItemDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))]))
    api_module_dmg_pre, api_module_dmg_default, api_module_dmg_full = api_module_stats.dmg
    assert api_module_dmg_pre.dps == [0, approx(125.549579), 0, approx(65.764065)]
    assert api_module_dmg_pre.volley == [0, approx(273.698082), 0, approx(143.365662)]
    assert api_module_dmg_default.dps == [0, approx(266.165107), 0, approx(139.419818)]
    assert api_module_dmg_default.volley == [0, approx(580.239934), 0, approx(303.935203)]
    assert api_module_dmg_full.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_module_dmg_full.volley == [0, approx(855.306506), 0, approx(448.017694)]
    # Action
    api_module.change_module(spool_override=Spool.spool_scale_to_api(val=0.7))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=[
        StatsOptionFitDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionFitDmg(time=StatTimeBurst()),
        StatsOptionFitDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))]))
    api_fleet_dmg_pre, api_fleet_dmg_default, api_fleet_dmg_full = api_fleet_stats.dmg
    assert api_fleet_dmg_pre.dps == [0, approx(125.549579), 0, approx(65.764065)]
    assert api_fleet_dmg_pre.volley == [0, approx(273.698082), 0, approx(143.365662)]
    assert api_fleet_dmg_default.dps == [0, approx(318.89593), 0, approx(167.040725)]
    assert api_fleet_dmg_default.volley == [0, approx(695.193128), 0, approx(364.148781)]
    assert api_fleet_dmg_full.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_fleet_dmg_full.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=[
        StatsOptionFitDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionFitDmg(time=StatTimeBurst()),
        StatsOptionFitDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))]))
    api_fit_dmg_pre, api_fit_dmg_default, api_fit_dmg_full = api_fit_stats.dmg
    assert api_fit_dmg_pre.dps == [0, approx(125.549579), 0, approx(65.764065)]
    assert api_fit_dmg_pre.volley == [0, approx(273.698082), 0, approx(143.365662)]
    assert api_fit_dmg_default.dps == [0, approx(318.89593), 0, approx(167.040725)]
    assert api_fit_dmg_default.volley == [0, approx(695.193128), 0, approx(364.148781)]
    assert api_fit_dmg_full.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_fit_dmg_full.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=[
        StatsOptionItemDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionItemDmg(time=StatTimeBurst()),
        StatsOptionItemDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))]))
    api_module_dmg_pre, api_module_dmg_default, api_module_dmg_full = api_module_stats.dmg
    assert api_module_dmg_pre.dps == [0, approx(125.549579), 0, approx(65.764065)]
    assert api_module_dmg_default.dps == [0, approx(318.89593), 0, approx(167.040725)]
    assert api_module_dmg_full.dps == [0, approx(392.342434), 0, approx(205.512704)]
    assert api_module_dmg_pre.volley == [0, approx(273.698082), 0, approx(143.365662)]
    assert api_module_dmg_default.volley == [0, approx(695.193128), 0, approx(364.148781)]
    assert api_module_dmg_full.volley == [0, approx(855.306506), 0, approx(448.017694)]


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
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    api_module.update()
    # Verification - sim without time means that reload is considered, and number is average over
    # whole period, which includes spool
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=None))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(383.622607), 0, approx(200.945175)]
    assert api_fleet_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=None))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(383.622607), 0, approx(200.945175)]
    assert api_fit_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=None))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(383.622607), 0, approx(200.945175)]
    assert api_module_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    # Verification - just after first hit landed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(273.698082), 0, approx(143.365662)]
    assert api_fleet_dmg_stats.volley == [0, approx(273.698082), 0, approx(143.365662)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(273.698082), 0, approx(143.365662)]
    assert api_fit_dmg_stats.volley == [0, approx(273.698082), 0, approx(143.365662)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=1))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(273.698082), 0, approx(143.365662)]
    assert api_module_dmg_stats.volley == [0, approx(273.698082), 0, approx(143.365662)]
    # Verification - just before second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=2.1))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(130.33242), 0, approx(68.269363)]
    assert api_fleet_dmg_stats.volley == [0, approx(273.698082), 0, approx(143.365662)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=2.1))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(130.33242), 0, approx(68.269363)]
    assert api_fit_dmg_stats.volley == [0, approx(273.698082), 0, approx(143.365662)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=2.1))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(130.33242), 0, approx(68.269363)]
    assert api_module_dmg_stats.volley == [0, approx(273.698082), 0, approx(143.365662)]
    # Verification - just after second hit (higher volley was recorded, higher dps)
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=2.2))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(257.525014), 0, approx(134.894055)]
    assert api_fleet_dmg_stats.volley == [0, approx(292.856948), 0, approx(153.401258)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=2.2))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(257.525014), 0, approx(134.894055)]
    assert api_fit_dmg_stats.volley == [0, approx(292.856948), 0, approx(153.401258)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=2.2))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(257.525014), 0, approx(134.894055)]
    assert api_module_dmg_stats.volley == [0, approx(292.856948), 0, approx(153.401258)]
    # Verification - after 15th hit, which is about mid-spool
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=32.8))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(203.604671), 0, approx(106.650066)]
    assert api_fleet_dmg_stats.volley == [0, approx(561.081068), 0, approx(293.899607)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=32.8))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(203.604671), 0, approx(106.650066)]
    assert api_fit_dmg_stats.volley == [0, approx(561.081068), 0, approx(293.899607)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=32.8))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(203.604671), 0, approx(106.650066)]
    assert api_module_dmg_stats.volley == [0, approx(561.081068), 0, approx(293.899607)]
    # Verification - after 30th hit, which is almost full spool
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=65.5))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(265.549818), 0, approx(139.097524)]
    assert api_fleet_dmg_stats.volley == [0, approx(848.464054), 0, approx(444.433552)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=65.5))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(265.549818), 0, approx(139.097524)]
    assert api_fit_dmg_stats.volley == [0, approx(848.464054), 0, approx(444.433552)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=65.5))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(265.549818), 0, approx(139.097524)]
    assert api_module_dmg_stats.volley == [0, approx(848.464054), 0, approx(444.433552)]
    # Verification - after 31st hit, which is full spool
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=67.6))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(269.952953), 0, approx(141.403928)]
    assert api_fleet_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=67.6))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(269.952953), 0, approx(141.403928)]
    assert api_fit_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=67.6))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(269.952953), 0, approx(141.403928)]
    assert api_module_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    # Verification - just before reload starts
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1089.9))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(384.009785), 0, approx(201.147983)]
    assert api_fleet_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1089.9))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(384.009785), 0, approx(201.147983)]
    assert api_fit_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=1089.9))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(384.009785), 0, approx(201.147983)]
    assert api_module_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    # Verification - just before reload completes
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1090.9))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(383.657773), 0, approx(200.963595)]
    assert api_fleet_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1090.9))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(383.657773), 0, approx(200.963595)]
    assert api_fit_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=1090.9))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(383.657773), 0, approx(200.963595)]
    assert api_module_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    # Verification - after reload is done and another laser shot
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1091.1))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(383.838294), 0, approx(201.058154)]
    assert api_fleet_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1091.1))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(383.838294), 0, approx(201.058154)]
    assert api_fit_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=1091.1))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(383.838294), 0, approx(201.058154)]
    assert api_module_dmg_stats.volley == [0, approx(855.306506), 0, approx(448.017694)]


def test_crit(client, consts):
    # Test crit flag and its combination with other features
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
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification - burst time
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=[
        StatsOptionFitDmg(time=StatTimeBurst()),
        StatsOptionFitDmg(time=StatTimeBurst(), crits=consts.ApiStatCrits.exclude),
        StatsOptionFitDmg(time=StatTimeBurst(), crits=consts.ApiStatCrits.include)]))
    api_fleet_stats_default, api_fleet_stats_excluded, api_fleet_stats_included = api_fleet_stats.dmg
    assert api_fleet_stats_default.dps == [0, approx(266.165107), 0, approx(139.419818)]
    assert api_fleet_stats_default.volley == [0, approx(580.239934), 0, approx(303.935203)]
    assert api_fleet_stats_excluded.dps == [0, approx(262.218716), 0, approx(137.352661)]
    assert api_fleet_stats_excluded.volley == [0, approx(571.6368), 0, approx(299.4288)]
    assert api_fleet_stats_included.dps == [0, approx(266.165107), 0, approx(139.419818)]
    assert api_fleet_stats_included.volley == [0, approx(580.239934), 0, approx(303.935203)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=[
        StatsOptionFitDmg(time=StatTimeBurst()),
        StatsOptionFitDmg(time=StatTimeBurst(), crits=consts.ApiStatCrits.exclude),
        StatsOptionFitDmg(time=StatTimeBurst(), crits=consts.ApiStatCrits.include)]))
    api_fit_stats_default, api_fit_stats_excluded, api_fit_stats_included = api_fit_stats.dmg
    assert api_fit_stats_default.dps == [0, approx(266.165107), 0, approx(139.419818)]
    assert api_fit_stats_default.volley == [0, approx(580.239934), 0, approx(303.935203)]
    assert api_fit_stats_excluded.dps == [0, approx(262.218716), 0, approx(137.352661)]
    assert api_fit_stats_excluded.volley == [0, approx(571.6368), 0, approx(299.4288)]
    assert api_fit_stats_included.dps == [0, approx(266.165107), 0, approx(139.419818)]
    assert api_fit_stats_included.volley == [0, approx(580.239934), 0, approx(303.935203)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=[
        StatsOptionItemDmg(time=StatTimeBurst()),
        StatsOptionItemDmg(time=StatTimeBurst(), crits=consts.ApiStatCrits.exclude),
        StatsOptionItemDmg(time=StatTimeBurst(), crits=consts.ApiStatCrits.include)]))
    api_module_stats_default, api_module_stats_excluded, api_module_stats_included = api_module_stats.dmg
    assert api_module_stats_default.dps == [0, approx(266.165107), 0, approx(139.419818)]
    assert api_module_stats_default.volley == [0, approx(580.239934), 0, approx(303.935203)]
    assert api_module_stats_excluded.dps == [0, approx(262.218716), 0, approx(137.352661)]
    assert api_module_stats_excluded.volley == [0, approx(571.6368), 0, approx(299.4288)]
    assert api_module_stats_included.dps == [0, approx(266.165107), 0, approx(139.419818)]
    assert api_module_stats_included.volley == [0, approx(580.239934), 0, approx(303.935203)]
    # Verification - looped time
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=[
        StatsOptionFitDmg(time=StatTimeSim(time=None)),
        StatsOptionFitDmg(time=StatTimeSim(time=None), crits=consts.ApiStatCrits.exclude),
        StatsOptionFitDmg(time=StatTimeSim(time=None), crits=consts.ApiStatCrits.include)]))
    api_fleet_stats_default, api_fleet_stats_excluded, api_fleet_stats_included = api_fleet_stats.dmg
    assert api_fleet_stats_default.dps == [0, approx(383.622607), 0, approx(200.945175)]
    assert api_fleet_stats_default.volley == [0, approx(855.306506), 0, approx(448.017694)]
    assert api_fleet_stats_excluded.dps == [0, approx(377.93469), 0, approx(197.96579)]
    assert api_fleet_stats_excluded.volley == [0, approx(842.625), 0, approx(441.375)]
    assert api_fleet_stats_included.dps == [0, approx(383.622607), 0, approx(200.945175)]
    assert api_fleet_stats_included.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=[
        StatsOptionFitDmg(time=StatTimeSim(time=None)),
        StatsOptionFitDmg(time=StatTimeSim(time=None), crits=consts.ApiStatCrits.exclude),
        StatsOptionFitDmg(time=StatTimeSim(time=None), crits=consts.ApiStatCrits.include)]))
    api_fit_stats_default, api_fit_stats_excluded, api_fit_stats_included = api_fit_stats.dmg
    assert api_fit_stats_default.dps == [0, approx(383.622607), 0, approx(200.945175)]
    assert api_fit_stats_default.volley == [0, approx(855.306506), 0, approx(448.017694)]
    assert api_fit_stats_excluded.dps == [0, approx(377.93469), 0, approx(197.96579)]
    assert api_fit_stats_excluded.volley == [0, approx(842.625), 0, approx(441.375)]
    assert api_fit_stats_included.dps == [0, approx(383.622607), 0, approx(200.945175)]
    assert api_fit_stats_included.volley == [0, approx(855.306506), 0, approx(448.017694)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=[
        StatsOptionItemDmg(time=StatTimeSim(time=None)),
        StatsOptionItemDmg(time=StatTimeSim(time=None), crits=consts.ApiStatCrits.exclude),
        StatsOptionItemDmg(time=StatTimeSim(time=None), crits=consts.ApiStatCrits.include)]))
    api_module_stats_default, api_module_stats_excluded, api_module_stats_included = api_module_stats.dmg
    assert api_module_stats_default.dps == [0, approx(383.622607), 0, approx(200.945175)]
    assert api_module_stats_default.volley == [0, approx(855.306506), 0, approx(448.017694)]
    assert api_module_stats_excluded.dps == [0, approx(377.93469), 0, approx(197.96579)]
    assert api_module_stats_excluded.volley == [0, approx(842.625), 0, approx(441.375)]
    assert api_module_stats_included.dps == [0, approx(383.622607), 0, approx(200.945175)]
    assert api_module_stats_included.volley == [0, approx(855.306506), 0, approx(448.017694)]
    # Verification - specific time
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=[
        StatsOptionFitDmg(time=StatTimeSim(time=10)),
        StatsOptionFitDmg(time=StatTimeSim(time=10), crits=consts.ApiStatCrits.exclude),
        StatsOptionFitDmg(time=StatTimeSim(time=10), crits=consts.ApiStatCrits.include)]))
    api_fleet_stats_default, api_fleet_stats_excluded, api_fleet_stats_included = api_fleet_stats.dmg
    assert api_fleet_stats_default.dps == [0, approx(156.007907), 0, approx(81.718427)]
    assert api_fleet_stats_default.volley == [0, approx(350.333545), 0, approx(183.508047)]
    assert api_fleet_stats_excluded.dps == [0, approx(153.6948), 0, approx(80.5068)]
    assert api_fleet_stats_excluded.volley == [0, approx(345.1392), 0, approx(180.7872)]
    assert api_fleet_stats_included.dps == [0, approx(156.007907), 0, approx(81.718427)]
    assert api_fleet_stats_included.volley == [0, approx(350.333545), 0, approx(183.508047)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=[
        StatsOptionFitDmg(time=StatTimeSim(time=10)),
        StatsOptionFitDmg(time=StatTimeSim(time=10), crits=consts.ApiStatCrits.exclude),
        StatsOptionFitDmg(time=StatTimeSim(time=10), crits=consts.ApiStatCrits.include)]))
    api_fit_stats_default, api_fit_stats_excluded, api_fit_stats_included = api_fit_stats.dmg
    assert api_fit_stats_default.dps == [0, approx(156.007907), 0, approx(81.718427)]
    assert api_fit_stats_default.volley == [0, approx(350.333545), 0, approx(183.508047)]
    assert api_fit_stats_excluded.dps == [0, approx(153.6948), 0, approx(80.5068)]
    assert api_fit_stats_excluded.volley == [0, approx(345.1392), 0, approx(180.7872)]
    assert api_fit_stats_included.dps == [0, approx(156.007907), 0, approx(81.718427)]
    assert api_fit_stats_included.volley == [0, approx(350.333545), 0, approx(183.508047)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=[
        StatsOptionItemDmg(time=StatTimeSim(time=10)),
        StatsOptionItemDmg(time=StatTimeSim(time=10), crits=consts.ApiStatCrits.exclude),
        StatsOptionItemDmg(time=StatTimeSim(time=10), crits=consts.ApiStatCrits.include)]))
    api_module_stats_default, api_module_stats_excluded, api_module_stats_included = api_module_stats.dmg
    assert api_module_stats_default.dps == [0, approx(156.007907), 0, approx(81.718427)]
    assert api_module_stats_default.volley == [0, approx(350.333545), 0, approx(183.508047)]
    assert api_module_stats_excluded.dps == [0, approx(153.6948), 0, approx(80.5068)]
    assert api_module_stats_excluded.volley == [0, approx(345.1392), 0, approx(180.7872)]
    assert api_module_stats_included.dps == [0, approx(156.007907), 0, approx(81.718427)]
    assert api_module_stats_included.volley == [0, approx(350.333545), 0, approx(183.508047)]


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
