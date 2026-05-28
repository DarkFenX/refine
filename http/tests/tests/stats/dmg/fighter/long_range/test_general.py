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


def test_include_charges(client, consts):
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
        abilities={eve_basic_info.atkm_abil_id: False, eve_basic_info.bomb_abil_id: True})
    api_autocharge = api_fighter.autocharges[eve_basic_info.ftr_abil_bomb_effect_id]
    # Verification - need to include charges for module to show dps, since it's on-charge effect
    # which deals damage. For charges, this option doesn't do anything
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=False), StatsOptionItemDmg(include_charges=True)])))
    api_fighter_dmg_without, api_fighter_dmg_with = api_fighter_stats.dmg
    assert api_fighter_dmg_without.dps == [0, 0, 0, 0]
    assert api_fighter_dmg_with.dps == [0, 0, 0, approx(64)]
    assert api_fighter_dmg_without.volley == [0, 0, 0, 0]
    assert api_fighter_dmg_with.volley == [0, 0, 0, approx(3840)]
    api_autocharge_stats = api_autocharge.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(include_charges=False), StatsOptionItemDmg(include_charges=True)])))
    api_autocharge_dmg_without, api_autocharge_dmg_with = api_autocharge_stats.dmg
    assert api_autocharge_dmg_without.dps == [0, 0, 0, approx(64)]
    assert api_autocharge_dmg_with.dps == [0, 0, 0, approx(64)]
    assert api_autocharge_dmg_without.volley == [0, 0, 0, approx(3840)]
    assert api_autocharge_dmg_with.volley == [0, 0, 0, approx(3840)]


