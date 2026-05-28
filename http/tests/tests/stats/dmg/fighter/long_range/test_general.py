from fw import approx
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionItemDmg
from tests.stats.dmg import make_eve_bomb, make_eve_fighter_lr, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000, sec_charge_count=5, sec_charge_rearm_time=20,
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
