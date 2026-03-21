from fw import approx
from fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    StatsOptionFitDps,
    StatsOptionFitVolley,
    StatTimeBurst,
    StatTimeSim,
)
from tests.stats.dmg import make_eve_breacher, make_eve_launcher, make_eve_ship, setup_dmg_basics


def test_simple(client, consts):
    # Simple scenario is when best relative/absolute breachers are infinitely cycling; in this case,
    # breacher full "sim" is not used
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=25, cycle_time=10000, reload_time=30000)
    eve_charge1_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=1000, dmg_rel=0.8, dmg_duration=75000, volume=0.5,
        speed=3000, flight_time=4000, mass=1000, agility=8)
    eve_charge2_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=800, dmg_rel=1, dmg_duration=75000, volume=0.5,
        speed=3000, flight_time=4000, mass=1000, agility=8)
    eve_tgt_ship_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, hps=(0, 0, 100000), radius=3000, speed=1000, sig_radius=40)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge1_id)
    api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge2_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    # Verification - in this case we also fetch applied stats, which show mismatch between
    # approximate (non-applied) stats and accurate (applied) stats. Non-applied stats just take max
    # of absolute and relative values (which, if applied to a 100k HP ship, would apply 1000 dps),
    # while applied stats see that neither breacher applies more than 800, and expose that.
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [
            StatsOptionFitDps(time_options=StatTimeSim(time=None)),
            StatsOptionFitDps(time_options=StatTimeSim(time=None), projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [
            StatsOptionFitVolley(time_options=StatTimeSim(time=None)),
            StatsOptionFitVolley(time_options=StatTimeSim(time=None), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_stats_dps_raw, api_fleet_stats_dps_applied = api_fleet_stats.dps.map(lambda i: i.breacher)
    api_fleet_stats_volley_raw, api_fleet_stats_volley_applied = api_fleet_stats.volley.map(lambda i: i.breacher)
    assert api_fleet_stats_dps_raw == [approx(1000), approx(0.01)]
    assert api_fleet_stats_volley_raw == [approx(1000), approx(0.01)]
    assert api_fleet_stats_dps_applied == 800
    assert api_fleet_stats_volley_applied == 800
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [
            StatsOptionFitDps(time_options=StatTimeSim(time=None)),
            StatsOptionFitDps(time_options=StatTimeSim(time=None), projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [
            StatsOptionFitVolley(time_options=StatTimeSim(time=None)),
            StatsOptionFitVolley(time_options=StatTimeSim(time=None), projectee_item_id=api_tgt_ship.id)])))
    api_src_fit_stats_dps_raw, api_src_fit_stats_dps_applied = api_src_fit_stats.dps.map(lambda i: i.breacher)
    api_src_fit_stats_volley_raw, api_src_fit_stats_volley_applied = api_src_fit_stats.volley.map(lambda i: i.breacher)
    assert api_src_fit_stats_dps_raw == [approx(1000), approx(0.01)]
    assert api_src_fit_stats_volley_raw == [approx(1000), approx(0.01)]
    assert api_src_fit_stats_dps_applied == 800
    assert api_src_fit_stats_volley_applied == 800


def test_reload_gap_realistic(client, consts):
    # Realistic case of 2 Tholoses - one with higher DPS and bad reload/duration skills, and another
    # with permanently applied breacher with worse DPS
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module1_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=4, cycle_time=24000, reload_time=30000)
    eve_module2_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=4, cycle_time=12000, reload_time=30000)
    eve_charge1_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=200, dmg_rel=0.75, dmg_duration=50000, volume=0.1)
    eve_charge2_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=160, dmg_rel=0.6, dmg_duration=75000, volume=0.1)
    eve_tgt_ship_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, hps=(0, 0, 26667), radius=3000, speed=1000, sig_radius=40)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit1 = api_sol.create_fit()
    api_src_fit1.add_module(type_id=eve_module1_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge1_id)
    api_src_fit2 = api_sol.create_fit()
    api_src_fit2.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge2_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit1.id, api_src_fit2.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    # Verification - burst stats, no reload - no gaps
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [
            StatsOptionFitDps(time_options=StatTimeBurst()),
            StatsOptionFitDps(time_options=StatTimeBurst(), projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [
            StatsOptionFitVolley(time_options=StatTimeBurst()),
            StatsOptionFitVolley(time_options=StatTimeBurst(), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_stats_dps_raw, api_fleet_stats_dps_applied = api_fleet_stats.dps.map(lambda i: i.breacher)
    api_fleet_stats_volley_raw, api_fleet_stats_volley_applied = api_fleet_stats.volley.map(lambda i: i.breacher)
    assert api_fleet_stats_dps_raw == [approx(200), approx(0.0075)]
    assert api_fleet_stats_volley_raw == [approx(200), approx(0.0075)]
    assert api_fleet_stats_dps_applied == 200
    assert api_fleet_stats_volley_applied == 200
    # Verification - sim stats over infinite period with gaps during reload
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [
            StatsOptionFitDps(time_options=StatTimeSim(time=None)),
            StatsOptionFitDps(time_options=StatTimeSim(time=None), projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [
            StatsOptionFitVolley(time_options=StatTimeSim(time=None)),
            StatsOptionFitVolley(time_options=StatTimeSim(time=None), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_stats_dps_raw, api_fleet_stats_dps_applied = api_fleet_stats.dps.map(lambda i: i.breacher)
    api_fleet_stats_volley_raw, api_fleet_stats_volley_applied = api_fleet_stats.volley.map(lambda i: i.breacher)
    assert api_fleet_stats_dps_raw == [approx(199.838384), approx(0.007493939)]
    assert api_fleet_stats_volley_raw == [approx(200), approx(0.0075)]
    assert api_fleet_stats_dps_applied == approx(199.838384)
    assert api_fleet_stats_volley_applied == 200
    # Verification - sim stats just before first damage tick
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [
            StatsOptionFitDps(time_options=StatTimeSim(time=0.9)),
            StatsOptionFitDps(time_options=StatTimeSim(time=0.9), projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [
            StatsOptionFitVolley(time_options=StatTimeSim(time=0.9)),
            StatsOptionFitVolley(time_options=StatTimeSim(time=0.9), projectee_item_id=api_tgt_ship.id)])))
    api_fleet_stats_dps_raw, api_fleet_stats_dps_applied = api_fleet_stats.dps.map(lambda i: i.breacher)
    api_fleet_stats_volley_raw, api_fleet_stats_volley_applied = api_fleet_stats.volley.map(lambda i: i.breacher)
    assert api_fleet_stats_dps_raw == [approx(222.222222), approx(0.008333333)]
    assert api_fleet_stats_volley_raw == [approx(200), approx(0.0075)]
    assert api_fleet_stats_dps_applied == approx(222.222222)
    assert api_fleet_stats_volley_applied == approx(200)


def test_complex_different_multiple_downtimes(client, consts):
    # Both breachers in this case have different downtime - shorter on every module cycle, longer
    # during reload. They partially cover each other's downtime, so when both are on, they deal more
    # damage. Also, this test checks that stacking rules are applied if there are multiple breachers
    # on one fit.
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module1_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=10, cycle_time=30000, reload_time=15000)
    eve_charge1_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=1000, dmg_rel=0.5, dmg_duration=15000, volume=1)
    eve_module2_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=10, cycle_time=20000, reload_time=10000)
    eve_charge2_id = make_eve_breacher(
        client=client, basic_info=eve_basic_info, dmg_abs=500, dmg_rel=1, dmg_duration=10000, volume=1)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module1 = api_fit.add_module(
        type_id=eve_module1_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge1_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(), StatsOptionFitDps(time_options=StatTimeSim(time=None))])))
    api_fleet_dps_burst, api_fleet_dps_reload = api_fleet_stats.dps
    assert api_fleet_dps_burst.breacher == [approx(500), approx(0.0025)]
    assert api_fleet_dps_reload.breacher == [approx(476.190476), approx(0.002380952)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(), StatsOptionFitDps(time_options=StatTimeSim(time=None))])))
    api_fit_dps_burst, api_fit_dps_reload = api_fit_stats.dps
    assert api_fit_dps_burst.breacher == [approx(500), approx(0.0025)]
    assert api_fit_dps_reload.breacher == [approx(476.190476), approx(0.002380952)]
    # Action
    api_fit.add_module(type_id=eve_module2_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge2_id)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(), StatsOptionFitDps(time_options=StatTimeSim(time=None))])))
    api_fleet_dps_burst, api_fleet_dps_reload = api_fleet_stats.dps
    assert api_fleet_dps_burst.breacher == [approx(625), approx(0.00625)]
    assert api_fleet_dps_reload.breacher == [approx(599.206349), approx(0.005992063)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(), StatsOptionFitDps(time_options=StatTimeSim(time=None))])))
    api_fit_dps_burst, api_fit_dps_reload = api_fit_stats.dps
    assert api_fit_dps_burst.breacher == [approx(625), approx(0.00625)]
    assert api_fit_dps_reload.breacher == [approx(599.206349), approx(0.005992063)]
    # Action
    api_module1.remove()
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(), StatsOptionFitDps(time_options=StatTimeSim(time=None))])))
    api_fleet_dps_burst, api_fleet_dps_reload = api_fleet_stats.dps
    assert api_fleet_dps_burst.breacher == [approx(250), approx(0.005)]
    assert api_fleet_dps_reload.breacher == [approx(238.095238), approx(0.004761905)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(), StatsOptionFitDps(time_options=StatTimeSim(time=None))])))
    api_fit_dps_burst, api_fit_dps_reload = api_fit_stats.dps
    assert api_fit_dps_burst.breacher == [approx(250), approx(0.005)]
    assert api_fit_dps_reload.breacher == [approx(238.095238), approx(0.004761905)]