def test_time(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000, sec_charge_count=5, sec_charge_rearm_time=20,
        refuel_duration=2000, sq_size=6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fighter = api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True})
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification - burst stats
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(762.730469)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeBurst())]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(762.730469)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeBurst(), include_charges=True)]))).dmg.one()
    assert api_fighter_dmg_stats.dps == [0, 0, 0, approx(762.730469)]
    assert api_fighter_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    # Verification - sim without time. When rearm is disabled, secondary ability is ignored, since
    # it has finite charges. When rearm is enabled, fighter is recalled when last cycle of secondary
    # ability completes, and primary ability cycles as many full cycles as in-space duration can fit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=None, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fleet_dmg_disabled, api_fleet_dmg_rearm = api_fleet_stats.dmg
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, approx(698.730469)]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, approx(5589.84375)]
    assert api_fleet_dmg_rearm.dps == [0, 0, 0, approx(573.302135)]
    assert api_fleet_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=None, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fit_dmg_disabled, api_fit_dmg_rearm = api_fit_stats.dmg
    assert api_fit_dmg_disabled.dps == [0, 0, 0, approx(698.730469)]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, approx(5589.84375)]
    assert api_fit_dmg_rearm.dps == [0, 0, 0, approx(573.302135)]
    assert api_fit_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(
            time_options=StatTimeSim(time=None, rearm_minions=consts.ApiRearmMinion.disabled),
            include_charges=True),
        StatsOptionItemDmg(
            time_options=StatTimeSim(time=None, rearm_minions=consts.ApiRearmMinion.on_first_empty),
            include_charges=True)])))
    api_fighter_dmg_disabled, api_fighter_dmg_rearm = api_fighter_stats.dmg
    assert api_fighter_dmg_disabled.dps == [0, 0, 0, approx(698.730469)]
    assert api_fighter_dmg_disabled.volley == [0, 0, 0, approx(5589.84375)]
    assert api_fighter_dmg_rearm.dps == [0, 0, 0, approx(573.302135)]
    assert api_fighter_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]
    # Verification - time after first volleys of both abilities were launched
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(9429.84375)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(time_options=StatTimeSim(time=1))]))).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, approx(9429.84375)]
    assert api_fit_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    api_fighter_dmg_stats = api_fighter.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(time_options=StatTimeSim(time=1), include_charges=True)]))).dmg.one()
    assert api_fighter_dmg_stats.dps == [0, 0, 0, approx(9429.84375)]
    assert api_fighter_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    # Verification - time when one fighter is being rearmed, but the other one still haven't made an
    # additional attack, so numbers match
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=303, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=303, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fleet_dmg_disabled, api_fleet_dmg_rearm = api_fleet_stats.dmg
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, approx(764.402846)]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, approx(9429.84375)]
    assert api_fleet_dmg_rearm.dps == [0, 0, 0, approx(764.402846)]
    assert api_fleet_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=303, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=303, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fit_dmg_disabled, api_fit_dmg_rearm = api_fit_stats.dmg
    assert api_fit_dmg_disabled.dps == [0, 0, 0, approx(764.402846)]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, approx(9429.84375)]
    assert api_fit_dmg_rearm.dps == [0, 0, 0, approx(764.402846)]
    assert api_fit_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(
            time_options=StatTimeSim(time=303, rearm_minions=consts.ApiRearmMinion.disabled),
            include_charges=True),
        StatsOptionItemDmg(
            time_options=StatTimeSim(time=303, rearm_minions=consts.ApiRearmMinion.on_first_empty),
            include_charges=True)])))
    api_fighter_dmg_disabled, api_fighter_dmg_rearm = api_fighter_stats.dmg
    assert api_fighter_dmg_disabled.dps == [0, 0, 0, approx(764.402846)]
    assert api_fighter_dmg_disabled.volley == [0, 0, 0, approx(9429.84375)]
    assert api_fighter_dmg_rearm.dps == [0, 0, 0, approx(764.402846)]
    assert api_fighter_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]
    # Verification - time after fighter is being rearmed in one case, and fires another primary
    # ability shot in the other
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=305, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=305, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fleet_dmg_disabled, api_fleet_dmg_rearm = api_fleet_stats.dmg
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, approx(777.717725)]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, approx(9429.84375)]
    assert api_fleet_dmg_rearm.dps == [0, 0, 0, approx(759.390369)]
    assert api_fleet_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=305, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=305, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fit_dmg_disabled, api_fit_dmg_rearm = api_fit_stats.dmg
    assert api_fit_dmg_disabled.dps == [0, 0, 0, approx(777.717725)]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, approx(9429.84375)]
    assert api_fit_dmg_rearm.dps == [0, 0, 0, approx(759.390369)]
    assert api_fit_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(
            time_options=StatTimeSim(time=305, rearm_minions=consts.ApiRearmMinion.disabled),
            include_charges=True),
        StatsOptionItemDmg(
            time_options=StatTimeSim(time=305, rearm_minions=consts.ApiRearmMinion.on_first_empty),
            include_charges=True)])))
    api_fighter_dmg_disabled, api_fighter_dmg_rearm = api_fighter_stats.dmg
    assert api_fighter_dmg_disabled.dps == [0, 0, 0, approx(777.717725)]
    assert api_fighter_dmg_disabled.volley == [0, 0, 0, approx(9429.84375)]
    assert api_fighter_dmg_rearm.dps == [0, 0, 0, approx(759.390369)]
    assert api_fighter_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]
    # Verification - time when rearm is about to complete for rearm mode
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=403, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=403, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fleet_dmg_disabled, api_fleet_dmg_rearm = api_fleet_stats.dmg
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, approx(755.042261)]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, approx(9429.84375)]
    assert api_fleet_dmg_rearm.dps == [0, 0, 0, approx(574.724721)]
    assert api_fleet_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=403, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=403, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fit_dmg_disabled, api_fit_dmg_rearm = api_fit_stats.dmg
    assert api_fit_dmg_disabled.dps == [0, 0, 0, approx(755.042261)]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, approx(9429.84375)]
    assert api_fit_dmg_rearm.dps == [0, 0, 0, approx(574.724721)]
    assert api_fit_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(
            time_options=StatTimeSim(time=403, rearm_minions=consts.ApiRearmMinion.disabled),
            include_charges=True),
        StatsOptionItemDmg(
            time_options=StatTimeSim(time=403, rearm_minions=consts.ApiRearmMinion.on_first_empty),
            include_charges=True)])))
    api_fighter_dmg_disabled, api_fighter_dmg_rearm = api_fighter_stats.dmg
    assert api_fighter_dmg_disabled.dps == [0, 0, 0, approx(755.042261)]
    assert api_fighter_dmg_disabled.volley == [0, 0, 0, approx(9429.84375)]
    assert api_fighter_dmg_rearm.dps == [0, 0, 0, approx(574.724721)]
    assert api_fighter_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]
    # Verification - time when rearm is complete for rearm mode and volleys for both abilities are
    # out
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=405, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=405, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fleet_dmg_disabled, api_fleet_dmg_rearm = api_fleet_stats.dmg
    assert api_fleet_dmg_disabled.dps == [0, 0, 0, approx(751.313657)]
    assert api_fleet_dmg_disabled.volley == [0, 0, 0, approx(9429.84375)]
    assert api_fleet_dmg_rearm.dps == [0, 0, 0, approx(595.170139)]
    assert api_fleet_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=405, rearm_minions=consts.ApiRearmMinion.disabled)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=405, rearm_minions=consts.ApiRearmMinion.on_first_empty))])))
    api_fit_dmg_disabled, api_fit_dmg_rearm = api_fit_stats.dmg
    assert api_fit_dmg_disabled.dps == [0, 0, 0, approx(751.313657)]
    assert api_fit_dmg_disabled.volley == [0, 0, 0, approx(9429.84375)]
    assert api_fit_dmg_rearm.dps == [0, 0, 0, approx(595.170139)]
    assert api_fit_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]
    api_fighter_stats = api_fighter.get_stats(options=ItemStatsOptions(dmg=(True, [
        StatsOptionItemDmg(
            time_options=StatTimeSim(time=405, rearm_minions=consts.ApiRearmMinion.disabled),
            include_charges=True),
        StatsOptionItemDmg(
            time_options=StatTimeSim(time=405, rearm_minions=consts.ApiRearmMinion.on_first_empty),
            include_charges=True)])))
    api_fighter_dmg_disabled, api_fighter_dmg_rearm = api_fighter_stats.dmg
    assert api_fighter_dmg_disabled.dps == [0, 0, 0, approx(751.313657)]
    assert api_fighter_dmg_disabled.volley == [0, 0, 0, approx(9429.84375)]
    assert api_fighter_dmg_rearm.dps == [0, 0, 0, approx(595.170139)]
    assert api_fighter_dmg_rearm.volley == [0, 0, 0, approx(9429.84375)]


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


def test_autocharge_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = client.alloc_item_id()
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000,
        sq_size=6)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.atkm_abil_id: False, eve_basic_info.bomb_abil_id: True})
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=True)).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_fit_dmg_stats = api_fit.get_stats(options=FitStatsOptions(dmg=True)).dmg.one()
    assert api_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fit_dmg_stats.volley == [0, 0, 0, 0]
