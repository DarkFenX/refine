from tests import approx
from tests.fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatsOptionFitDps,
    StatsOptionFitVolley,
    StatsOptionItemDps,
    StatsOptionItemVolley,
)
from tests.tests.stats.dmg import make_eve_bomb, make_eve_drone, make_eve_launcher, make_eve_ship, setup_dmg_basics


def test_range_tick_aligned(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=20.5)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 11999, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, 0, 0]
    assert api_fleet_stats.volley.one() == [0, 0, 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_src_fit_stats.volley.one() == [0, 0, 0, 0]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 12001, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(187.096774), 0, 0, 0]
    assert api_fleet_stats.volley.one() == [approx(14500), 0, 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(187.096774), 0, 0, 0]
    assert api_src_fit_stats.volley.one() == [approx(14500), 0, 0, 0]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 47999, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(187.096774), 0, 0, 0]
    assert api_fleet_stats.volley.one() == [approx(14500), 0, 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(187.096774), 0, 0, 0]
    assert api_src_fit_stats.volley.one() == [approx(14500), 0, 0, 0]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 48001, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, 0, 0]
    assert api_fleet_stats.volley.one() == [0, 0, 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_src_fit_stats.volley.one() == [0, 0, 0, 0]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_range_tick_misaligned(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7, 7, 7, 7), volume=75,
        speed=4000, flight_time=7500, mass=1000, agility=0.0000251, exp_range=15000, exp_radius=400)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=20.5)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 9999, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, 0, 0]
    assert api_fleet_stats.volley.one() == [0, 0, 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_src_fit_stats.volley.one() == [0, 0, 0, 0]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 10001, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [
        approx(0.09032258), approx(0.09032258), approx(0.09032258), approx(0.09032258)]
    assert api_fleet_stats.volley.one() == [approx(7), approx(7), approx(7), approx(7)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [
        approx(0.09032258), approx(0.09032258), approx(0.09032258), approx(0.09032258)]
    assert api_src_fit_stats.volley.one() == [approx(7), approx(7), approx(7), approx(7)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [
        approx(0.0451613), approx(0.0451613), approx(0.0451613), approx(0.0451613)]
    assert api_charge_proj_stats.volley.one() == [approx(3.5), approx(3.5), approx(3.5), approx(3.5)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [
        approx(0.0451613), approx(0.0451613), approx(0.0451613), approx(0.0451613)]
    assert api_charge_nonproj_stats.volley.one() == [approx(3.5), approx(3.5), approx(3.5), approx(3.5)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 13999, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [
        approx(0.09032258), approx(0.09032258), approx(0.09032258), approx(0.09032258)]
    assert api_fleet_stats.volley.one() == [approx(7), approx(7), approx(7), approx(7)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [
        approx(0.09032258), approx(0.09032258), approx(0.09032258), approx(0.09032258)]
    assert api_src_fit_stats.volley.one() == [approx(7), approx(7), approx(7), approx(7)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [
        approx(0.0451613), approx(0.0451613), approx(0.0451613), approx(0.0451613)]
    assert api_charge_proj_stats.volley.one() == [approx(3.5), approx(3.5), approx(3.5), approx(3.5)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [
        approx(0.0451613), approx(0.0451613), approx(0.0451613), approx(0.0451613)]
    assert api_charge_nonproj_stats.volley.one() == [approx(3.5), approx(3.5), approx(3.5), approx(3.5)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 14001, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(0.1806452), approx(0.1806452), approx(0.1806452), approx(0.1806452)]
    assert api_fleet_stats.volley.one() == [approx(14), approx(14), approx(14), approx(14)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(0.1806452), approx(0.1806452), approx(0.1806452), approx(0.1806452)]
    assert api_src_fit_stats.volley.one() == [approx(14), approx(14), approx(14), approx(14)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [
        approx(0.09032258), approx(0.09032258), approx(0.09032258), approx(0.09032258)]
    assert api_charge_proj_stats.volley.one() == [approx(7), approx(7), approx(7), approx(7)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [
        approx(0.09032258), approx(0.09032258), approx(0.09032258), approx(0.09032258)]
    assert api_charge_nonproj_stats.volley.one() == [approx(7), approx(7), approx(7), approx(7)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 45999, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(0.1806452), approx(0.1806452), approx(0.1806452), approx(0.1806452)]
    assert api_fleet_stats.volley.one() == [approx(14), approx(14), approx(14), approx(14)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(0.1806452), approx(0.1806452), approx(0.1806452), approx(0.1806452)]
    assert api_src_fit_stats.volley.one() == [approx(14), approx(14), approx(14), approx(14)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [
        approx(0.09032258), approx(0.09032258), approx(0.09032258), approx(0.09032258)]
    assert api_charge_proj_stats.volley.one() == [approx(7), approx(7), approx(7), approx(7)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [
        approx(0.09032258), approx(0.09032258), approx(0.09032258), approx(0.09032258)]
    assert api_charge_nonproj_stats.volley.one() == [approx(7), approx(7), approx(7), approx(7)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 46001, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [
        approx(0.09032258), approx(0.09032258), approx(0.09032258), approx(0.09032258)]
    assert api_fleet_stats.volley.one() == [approx(7), approx(7), approx(7), approx(7)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [
        approx(0.09032258), approx(0.09032258), approx(0.09032258), approx(0.09032258)]
    assert api_src_fit_stats.volley.one() == [approx(7), approx(7), approx(7), approx(7)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [
        approx(0.0451613), approx(0.0451613), approx(0.0451613), approx(0.0451613)]
    assert api_charge_proj_stats.volley.one() == [approx(3.5), approx(3.5), approx(3.5), approx(3.5)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [
        approx(0.0451613), approx(0.0451613), approx(0.0451613), approx(0.0451613)]
    assert api_charge_nonproj_stats.volley.one() == [approx(3.5), approx(3.5), approx(3.5), approx(3.5)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 49999, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [
        approx(0.09032258), approx(0.09032258), approx(0.09032258), approx(0.09032258)]
    assert api_fleet_stats.volley.one() == [approx(7), approx(7), approx(7), approx(7)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [
        approx(0.09032258), approx(0.09032258), approx(0.09032258), approx(0.09032258)]
    assert api_src_fit_stats.volley.one() == [approx(7), approx(7), approx(7), approx(7)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [
        approx(0.0451613), approx(0.0451613), approx(0.0451613), approx(0.0451613)]
    assert api_charge_proj_stats.volley.one() == [approx(3.5), approx(3.5), approx(3.5), approx(3.5)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [
        approx(0.0451613), approx(0.0451613), approx(0.0451613), approx(0.0451613)]
    assert api_charge_nonproj_stats.volley.one() == [approx(3.5), approx(3.5), approx(3.5), approx(3.5)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 50001, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, 0, 0]
    assert api_fleet_stats.volley.one() == [0, 0, 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, 0, 0]
    assert api_src_fit_stats.volley.one() == [0, 0, 0, 0]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_application(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=20.5)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=100)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 30000, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [approx(46.774194), 0, 0, 0]
    assert api_fleet_stats.volley.one() == [approx(3625), 0, 0, 0]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [approx(46.774194), 0, 0, 0]
    assert api_src_fit_stats.volley.one() == [approx(3625), 0, 0, 0]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(23.387097), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(1812.5), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(23.387097), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(1812.5), 0, 0, 0]


def test_npc_prop_mode(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=20.5)
    eve_tgt_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, radius=35,
        speed_cruise=500, speed_chase=1000, sig_radius=100, prop_sig_radius_mult=5)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_drone = api_tgt_fit.add_drone(
        type_id=eve_tgt_drone_id,
        coordinates=(0, 30000, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_module_proj.change_module(add_projs=[api_tgt_drone.id])
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(23.387097), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(1812.5), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(23.387097), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(1812.5), 0, 0, 0]
    # Action
    api_tgt_drone.change_drone(prop_mode=consts.ApiNpcPropMode.chase)
    # Verification - drone is in chase mode and has its sig blown, so bomb applies fully
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Action
    api_tgt_drone.change_drone(prop_mode=consts.ApiNpcPropMode.cruise)
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(23.387097), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(1812.5), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(23.387097), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(1812.5), 0, 0, 0]


def test_bomb_attr_speed_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75,
        flight_time=12000, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=20.5)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, -18001, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, -17999, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 17999, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 18001, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_bomb_attr_flight_time_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75,
        speed=2500, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=20.5)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, -18001, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, -17999, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 17999, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 18001, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_bomb_attr_mass_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75,
        speed=2500, flight_time=12000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=20.5)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 11999, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 12001, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 47999, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 48001, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_bomb_attr_agility_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75,
        speed=2500, flight_time=12000, mass=1000, exp_range=15000, exp_radius=400)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=20.5)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 11999, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 12001, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 47999, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 48001, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_bomb_attr_exp_range_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_radius=400)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=20.5)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 26999, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 27001, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 32999, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 33001, 0))
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_bomb_attr_exp_radius_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_range=15000)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=20.5)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=100)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 30000, 0), movement=(0, 0, 1))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]


def test_bomb_ship_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_src_ship_id = client.alloc_item_id()
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 30000, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [approx(7250), 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [approx(93.548387), 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [approx(7250), 0, 0, 0]


def test_tgt_attr_sig_radius_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=20.5)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=3000, speed=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 30000, 0), movement=(0, 0, 1))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_tgt_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=300,
        cycle_time=10000, reload_time=10000, reactivation_delay=67500)
    eve_charge_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(7250, 0, 0, 0), volume=75,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=20.5)
    eve_tgt_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 30000, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_proj_stats.volley.one() == [0, 0, 0, 0]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_charge_nonproj_stats.volley.one() == [0, 0, 0, 0]
