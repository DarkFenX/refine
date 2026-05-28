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
from tests.stats.dmg import make_eve_bomb, make_eve_fighter_lr, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000,
        sq_size=6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_autocharge = api_fighter.autocharges[eve_basic_info.ftr_abil_bomb_effect_id]
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(698.730469)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(5589.84375)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(698.730469)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(5589.84375)]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_fighter_dmg_stats.dps == [0, 0, 0, approx(698.730469)]
    assert api_fighter_dmg_stats.volley == [0, 0, 0, approx(5589.84375)]
    api_autocharge_stats = api_autocharge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_autocharge_dmg_normal, api_autocharge_dmg_ignored = api_autocharge_stats.dmg
    assert api_autocharge_dmg_normal.dps == [0, 0, 0, 0]
    assert api_autocharge_dmg_normal.volley == [0, 0, 0, 0]
    assert api_autocharge_dmg_ignored.dps == [0, 0, 0, 0]
    assert api_autocharge_dmg_ignored.volley == [0, 0, 0, 0]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(include_charges=True),
        StatsOptionItemDmg(include_charges=True, ignore_state=True)])))
    api_fighter_dmg_normal, api_fighter_dmg_ignored = api_fighter_stats.dmg
    assert api_fighter_dmg_normal.dps == [0, 0, 0, 0]
    assert api_fighter_dmg_normal.volley == [0, 0, 0, 0]
    assert api_fighter_dmg_ignored.dps == [0, 0, 0, approx(698.730469)]
    assert api_fighter_dmg_ignored.volley == [0, 0, 0, approx(5589.84375)]
    api_autocharge_stats = api_autocharge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_autocharge_dmg_normal, api_autocharge_dmg_ignored = api_autocharge_stats.dmg
    assert api_autocharge_dmg_normal.dps == [0, 0, 0, 0]
    assert api_autocharge_dmg_normal.volley == [0, 0, 0, 0]
    assert api_autocharge_dmg_ignored.dps == [0, 0, 0, 0]
    assert api_autocharge_dmg_ignored.volley == [0, 0, 0, 0]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging, abilities={eve_basic_info.bomb_abil_id: True})
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(762.730469)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(762.730469)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=True)]))).dmg.one()
    assert api_fighter_dmg_stats.dps == [0, 0, 0, approx(762.730469)]
    assert api_fighter_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    api_autocharge_dmg_stats = api_autocharge.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_autocharge_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_autocharge_dmg_stats.volley == [0, 0, 0, approx(3840)]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_bay)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(include_charges=True),
        StatsOptionItemDmg(include_charges=True, ignore_state=True)])))
    api_fighter_dmg_normal, api_fighter_dmg_ignored = api_fighter_stats.dmg
    assert api_fighter_dmg_normal.dps == [0, 0, 0, 0]
    assert api_fighter_dmg_normal.volley == [0, 0, 0, 0]
    assert api_fighter_dmg_ignored.dps == [0, 0, 0, approx(762.730469)]
    assert api_fighter_dmg_ignored.volley == [0, 0, 0, approx(9429.84375)]
    api_autocharge_stats = api_autocharge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_autocharge_dmg_normal, api_autocharge_dmg_ignored = api_autocharge_stats.dmg
    assert api_autocharge_dmg_normal.dps == [0, 0, 0, 0]
    assert api_autocharge_dmg_normal.volley == [0, 0, 0, 0]
    assert api_autocharge_dmg_ignored.dps == [0, 0, 0, approx(64)]
    assert api_autocharge_dmg_ignored.volley == [0, 0, 0, approx(3840)]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging, abilities={eve_basic_info.atkm_abil_id: False})
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(3840)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(3840)]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=True)]))).dmg.one()
    assert api_fighter_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_dmg_stats.volley == [0, 0, 0, approx(3840)]
    api_autocharge_dmg_stats = api_autocharge.get_stats(options=ItemStatsOptions(dmg=True)).dmg.one()
    assert api_autocharge_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_autocharge_dmg_stats.volley == [0, 0, 0, approx(3840)]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(include_charges=True),
        StatsOptionItemDmg(include_charges=True, ignore_state=True)])))
    api_fighter_dmg_normal, api_fighter_dmg_ignored = api_fighter_stats.dmg
    assert api_fighter_dmg_normal.dps == [0, 0, 0, 0]
    assert api_fighter_dmg_normal.volley == [0, 0, 0, 0]
    assert api_fighter_dmg_ignored.dps == [0, 0, 0, approx(64)]
    assert api_fighter_dmg_ignored.volley == [0, 0, 0, approx(3840)]
    api_autocharge_stats = api_autocharge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(), StatsOptionItemDmg(ignore_state=True)])))
    api_autocharge_dmg_normal, api_autocharge_dmg_ignored = api_autocharge_stats.dmg
    assert api_autocharge_dmg_normal.dps == [0, 0, 0, 0]
    assert api_autocharge_dmg_normal.volley == [0, 0, 0, 0]
    assert api_autocharge_dmg_ignored.dps == [0, 0, 0, approx(64)]
    assert api_autocharge_dmg_ignored.volley == [0, 0, 0, approx(3840)]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging, abilities={eve_basic_info.bomb_abil_id: False})
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=True, ignore_state=True)]))).dmg.one()
    assert api_fighter_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_dmg_stats.volley == [0, 0, 0, 0]
    api_autocharge_dmg_stats = api_autocharge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(ignore_state=True)]))).dmg.one()
    assert api_autocharge_dmg_stats.dps == [0, 0, 0, 0]
    assert api_autocharge_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_fighter.change_fighter(state=consts.ApiMinionState.in_space)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=True, ignore_state=True)]))).dmg.one()
    assert api_fighter_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_dmg_stats.volley == [0, 0, 0, 0]
    api_autocharge_dmg_stats = api_autocharge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(ignore_state=True)]))).dmg.one()
    assert api_autocharge_dmg_stats.dps == [0, 0, 0, 0]
    assert api_autocharge_dmg_stats.volley == [0, 0, 0, 0]


