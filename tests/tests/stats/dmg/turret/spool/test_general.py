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
    assert api_fleet_dmg_stats.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_fleet_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_fit_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_module_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
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
    assert api_module_dmg_ignored.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_module_dmg_ignored.volley == [0, approx(859.4775), 0, approx(450.2025)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_fleet_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_fit_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_module_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]


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
    assert api_fleet_dmg_stats.dps == [0, approx(1239.08945), 0, approx(807.28555)]
    assert api_fleet_dmg_stats.volley == [0, approx(2701.215), 0, approx(1759.8825)]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [0, approx(844.833716), 0, approx(600.770642)]
    assert api_fit1_dmg_stats.volley == [0, approx(1841.7375), 0, approx(1309.68)]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_fit2_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]


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
    assert api_fleet_dmg_default.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_fleet_dmg_default.volley == [0, approx(859.4775), 0, approx(450.2025)]
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_fleet_dmg_enabled.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=[
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, turret=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, turret=True))]))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_fit_dmg_default.volley == [0, approx(859.4775), 0, approx(450.2025)]
    assert api_fit_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_fit_dmg_enabled.volley == [0, approx(859.4775), 0, approx(450.2025)]


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
    assert api_fleet_dmg_pre.dps == [0, approx(126.161835), 0, approx(66.084771)]
    assert api_fleet_dmg_pre.volley == [0, approx(275.0328), 0, approx(144.0648)]
    assert api_fleet_dmg_default.dps == [0, approx(267.46309), 0, approx(140.099714)]
    assert api_fleet_dmg_default.volley == [0, approx(583.069536), 0, approx(305.417376)]
    assert api_fleet_dmg_full.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_fleet_dmg_full.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=[
        StatsOptionFitDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionFitDmg(time=StatTimeBurst()),
        StatsOptionFitDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))]))
    api_fit_dmg_pre, api_fit_dmg_default, api_fit_dmg_full = api_fit_stats.dmg
    assert api_fit_dmg_pre.dps == [0, approx(126.161835), 0, approx(66.084771)]
    assert api_fit_dmg_pre.volley == [0, approx(275.0328), 0, approx(144.0648)]
    assert api_fit_dmg_default.dps == [0, approx(267.46309), 0, approx(140.099714)]
    assert api_fit_dmg_default.volley == [0, approx(583.069536), 0, approx(305.417376)]
    assert api_fit_dmg_full.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_fit_dmg_full.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=[
        StatsOptionItemDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionItemDmg(time=StatTimeBurst()),
        StatsOptionItemDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))]))
    api_module_dmg_pre, api_module_dmg_default, api_module_dmg_full = api_module_stats.dmg
    assert api_module_dmg_pre.dps == [0, approx(126.161835), 0, approx(66.084771)]
    assert api_module_dmg_pre.volley == [0, approx(275.0328), 0, approx(144.0648)]
    assert api_module_dmg_default.dps == [0, approx(267.46309), 0, approx(140.099714)]
    assert api_module_dmg_default.volley == [0, approx(583.069536), 0, approx(305.417376)]
    assert api_module_dmg_full.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_module_dmg_full.volley == [0, approx(859.4775), 0, approx(450.2025)]
    # Action
    api_module.change_module(spool=Spool.spool_scale_to_api(val=0.7))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=[
        StatsOptionFitDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionFitDmg(time=StatTimeBurst()),
        StatsOptionFitDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))]))
    api_fleet_dmg_pre, api_fleet_dmg_default, api_fleet_dmg_full = api_fleet_stats.dmg
    assert api_fleet_dmg_pre.dps == [0, approx(126.161835), 0, approx(66.084771)]
    assert api_fleet_dmg_pre.volley == [0, approx(275.0328), 0, approx(144.0648)]
    assert api_fleet_dmg_default.dps == [0, approx(320.451061), 0, approx(167.855317)]
    assert api_fleet_dmg_default.volley == [0, approx(698.583312), 0, approx(365.924592)]
    assert api_fleet_dmg_full.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_fleet_dmg_full.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=[
        StatsOptionFitDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionFitDmg(time=StatTimeBurst()),
        StatsOptionFitDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))]))
    api_fit_dmg_pre, api_fit_dmg_default, api_fit_dmg_full = api_fit_stats.dmg
    assert api_fit_dmg_pre.dps == [0, approx(126.161835), 0, approx(66.084771)]
    assert api_fit_dmg_pre.volley == [0, approx(275.0328), 0, approx(144.0648)]
    assert api_fit_dmg_default.dps == [0, approx(320.451061), 0, approx(167.855317)]
    assert api_fit_dmg_default.volley == [0, approx(698.583312), 0, approx(365.924592)]
    assert api_fit_dmg_full.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_fit_dmg_full.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(dmg=[
        StatsOptionItemDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=0))),
        StatsOptionItemDmg(time=StatTimeBurst()),
        StatsOptionItemDmg(time=StatTimeBurst(spool=Spool.spool_scale_to_api(val=1)))]))
    api_module_dmg_pre, api_module_dmg_default, api_module_dmg_full = api_module_stats.dmg
    assert api_module_dmg_pre.dps == [0, approx(126.161835), 0, approx(66.084771)]
    assert api_module_dmg_default.dps == [0, approx(320.451061), 0, approx(167.855317)]
    assert api_module_dmg_full.dps == [0, approx(394.255734), 0, approx(206.514908)]
    assert api_module_dmg_pre.volley == [0, approx(275.0328), 0, approx(144.0648)]
    assert api_module_dmg_default.volley == [0, approx(698.583312), 0, approx(365.924592)]
    assert api_module_dmg_full.volley == [0, approx(859.4775), 0, approx(450.2025)]


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
    assert api_fleet_dmg_stats.dps == [0, approx(385.493384), 0, approx(201.925106)]
    assert api_fleet_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=None))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(385.493384), 0, approx(201.925106)]
    assert api_fit_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=None))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(385.493384), 0, approx(201.925106)]
    assert api_module_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    # Verification - just after first hit landed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(275.0328), 0, approx(144.0648)]
    assert api_fleet_dmg_stats.volley == [0, approx(275.0328), 0, approx(144.0648)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(275.0328), 0, approx(144.0648)]
    assert api_fit_dmg_stats.volley == [0, approx(275.0328), 0, approx(144.0648)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=1))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(275.0328), 0, approx(144.0648)]
    assert api_module_dmg_stats.volley == [0, approx(275.0328), 0, approx(144.0648)]
    # Verification - just before second hit
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=2.1))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(130.968), 0, approx(68.602286)]
    assert api_fleet_dmg_stats.volley == [0, approx(275.0328), 0, approx(144.0648)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=2.1))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(130.968), 0, approx(68.602286)]
    assert api_fit_dmg_stats.volley == [0, approx(275.0328), 0, approx(144.0648)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=2.1))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(130.968), 0, approx(68.602286)]
    assert api_module_dmg_stats.volley == [0, approx(275.0328), 0, approx(144.0648)]
    # Verification - just after second hit (higher volley was recorded, higher dps)
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=2.2))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(258.780862), 0, approx(135.55188)]
    assert api_fleet_dmg_stats.volley == [0, approx(294.285096), 0, approx(154.149336)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=2.2))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(258.780862), 0, approx(135.55188)]
    assert api_fit_dmg_stats.volley == [0, approx(294.285096), 0, approx(154.149336)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=2.2))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(258.780862), 0, approx(135.55188)]
    assert api_module_dmg_stats.volley == [0, approx(294.285096), 0, approx(154.149336)]
    # Verification - after 15th hit, which is about mid-spool
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=32.8))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(204.597571), 0, approx(107.170156)]
    assert api_fleet_dmg_stats.volley == [0, approx(563.81724), 0, approx(295.33284)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=32.8))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(204.597571), 0, approx(107.170156)]
    assert api_fit_dmg_stats.volley == [0, approx(563.81724), 0, approx(295.33284)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=32.8))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(204.597571), 0, approx(107.170156)]
    assert api_module_dmg_stats.volley == [0, approx(563.81724), 0, approx(295.33284)]
    # Verification - after 30th hit, which is almost full spool
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=65.5))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(266.844801), 0, approx(139.775848)]
    assert api_fleet_dmg_stats.volley == [0, approx(852.60168), 0, approx(446.60088)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=65.5))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(266.844801), 0, approx(139.775848)]
    assert api_fit_dmg_stats.volley == [0, approx(852.60168), 0, approx(446.60088)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=65.5))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(266.844801), 0, approx(139.775848)]
    assert api_module_dmg_stats.volley == [0, approx(852.60168), 0, approx(446.60088)]
    # Verification - after 31st hit, which is full spool
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=67.6))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(271.269407), 0, approx(142.093499)]
    assert api_fleet_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=67.6))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(271.269407), 0, approx(142.093499)]
    assert api_fit_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=67.6))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(271.269407), 0, approx(142.093499)]
    assert api_module_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    # Verification - just before reload starts
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1089.9))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(385.88245), 0, approx(202.128902)]
    assert api_fleet_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1089.9))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(385.88245), 0, approx(202.128902)]
    assert api_fit_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=1089.9))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(385.88245), 0, approx(202.128902)]
    assert api_module_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    # Verification - just before reload completes
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1090.9))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(385.528721), 0, approx(201.943616)]
    assert api_fleet_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1090.9))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(385.528721), 0, approx(201.943616)]
    assert api_fit_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=1090.9))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(385.528721), 0, approx(201.943616)]
    assert api_module_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    # Verification - after reload is done and another laser shot
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1091.1))])).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, approx(385.710123), 0, approx(202.038636)]
    assert api_fleet_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=[StatsOptionFitDmg(time=StatTimeSim(time=1091.1))])).dmg.one()
    assert api_fit_dmg_stats.dps == [0, approx(385.710123), 0, approx(202.038636)]
    assert api_fit_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]
    api_module_dmg_stats = api_module.get_stats(options=ItemStatsOptions(
        dmg=[StatsOptionItemDmg(time=StatTimeSim(time=1091.1))])).dmg.one()
    assert api_module_dmg_stats.dps == [0, approx(385.710123), 0, approx(202.038636)]
    assert api_module_dmg_stats.volley == [0, approx(859.4775), 0, approx(450.2025)]


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
