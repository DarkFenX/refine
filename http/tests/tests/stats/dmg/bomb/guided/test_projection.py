from fw import approx
from fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatsOptionFitDps,
    StatsOptionFitVolley,
    StatsOptionItemDps,
    StatsOptionItemVolley,
)
from tests.stats.dmg import (
    make_eve_bomb_guided,
    make_eve_drone,
    make_eve_launcher,
    make_eve_ship,
    setup_dmg_basics,
)


def test_range(client, consts):
    # Guided bomb range follows regular missile mechanics. In this case, stats are taken off heavy
    # guided bomb launched from a keepstar. It has terrible agility for a charge, so takes a while
    # to accelerate, but very high flight time. On top of that, it gets a sizeable flight time bonus
    # from the keepstar due to it having huge radius
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=400, cycle_time=33600, reload_time=60000)
    eve_charge_id = make_eve_bomb_guided(
        client=client, basic_info=eve_basic_info, dmgs=(2640, 2640, 2640, 2640), volume=100,
        speed=3625, flight_time=400000, mass=1500, agility=1000, exp_range=20000, exp_radius=350,
        exp_speed=100, drf=0.944)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=150000)
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
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification - unlike regular bombs, guided bombs reach everywhere within their range
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [
        approx(157.142857), approx(157.142857), approx(157.142857), approx(157.142857)]
    assert api_fleet_stats.volley.one() == [
        approx(5280), approx(5280), approx(5280), approx(5280)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [
        approx(157.142857), approx(157.142857), approx(157.142857), approx(157.142857)]
    assert api_src_fit_stats.volley.one() == [
        approx(5280), approx(5280), approx(5280), approx(5280)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [
        approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_charge_proj_stats.volley.one() == [
        approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [
        approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_charge_nonproj_stats.volley.one() == [
        approx(2640), approx(2640), approx(2640), approx(2640)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 1598900, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [
        approx(157.142857), approx(157.142857), approx(157.142857), approx(157.142857)]
    assert api_fleet_stats.volley.one() == [
        approx(5280), approx(5280), approx(5280), approx(5280)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [
        approx(157.142857), approx(157.142857), approx(157.142857), approx(157.142857)]
    assert api_src_fit_stats.volley.one() == [
        approx(5280), approx(5280), approx(5280), approx(5280)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [
        approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_charge_proj_stats.volley.one() == [
        approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [
        approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_charge_nonproj_stats.volley.one() == [
        approx(2640), approx(2640), approx(2640), approx(2640)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 1599000, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [
        approx(59.605911), approx(59.605911), approx(59.605911), approx(59.605911)]
    assert api_fleet_stats.volley.one() == [
        approx(2002.758621), approx(2002.758621), approx(2002.758621), approx(2002.758621)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [
        approx(59.605911), approx(59.605911), approx(59.605911), approx(59.605911)]
    assert api_src_fit_stats.volley.one() == [
        approx(2002.758621), approx(2002.758621), approx(2002.758621), approx(2002.758621)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [
        approx(29.802956), approx(29.802956), approx(29.802956), approx(29.802956)]
    assert api_charge_proj_stats.volley.one() == [
        approx(1001.37931), approx(1001.37931), approx(1001.37931), approx(1001.37931)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [
        approx(29.802956), approx(29.802956), approx(29.802956), approx(29.802956)]
    assert api_charge_nonproj_stats.volley.one() == [
        approx(1001.37931), approx(1001.37931), approx(1001.37931), approx(1001.37931)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 1602625, 0))
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
        client=client, basic_info=eve_basic_info, capacity=400, cycle_time=33600, reload_time=60000)
    eve_charge_id = make_eve_bomb_guided(
        client=client, basic_info=eve_basic_info, dmgs=(2640, 2640, 2640, 2640), volume=100,
        speed=3625, flight_time=400000, mass=1500, agility=1000, exp_range=20000, exp_radius=350,
        exp_speed=100, drf=0.944)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=150000)
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
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [
        approx(44.897959), approx(44.897959), approx(44.897959), approx(44.897959)]
    assert api_fleet_stats.volley.one() == [
        approx(1508.571429), approx(1508.571429), approx(1508.571429), approx(1508.571429)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [
        approx(44.897959), approx(44.897959), approx(44.897959), approx(44.897959)]
    assert api_src_fit_stats.volley.one() == [
        approx(1508.571429), approx(1508.571429), approx(1508.571429), approx(1508.571429)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [
        approx(22.44898), approx(22.44898), approx(22.44898), approx(22.44898)]
    assert api_charge_proj_stats.volley.one() == [
        approx(754.285714), approx(754.285714), approx(754.285714), approx(754.285714)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [
        approx(22.44898), approx(22.44898), approx(22.44898), approx(22.44898)]
    assert api_charge_nonproj_stats.volley.one() == [
        approx(754.285714), approx(754.285714), approx(754.285714), approx(754.285714)]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification - the bomb uses attributes for missile application and has missile effect, but
    # uses bomb application nevertheless
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [
        approx(44.897959), approx(44.897959), approx(44.897959), approx(44.897959)]
    assert api_fleet_stats.volley.one() == [
        approx(1508.571429), approx(1508.571429), approx(1508.571429), approx(1508.571429)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [
        approx(44.897959), approx(44.897959), approx(44.897959), approx(44.897959)]
    assert api_src_fit_stats.volley.one() == [
        approx(1508.571429), approx(1508.571429), approx(1508.571429), approx(1508.571429)]
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_proj_stats.dps.one() == [
        approx(22.44898), approx(22.44898), approx(22.44898), approx(22.44898)]
    assert api_charge_proj_stats.volley.one() == [
        approx(754.285714), approx(754.285714), approx(754.285714), approx(754.285714)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_charge_nonproj_stats.dps.one() == [
        approx(22.44898), approx(22.44898), approx(22.44898), approx(22.44898)]
    assert api_charge_nonproj_stats.volley.one() == [
        approx(754.285714), approx(754.285714), approx(754.285714), approx(754.285714)]


def test_npc_prop_mode(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_launcher(
        client=client, basic_info=eve_basic_info, capacity=400, cycle_time=33600, reload_time=60000)
    eve_charge_id = make_eve_bomb_guided(
        client=client, basic_info=eve_basic_info, dmgs=(2640, 2640, 2640, 2640), volume=100,
        speed=3625, flight_time=400000, mass=1500, agility=1000, exp_range=20000, exp_radius=350,
        exp_speed=100, drf=0.944)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=150000)
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
        coordinates=(0, 0, 0),
        movement=(0, 0, 0),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_module_proj.change_module(add_projs=[api_tgt_drone.id])
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_charge_proj_stats.dps.one() == [
        approx(22.44898), approx(22.44898), approx(22.44898), approx(22.44898)]
    assert api_charge_proj_stats.volley.one() == [
        approx(754.285714), approx(754.285714), approx(754.285714), approx(754.285714)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_charge_nonproj_stats.dps.one() == [
        approx(22.44898), approx(22.44898), approx(22.44898), approx(22.44898)]
    assert api_charge_nonproj_stats.volley.one() == [
        approx(754.285714), approx(754.285714), approx(754.285714), approx(754.285714)]
    # Action
    api_tgt_drone.change_drone(prop_mode=consts.ApiNpcPropMode.chase)
    # Verification - drone is in chase mode and has its sig blown, so bomb applies fully
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_charge_proj_stats.dps.one() == [
        approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_charge_proj_stats.volley.one() == [
        approx(2640), approx(2640), approx(2640), approx(2640)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_charge_nonproj_stats.dps.one() == [
        approx(78.571429), approx(78.571429), approx(78.571429), approx(78.571429)]
    assert api_charge_nonproj_stats.volley.one() == [
        approx(2640), approx(2640), approx(2640), approx(2640)]
    # Action
    api_tgt_drone.change_drone(prop_mode=consts.ApiNpcPropMode.cruise)
    # Verification
    api_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_charge_proj_stats.dps.one() == [
        approx(22.44898), approx(22.44898), approx(22.44898), approx(22.44898)]
    assert api_charge_proj_stats.volley.one() == [
        approx(754.285714), approx(754.285714), approx(754.285714), approx(754.285714)]
    api_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_charge_nonproj_stats.dps.one() == [
        approx(22.44898), approx(22.44898), approx(22.44898), approx(22.44898)]
    assert api_charge_nonproj_stats.volley.one() == [
        approx(754.285714), approx(754.285714), approx(754.285714), approx(754.285714)]