def test_stacking(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000,
        sq_size=6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit1 = api_sol.create_fit()
    api_fit1.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fit1.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True})
    api_fit2 = api_sol.create_fit()
    api_fit2.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True})
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit1.id, api_fit2.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(2224.191406)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(24449.53125)]
    api_fit1_dmg_stats = api_fit1.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit1_dmg_stats.dps == [0, 0, 0, approx(1461.460938)]
    assert api_fit1_dmg_stats.volley == [0, 0, 0, approx(15019.6875)]
    api_fit2_dmg_stats = api_fit2.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit2_dmg_stats.dps == [0, 0, 0, approx(762.730469)]
    assert api_fit2_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]


def test_item_kind(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000,
        speed=950, sq_size=6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True})
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, minion_mobile=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, bomb=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, minion_mobile=True)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, bomb=True))])))
    (api_fleet_dmg_default,
     api_fleet_dmg_no_minion,
     api_fleet_dmg_no_bomb,
     api_fleet_dmg_minion,
     api_fleet_dmg_bomb) = api_fleet_stats.dmg
    assert api_fleet_dmg_default.dps == [0, 0, 0, approx(762.730469)]
    assert api_fleet_dmg_default.volley == [0, 0, 0, approx(9429.84375)]
    assert api_fleet_dmg_no_minion.dps == [0, 0, 0, approx(64)]
    assert api_fleet_dmg_no_minion.volley == [0, 0, 0, approx(3840)]
    assert api_fleet_dmg_no_bomb.dps == [0, 0, 0, approx(698.730469)]
    assert api_fleet_dmg_no_bomb.volley == [0, 0, 0, approx(5589.84375)]
    assert api_fleet_dmg_minion.dps == [0, 0, 0, approx(698.730469)]
    assert api_fleet_dmg_minion.volley == [0, 0, 0, approx(5589.84375)]
    assert api_fleet_dmg_bomb.dps == [0, 0, 0, approx(64)]
    assert api_fleet_dmg_bomb.volley == [0, 0, 0, approx(3840)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, minion_mobile=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=True, bomb=False)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, minion_mobile=True)),
        StatsOptionFitDmg(item_kinds=StatDmgItemKinds(default=False, bomb=True))])))
    (api_fit_dmg_default,
     api_fit_dmg_no_minion,
     api_fit_dmg_no_bomb,
     api_fit_dmg_minion,
     api_fit_dmg_bomb) = api_fit_stats.dmg
    assert api_fit_dmg_default.dps == [0, 0, 0, approx(762.730469)]
    assert api_fit_dmg_default.volley == [0, 0, 0, approx(9429.84375)]
    assert api_fit_dmg_no_minion.dps == [0, 0, 0, approx(64)]
    assert api_fit_dmg_no_minion.volley == [0, 0, 0, approx(3840)]
    assert api_fit_dmg_no_bomb.dps == [0, 0, 0, approx(698.730469)]
    assert api_fit_dmg_no_bomb.volley == [0, 0, 0, approx(5589.84375)]
    assert api_fit_dmg_minion.dps == [0, 0, 0, approx(698.730469)]
    assert api_fit_dmg_minion.volley == [0, 0, 0, approx(5589.84375)]
    assert api_fit_dmg_bomb.dps == [0, 0, 0, approx(64)]
    assert api_fit_dmg_bomb.volley == [0, 0, 0, approx(3840)]


def test_count_override(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000,
        sq_size=6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True},
        count=4)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(508.486979)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(6286.5625)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(508.486979)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(6286.5625)]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=True)]))).dmg.one()
    assert api_fighter_dmg_stats.dps == [0, 0, 0, approx(508.486979)]
    assert api_fighter_dmg_stats.volley == [0, 0, 0, approx(6286.5625)]
    # Action
    api_fighter.change_fighter(count=8)
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(1016.973958)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(12573.125)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(1016.973958)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(12573.125)]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=True)]))).dmg.one()
    assert api_fighter_dmg_stats.dps == [0, 0, 0, approx(1016.973958)]
    assert api_fighter_dmg_stats.volley == [0, 0, 0, approx(12573.125)]
