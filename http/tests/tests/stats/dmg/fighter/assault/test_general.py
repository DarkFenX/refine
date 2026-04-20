from fw import approx
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionItemDmg
from tests.stats.dmg import make_eve_fighter_assault, setup_dmg_basics


def test_state(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.34375, prm_cycle_time=5000,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.34375, sec_cycle_time=14000,
        sec_charge_count=18, sec_charge_rearm_time=4, sq_size=6)
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
    api_fighter.change_fighter(state=consts.ApiMinionState.engaging)
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
