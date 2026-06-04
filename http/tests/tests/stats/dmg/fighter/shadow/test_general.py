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
from tests.stats.dmg import make_eve_fighter_shadow, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(200, 200, 0, 0), prm_dmg_mult=3.515625, prm_cycle_time=3500,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000,
        sq_size=6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(4218.75), approx(4218.75), 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_fit_dmg_stats.volley == [approx(4218.75), approx(4218.75), 0, 0]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_fighter_dmg_stats.volley == [approx(4218.75), approx(4218.75), 0, 0]
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
    assert api_fighter_dmg_ignored.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_fighter_dmg_ignored.volley == [approx(4218.75), approx(4218.75), 0, 0]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging, abilities={eve_basic_info.kamikaze_abil_id: True})
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fleet_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fit_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
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
    assert api_fighter_dmg_ignored.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_dmg_ignored.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging, abilities={eve_basic_info.atkm_abil_id: False})
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fleet_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fit_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
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
    assert api_fighter_dmg_ignored.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_dmg_ignored.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging, abilities={eve_basic_info.missiles_abil_id: False})
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fleet_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fit_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
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
    assert api_fighter_dmg_ignored.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_dmg_ignored.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(200, 200, 0, 0), prm_dmg_mult=3.515625, prm_cycle_time=3500,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000,
        sq_size=6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1_fighter1 = api_fit1.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit1_fighter2 = api_fit1.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit2 = api_sol.create_fit()
    api_fit2_fighter = api_fit2.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(3616.071429), approx(3616.071429), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(12656.25), approx(12656.25), 0, 0]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [approx(2410.714286), approx(2410.714286), 0, 0]
    assert api_fit1_dmg_stats.volley == [approx(8437.5), approx(8437.5), 0, 0]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_fit2_dmg_stats.volley == [approx(4218.75), approx(4218.75), 0, 0]
    # Action
    api_fit1_fighter1.change_fighter(abilities={consts.EveAbil.true_sacrifice: True})
    api_fit1_fighter2.change_fighter(abilities={consts.EveAbil.true_sacrifice: True})
    api_fit2_fighter.change_fighter(abilities={consts.EveAbil.true_sacrifice: True})
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(90000), approx(90000), approx(90000), approx(90000)]
    assert api_fleet_dmg_stats.volley == [approx(900000), approx(900000), approx(900000), approx(900000)]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [approx(60000), approx(60000), approx(60000), approx(60000)]
    assert api_fit1_dmg_stats.volley == [approx(600000), approx(600000), approx(600000), approx(600000)]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fit2_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(200, 200, 0, 0), prm_dmg_mult=3.515625, prm_cycle_time=3500,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000,
        speed=1250, sq_size=6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, minion_mobile=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, minion_mobile=True))])))
    api_fleet_dmg_default, api_fleet_dmg_disabled, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_fleet_dmg_default.volley == [approx(4218.75), approx(4218.75), 0, 0]
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_fleet_dmg_enabled.volley == [approx(4218.75), approx(4218.75), 0, 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, minion_mobile=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, minion_mobile=True))])))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_fit_dmg_default.volley == [approx(4218.75), approx(4218.75), 0, 0]
    assert api_fit_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_fit_dmg_enabled.volley == [approx(4218.75), approx(4218.75), 0, 0]
    # Action
    api_fighter.change_fighter(abilities={consts.EveAbil.true_sacrifice: True})
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, minion_mobile=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, minion_mobile=True))])))
    api_fleet_dmg_default, api_fleet_dmg_disabled, api_fleet_dmg_enabled = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fleet_dmg_default.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fleet_dmg_enabled.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fleet_dmg_enabled.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, minion_mobile=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, minion_mobile=True))])))
    api_fit_dmg_default, api_fit_dmg_disabled, api_fit_dmg_enabled = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fit_dmg_default.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    assert api_fit_dmg_disabled.dps == [0, 0, 0, 0]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, 0]
    assert api_fit_dmg_enabled.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fit_dmg_enabled.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]


def test_count_override(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(200, 200, 0, 0), prm_dmg_mult=3.515625, prm_cycle_time=3500,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000,
        sq_size=6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging, count=4)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(803.571429), approx(803.571429), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(2812.5), approx(2812.5), 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(803.571429), approx(803.571429), 0, 0]
    assert api_fit_dmg_stats.volley == [approx(2812.5), approx(2812.5), 0, 0]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(803.571429), approx(803.571429), 0, 0]
    assert api_fighter_dmg_stats.volley == [approx(2812.5), approx(2812.5), 0, 0]
    # Action
    api_fighter.change_fighter(abilities={consts.EveAbil.true_sacrifice: True})
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(20000), approx(20000), approx(20000), approx(20000)]
    assert api_fleet_dmg_stats.volley == [approx(200000), approx(200000), approx(200000), approx(200000)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(20000), approx(20000), approx(20000), approx(20000)]
    assert api_fit_dmg_stats.volley == [approx(200000), approx(200000), approx(200000), approx(200000)]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(20000), approx(20000), approx(20000), approx(20000)]
    assert api_fighter_dmg_stats.volley == [approx(200000), approx(200000), approx(200000), approx(200000)]
    # Action
    api_fighter.change_fighter(abilities={consts.EveAbil.true_sacrifice: False}, count=8)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(1607.142857), approx(1607.142857), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(5625), approx(5625), 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(1607.142857), approx(1607.142857), 0, 0]
    assert api_fit_dmg_stats.volley == [approx(5625), approx(5625), 0, 0]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(1607.142857), approx(1607.142857), 0, 0]
    assert api_fighter_dmg_stats.volley == [approx(5625), approx(5625), 0, 0]
    # Action
    api_fighter.change_fighter(abilities={consts.EveAbil.true_sacrifice: True})
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(40000), approx(40000), approx(40000), approx(40000)]
    assert api_fleet_dmg_stats.volley == [approx(400000), approx(400000), approx(400000), approx(400000)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [approx(40000), approx(40000), approx(40000), approx(40000)]
    assert api_fit_dmg_stats.volley == [approx(400000), approx(400000), approx(400000), approx(400000)]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [approx(40000), approx(40000), approx(40000), approx(40000)]
    assert api_fighter_dmg_stats.volley == [approx(400000), approx(400000), approx(400000), approx(400000)]
