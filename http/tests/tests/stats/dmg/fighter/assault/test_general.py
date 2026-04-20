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
from tests.stats.dmg import make_eve_fighter_assault, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.34375, prm_cycle_time=5000,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.34375, sec_cycle_time=14000,
        sq_size=6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(303.75), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(1518.75), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(303.75), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(1518.75), 0, 0, 0]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(303.75), 0, 0, 0]
    assert api_fighter_dmg_stats.volley == [approx(1518.75), 0, 0, 0]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_fighter_dmg_normal, api_fighter_dmg_ignored = api_fighter_stats.dmg
    assert api_fighter_dmg_normal.dps == [0, 0, 0, 0]
    assert api_fighter_dmg_normal.volley == [0, 0, 0, 0]
    assert api_fighter_dmg_ignored.dps == [approx(303.75), 0, 0, 0]
    assert api_fighter_dmg_ignored.volley == [approx(1518.75), 0, 0, 0]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging, abilities={eve_basic_info.missiles_abil_id: True})
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(501.127232), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(4282.03125), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(501.127232), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(4282.03125), 0, 0, 0]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(501.127232), 0, 0, 0]
    assert api_fighter_dmg_stats.volley == [approx(4282.03125), 0, 0, 0]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_bay)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_fighter_dmg_normal, api_fighter_dmg_ignored = api_fighter_stats.dmg
    assert api_fighter_dmg_normal.dps == [0, 0, 0, 0]
    assert api_fighter_dmg_normal.volley == [0, 0, 0, 0]
    assert api_fighter_dmg_ignored.dps == [approx(501.127232), 0, 0, 0]
    assert api_fighter_dmg_ignored.volley == [approx(4282.03125), 0, 0, 0]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging, abilities={eve_basic_info.atkm_abil_id: False})
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(197.377232), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(2763.28125), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(197.377232), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(2763.28125), 0, 0, 0]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(197.377232), 0, 0, 0]
    assert api_fighter_dmg_stats.volley == [approx(2763.28125), 0, 0, 0]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_fighter_dmg_normal, api_fighter_dmg_ignored = api_fighter_stats.dmg
    assert api_fighter_dmg_normal.dps == [0, 0, 0, 0]
    assert api_fighter_dmg_normal.volley == [0, 0, 0, 0]
    assert api_fighter_dmg_ignored.dps == [approx(197.377232), 0, 0, 0]
    assert api_fighter_dmg_ignored.volley == [approx(2763.28125), 0, 0, 0]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging, abilities={eve_basic_info.missiles_abil_id: False})
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_fighter_dmg_normal, api_fighter_dmg_ignored = api_fighter_stats.dmg
    assert api_fighter_dmg_normal.dps == [0, 0, 0, 0]
    assert api_fighter_dmg_normal.volley == [0, 0, 0, 0]
    assert api_fighter_dmg_ignored.dps == [0, 0, 0, 0]
    assert api_fighter_dmg_ignored.volley == [0, 0, 0, 0]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.34375, prm_cycle_time=5000,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.34375, sec_cycle_time=14000,
        sq_size=6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit1.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_fit2 = api_sol.create_fit()
    api_fit2.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(1108.627232), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(7319.53125), 0, 0, 0]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [approx(804.877232), 0, 0, 0]
    assert api_fit1_dmg_stats.volley == [approx(5800.78125), 0, 0, 0]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [approx(303.75), 0, 0, 0]
    assert api_fit2_dmg_stats.volley == [approx(1518.75), 0, 0, 0]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.34375, prm_cycle_time=5000,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.34375, sec_cycle_time=14000,
        sq_size=6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, minion_mobile=True)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, minion_mobile=False))])))
    api_fleet_dmg_default, api_fleet_dmg_disabled, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [approx(303.75), 0, 0, 0]
    assert api_fleet_dmg_default.volley == [approx(1518.75), 0, 0, 0]
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [approx(303.75), 0, 0, 0]
    assert api_fleet_dmg_enabled.volley == [approx(1518.75), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, minion_mobile=True)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, minion_mobile=False))])))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [approx(303.75), 0, 0, 0]
    assert api_fit_dmg_default.volley == [approx(1518.75), 0, 0, 0]
    assert api_fit_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [approx(303.75), 0, 0, 0]
    assert api_fit_dmg_enabled.volley == [approx(1518.75), 0, 0, 0]


