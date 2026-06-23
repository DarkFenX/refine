from fw import approx
from fw.api import FitStatsOptions, FleetStatsOptions, StatsOptionFitDmg, StatTimeBurst, StatTimeSim
from tests.stats.dmg import make_eve_breacher, make_eve_launcher, make_eve_ship, setup_dmg_basics


def test_simple(client, consts):
    # Simple scenario is when best relative/absolute breachers are infinitely cycling; in this case,
    # breacher full "sim" is not used
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=25, cycle_time=10000, reload_time=30000)
    eve_charge1_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=750, dmg_rel=0.8, dmg_duration=60000, volume=0.5,
        speed=3000, flight_time=4000, mass=1000, agility=8)
    eve_charge2_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=600, dmg_rel=1, dmg_duration=60000, volume=0.5,
        speed=3000, flight_time=4000, mass=1000, agility=8)
    eve_tgt_ship_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, hps=(0, 0, 75000), radius=3000, speed=1000, sig_radius=40)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge1_id)
    api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge2_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    # Verification - in this case we also fetch applied stats, which show mismatch between
    # approximate (non-applied) stats and accurate (applied) stats. Non-applied stats just take max
    # of absolute and relative values (which, if applied to a 75k HP ship, would result in 750 dps),
    # while applied stats see that neither breacher applies more than 600, and expose that.
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=None)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(750), approx(0.01)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(750), approx(0.01)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(600)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(600)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=None)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None), projectee_item_id=api_tgt_ship.id)])))
    api_src_fit_dmg_stats_raw, api_src_fit_dmg_stats_applied = api_src_fit_stats.dmg
    assert api_src_fit_dmg_stats_raw.dps.breacher == [approx(750), approx(0.01)]
    assert api_src_fit_dmg_stats_raw.volley.breacher == [approx(750), approx(0.01)]
    assert api_src_fit_dmg_stats_applied.dps.breacher == approx(600)
    assert api_src_fit_dmg_stats_applied.volley.breacher == approx(600)


def test_reload_gap_realistic(client, consts):
    # Realistic case of 2 Tholoses - one with higher DPS and bad duration skill, and another with
    # permanently applied breacher with worse DPS
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module1_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=4, cycle_time=24000, reload_time=30000)
    eve_module2_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=4, cycle_time=12000, reload_time=30000)
    eve_charge1_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=250, dmg_rel=0.75, dmg_duration=40000, volume=0.1,
        speed=3000, flight_time=2000, mass=100, agility=8)
    eve_charge2_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=200, dmg_rel=0.69, dmg_duration=60000, volume=0.1,
        speed=3000, flight_time=2000, mass=100, agility=8)
    eve_tgt_ship_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, hps=(0, 0, 33334), radius=3000, speed=1000, sig_radius=40)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit1 = api_sol.create_fit()
    api_src_fit1.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge1_id)
    api_src_fit2 = api_sol.create_fit()
    api_src_fit2.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge2_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit1.id, api_src_fit2.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    # Verification - burst stats, no reload - no gaps
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeBurst(), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(250), approx(0.0075)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(250), approx(0.0075)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(250)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(250)
    # Verification - sim stats over infinite period with gaps during reload
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=None)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(249.292929), approx(0.007491515)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(250), approx(0.0075)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(249.292929)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(250)
    # Verification - sim stats just after first damage tick
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=0.1)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=0.1), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(2500), approx(0.075)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(250), approx(0.0075)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(2500)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(250)
    # Verification - sim stats just before second damage tick
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=0.9)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=0.9), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(277.777778), approx(0.008333333)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(250), approx(0.0075)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(277.777778)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(250)
    # Verification - sim stats just after second damage tick
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=1.1)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=1.1), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(454.545455), approx(0.01363636)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(250), approx(0.0075)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(454.545455)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(250)
    # Verification - sim stats with time just after the last pre-gap tick (39 * 24 + 40 for first
    # launcher), dps is slightly higher than 250 since it's 977 damage ticks but 976.1 seconds
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=976.1)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=976.1), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(250.179285), approx(0.007506301)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(250), approx(0.0075)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(250.179285)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(250)
    # Verification - sim stats with time just after the last gap tick (40 * 24 + 30), here dps falls
    # a bit, because during the first launcher gap dps is limited to 200 from second launcher
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=990.1)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=990.1), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(249.52025), approx(0.007498334)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(250), approx(0.0075)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(249.52025)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(250)
    # Verification - sim stats with time just after the first post-gap tick, dps increases a bit
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=991.1)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=991.1), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(249.520735), approx(0.007498335)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(250), approx(0.0075)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(249.520735)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(250)