def test_time(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.34375, prm_cycle_time=5000,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.34375, sec_cycle_time=14000,
        sec_charge_count=18, sec_charge_rearm_time=4,
        sq_size=6, refuel_duration=5000)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification - burst stats
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(501.127232), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(4282.03125), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(501.127232), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(4282.03125), 0, 0, 0]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(501.127232), 0, 0, 0]
    assert api_fighter_dmg_stats.volley == [approx(4282.03125), 0, 0, 0]
    # Verification - sim without time. When rearm is disabled, secondary ability is ignored, since
    # it has infinite charges. When rearm is enabled, fighter is recalled when last cycle of
    # secondary ability completes, and primary ability cycles as many full cycles as in-space
    # duration can fit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=None, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fleet_dmg_disabled, api_fleet_dmg_rearm = api_fleet_stats.dmg
    assert api_fleet_dmg_disabled.dps == [approx(303.75), 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [approx(1518.75), 0, 0, 0]
    assert api_fleet_dmg_rearm.dps == [approx(379.6875), 0, 0, 0]
    assert api_fleet_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=None, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fit_dmg_disabled, api_fit_dmg_rearm = api_fit_stats.dmg
    assert api_fit_dmg_disabled.dps == [approx(303.75), 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [approx(1518.75), 0, 0, 0]
    assert api_fit_dmg_rearm.dps == [approx(379.6875), 0, 0, 0]
    assert api_fit_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(time_options=StatTimeSim(time=None, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionItemDmg(time_options=StatTimeSim(time=None, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fighter_dmg_disabled, api_fighter_dmg_rearm = api_fighter_stats.dmg
    assert api_fighter_dmg_disabled.dps == [approx(303.75), 0, 0, 0]
    assert api_fighter_dmg_disabled.volley == [approx(1518.75), 0, 0, 0]
    assert api_fighter_dmg_rearm.dps == [approx(379.6875), 0, 0, 0]
    assert api_fighter_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]
    # Verification - time after first volleys of both abilities were launched
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(4282.03125), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(4282.03125), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(4282.03125), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(4282.03125), 0, 0, 0]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(4282.03125), 0, 0, 0]
    assert api_fighter_dmg_stats.volley == [approx(4282.03125), 0, 0, 0]
    # Verification - time before last primary cycle is completed in rearm mode, numbers match
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=249, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=249, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fleet_dmg_disabled, api_fleet_dmg_rearm = api_fleet_stats.dmg
    assert api_fleet_dmg_disabled.dps == [approx(504.725151), 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [approx(4282.03125), 0, 0, 0]
    assert api_fleet_dmg_rearm.dps == [approx(504.725151), 0, 0, 0]
    assert api_fleet_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=249, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=249, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fit_dmg_disabled, api_fit_dmg_rearm = api_fit_stats.dmg
    assert api_fit_dmg_disabled.dps == [approx(504.725151), 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [approx(4282.03125), 0, 0, 0]
    assert api_fit_dmg_rearm.dps == [approx(504.725151), 0, 0, 0]
    assert api_fit_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(time_options=StatTimeSim(time=249, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionItemDmg(time_options=StatTimeSim(time=249, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fighter_dmg_disabled, api_fighter_dmg_rearm = api_fighter_stats.dmg
    assert api_fighter_dmg_disabled.dps == [approx(504.725151), 0, 0, 0]
    assert api_fighter_dmg_disabled.volley == [approx(4282.03125), 0, 0, 0]
    assert api_fighter_dmg_rearm.dps == [approx(504.725151), 0, 0, 0]
    assert api_fighter_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]
    # Verification - time after last primary cycle is completed in rearm mode. Numbers start
    # diverging, since incomplete cycle of primary ability is not allowed to start
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=251, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=251, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fleet_dmg_disabled, api_fleet_dmg_rearm = api_fleet_stats.dmg
    assert api_fleet_dmg_disabled.dps == [approx(506.754233), 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [approx(4282.03125), 0, 0, 0]
    assert api_fleet_dmg_rearm.dps == [approx(500.703436), 0, 0, 0]
    assert api_fleet_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=251, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=251, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fit_dmg_disabled, api_fit_dmg_rearm = api_fit_stats.dmg
    assert api_fit_dmg_disabled.dps == [approx(506.754233), 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [approx(4282.03125), 0, 0, 0]
    assert api_fit_dmg_rearm.dps == [approx(500.703436), 0, 0, 0]
    assert api_fit_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(time_options=StatTimeSim(time=251, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionItemDmg(time_options=StatTimeSim(time=251, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fighter_dmg_disabled, api_fighter_dmg_rearm = api_fighter_stats.dmg
    assert api_fighter_dmg_disabled.dps == [approx(506.754233), 0, 0, 0]
    assert api_fighter_dmg_disabled.volley == [approx(4282.03125), 0, 0, 0]
    assert api_fighter_dmg_rearm.dps == [approx(500.703436), 0, 0, 0]
    assert api_fighter_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]
    # Verification - time when rearm is about to complete for rearm mode
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=330, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=330, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fleet_dmg_disabled, api_fleet_dmg_rearm = api_fleet_stats.dmg
    assert api_fleet_dmg_disabled.dps == [approx(459.076705), 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [approx(4282.03125), 0, 0, 0]
    assert api_fleet_dmg_rearm.dps == [approx(380.838068), 0, 0, 0]
    assert api_fleet_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=330, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=330, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fit_dmg_disabled, api_fit_dmg_rearm = api_fit_stats.dmg
    assert api_fit_dmg_disabled.dps == [approx(459.076705), 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [approx(4282.03125), 0, 0, 0]
    assert api_fit_dmg_rearm.dps == [approx(380.838068), 0, 0, 0]
    assert api_fit_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(time_options=StatTimeSim(time=330, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionItemDmg(time_options=StatTimeSim(time=330, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fighter_dmg_disabled, api_fighter_dmg_rearm = api_fighter_stats.dmg
    assert api_fighter_dmg_disabled.dps == [approx(459.076705), 0, 0, 0]
    assert api_fighter_dmg_disabled.volley == [approx(4282.03125), 0, 0, 0]
    assert api_fighter_dmg_rearm.dps == [approx(380.838068), 0, 0, 0]
    assert api_fighter_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]
    # Verification - time when rearm is complete for rearm mode and volleys for both abilities are
    # out
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=332, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=332, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fleet_dmg_disabled, api_fleet_dmg_rearm = api_fleet_stats.dmg
    assert api_fleet_dmg_disabled.dps == [approx(456.311182), 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [approx(4282.03125), 0, 0, 0]
    assert api_fleet_dmg_rearm.dps == [approx(391.441547), 0, 0, 0]
    assert api_fleet_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=332, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=332, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fit_dmg_disabled, api_fit_dmg_rearm = api_fit_stats.dmg
    assert api_fit_dmg_disabled.dps == [approx(456.311182), 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [approx(4282.03125), 0, 0, 0]
    assert api_fit_dmg_rearm.dps == [approx(391.441547), 0, 0, 0]
    assert api_fit_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(time_options=StatTimeSim(time=332, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionItemDmg(time_options=StatTimeSim(time=332, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fighter_dmg_disabled, api_fighter_dmg_rearm = api_fighter_stats.dmg
    assert api_fighter_dmg_disabled.dps == [approx(456.311182), 0, 0, 0]
    assert api_fighter_dmg_disabled.volley == [approx(4282.03125), 0, 0, 0]
    assert api_fighter_dmg_rearm.dps == [approx(391.441547), 0, 0, 0]
    assert api_fighter_dmg_rearm.volley == [approx(4282.03125), 0, 0, 0]


def test_count_override(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.34375, prm_cycle_time=5000,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.34375, sec_cycle_time=14000,
        sq_size=6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging, count=4)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(202.5), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(1012.5), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(202.5), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(1012.5), 0, 0, 0]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(202.5), 0, 0, 0]
    assert api_fighter_dmg_stats.volley == [approx(1012.5), 0, 0, 0]
    # Action
    api_fighter.change_fighter(count=8)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(405), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(2025), 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(405), 0, 0, 0]
    assert api_fit_dmg_stats.volley == [approx(2025), 0, 0, 0]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(405), 0, 0, 0]
    assert api_fighter_dmg_stats.volley == [approx(2025), 0, 0, 0]