def test_cycle_and_reload_gaps(client, consts):
    # Both breachers in this case have different downtime - shorter on every module cycle, longer
    # during reload. They partially cover each other's downtime, so when both are on, they deal more
    # damage. Also, this test checks that stacking rules are applied if there are multiple breachers
    # on one fit.
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module1_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=10, cycle_time=30000, reload_time=15000)
    eve_module2_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=10, cycle_time=20000, reload_time=10000)
    eve_charge1_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=1000, dmg_rel=0.5, dmg_duration=15000, volume=1,
        speed=3000, flight_time=4000, mass=1000, agility=8)
    eve_charge2_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=500, dmg_rel=1, dmg_duration=10000, volume=1,
        speed=3000, flight_time=4000, mass=1000, agility=8)
    eve_tgt_ship_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, hps=(0, 0, 100000), radius=3000, speed=1000, sig_radius=40)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.add_module(
        type_id=eve_module1_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id)
    api_src_fit.add_module(
        type_id=eve_module2_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge2_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    # Verification - burst stats, no reload - only cycle gaps. Each breacher individually covers
    # only half of time, but both are 3/4, due to how their cycles are laid out
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeBurst(), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(625), approx(0.00625)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(1000), approx(0.01)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(375)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(500)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeBurst()),
        StatsOptionFitDmg(time_options=StatTimeBurst(), projectee_item_id=api_tgt_ship.id)])))
    api_src_fit_dmg_stats_raw, api_src_fit_dmg_stats_applied = api_src_fit_stats.dmg
    assert api_src_fit_dmg_stats_raw.dps.breacher == [approx(625), approx(0.00625)]
    assert api_src_fit_dmg_stats_raw.volley.breacher == [approx(1000), approx(0.01)]
    assert api_src_fit_dmg_stats_applied.dps.breacher == approx(375)
    assert api_src_fit_dmg_stats_applied.volley.breacher == approx(500)
    # Verification - sim stats over infinite period with both gaps considered
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=None)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(599.206349), approx(0.005992063)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(1000), approx(0.01)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(361.111111)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(500)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=None)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=None), projectee_item_id=api_tgt_ship.id)])))
    api_src_fit_dmg_stats_raw, api_src_fit_dmg_stats_applied = api_src_fit_stats.dmg
    assert api_src_fit_dmg_stats_raw.dps.breacher == [approx(599.206349), approx(0.005992063)]
    assert api_src_fit_dmg_stats_raw.volley.breacher == [approx(1000), approx(0.01)]
    assert api_src_fit_dmg_stats_applied.dps.breacher == approx(361.111111)
    assert api_src_fit_dmg_stats_applied.volley.breacher == approx(500)
    # Verification - sim stats with time which is the least common multiple of both full cycle times
    # (but does not get next tick into scope) yields the same result as full loop
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=629.999999999)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=629.999999999), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(599.206349), approx(0.005992063)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(1000), approx(0.01)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(361.111111)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(500)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=629.999999999)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=629.999999999), projectee_item_id=api_tgt_ship.id)])))
    api_src_fit_dmg_stats_raw, api_src_fit_dmg_stats_applied = api_src_fit_stats.dmg
    assert api_src_fit_dmg_stats_raw.dps.breacher == [approx(599.206349), approx(0.005992063)]
    assert api_src_fit_dmg_stats_raw.volley.breacher == [approx(1000), approx(0.01)]
    assert api_src_fit_dmg_stats_applied.dps.breacher == approx(361.111111)
    assert api_src_fit_dmg_stats_applied.volley.breacher == approx(500)
    # Verification - multiple LCMs to see if various optimizations do not break expected result
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=1889.999999999)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=1889.999999999), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(599.206349), approx(0.005992063)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(1000), approx(0.01)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(361.111111)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(500)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=1889.999999999)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=1889.999999999), projectee_item_id=api_tgt_ship.id)])))
    api_src_fit_dmg_stats_raw, api_src_fit_dmg_stats_applied = api_src_fit_stats.dmg
    assert api_src_fit_dmg_stats_raw.dps.breacher == [approx(599.206349), approx(0.005992063)]
    assert api_src_fit_dmg_stats_raw.volley.breacher == [approx(1000), approx(0.01)]
    assert api_src_fit_dmg_stats_applied.dps.breacher == approx(361.111111)
    assert api_src_fit_dmg_stats_applied.volley.breacher == approx(500)
    # Verification - some random time into second "loop"
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=852.2)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=852.2), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(607.838536), approx(0.006031448)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(1000), approx(0.01)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(362.590941)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(500)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=852.2)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=852.2), projectee_item_id=api_tgt_ship.id)])))
    api_src_fit_dmg_stats_raw, api_src_fit_dmg_stats_applied = api_src_fit_stats.dmg
    assert api_src_fit_dmg_stats_raw.dps.breacher == [approx(607.838536), approx(0.006031448)]
    assert api_src_fit_dmg_stats_raw.volley.breacher == [approx(1000), approx(0.01)]
    assert api_src_fit_dmg_stats_applied.dps.breacher == approx(362.590941)
    assert api_src_fit_dmg_stats_applied.volley.breacher == approx(500)
    # Verification - some random time as in last step, but into 4th loop, result should be
    # different (closer to average) in this case
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=2112.2)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=2112.2), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_dmg_stats_raw, api_fleet_dmg_stats_applied = api_fleet_stats.dmg
    assert api_fleet_dmg_stats_raw.dps.breacher == [approx(602.689139), approx(0.006007954)]
    assert api_fleet_dmg_stats_raw.volley.breacher == [approx(1000), approx(0.01)]
    assert api_fleet_dmg_stats_applied.dps.breacher == approx(361.708172)
    assert api_fleet_dmg_stats_applied.volley.breacher == approx(500)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(dmg=(True, [
        StatsOptionFitDmg(time_options=StatTimeSim(time=2112.2)),
        StatsOptionFitDmg(time_options=StatTimeSim(time=2112.2), projectee_item_id=api_tgt_ship.id)])))
    api_src_fit_dmg_stats_raw, api_src_fit_dmg_stats_applied = api_src_fit_stats.dmg
    assert api_src_fit_dmg_stats_raw.dps.breacher == [approx(602.689139), approx(0.006007954)]
    assert api_src_fit_dmg_stats_raw.volley.breacher == [approx(1000), approx(0.01)]
    assert api_src_fit_dmg_stats_applied.dps.breacher == approx(361.708172)
    assert api_src_fit_dmg_stats_applied.volley.breacher == approx(500)
