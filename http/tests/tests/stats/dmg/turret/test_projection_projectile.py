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
from tests.tests.stats.dmg import (
    make_eve_drone,
    make_eve_ship,
    make_eve_turret_charge_normal,
    make_eve_turret_proj,
    setup_dmg_basics,
)


def test_range(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_optimal=3000, range_falloff=43000, tracking=4.05, sig_resolution=40000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=2550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=123, speed=6300, sig_radius=316)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 3623, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(94.1755481), approx(345.103817)]
    assert api_fleet_stats.volley.one() == [0, 0, approx(290.060688), approx(1062.919758)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(94.1755481), approx(345.103817)]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(290.060688), approx(1062.919758)]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 46623, 0))
    # Verification - dps is more than halved at 1 falloff
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(36.652431), approx(134.311869)]
    assert api_fleet_stats.volley.one() == [0, 0, approx(112.889488), approx(413.680558)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(36.652431), approx(134.311869)]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(112.889488), approx(413.680558)]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(18.326216), approx(67.155935)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(56.444744), approx(206.840279)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(18.326216), approx(67.155935)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(56.444744), approx(206.840279)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 89623, 0))
    # Verification - 2 falloffs
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(5.346693), approx(19.592814)]
    assert api_fleet_stats.volley.one() == [0, 0, approx(16.467813), approx(60.345867)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(5.346693), approx(19.592814)]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(16.467813), approx(60.345867)]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(2.673346), approx(9.796407)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(8.233907), approx(30.172934)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(2.673346), approx(9.796407)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(8.233907), approx(30.172934)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 133000, 0))
    # Verification - more than 3 falloffs, still some damage applied
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(0.5241352), approx(1.92068)]
    assert api_fleet_stats.volley.one() == [0, 0, approx(1.614336), approx(5.915693)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(0.5241352), approx(1.92068)]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(1.614336), approx(5.915693)]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(0.2620676), approx(0.9603398)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(0.80716822), approx(2.957846)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(0.2620676), approx(0.9603398)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(0.80716822), approx(2.957846)]


def test_application(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_optimal=3000, range_falloff=43000, tracking=4.05, sig_resolution=40000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=2550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=123, speed=6300, sig_radius=316)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_ship = api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 3623, 0), movement=(0, 0, 0.05))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification - hard to hit at close range despite low speed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(1.665791), approx(6.104247)]
    assert api_fleet_stats.volley.one() == [0, 0, approx(5.130636), approx(18.801081)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(1.665791), approx(6.104247)]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(5.130636), approx(18.801081)]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(0.8328955), approx(3.052124)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(2.565318), approx(9.400541)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(0.8328955), approx(3.052124)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(2.565318), approx(9.400541)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 13623, 0))
    # Verification - easier to hit at a bit higher range
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(53.677567), approx(196.700033)]
    assert api_fleet_stats.volley.one() == [0, 0, approx(165.326908), approx(605.836102)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(53.677567), approx(196.700033)]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(165.326908), approx(605.836102)]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(26.838784), approx(98.350017)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(82.663454), approx(302.918051)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(26.838784), approx(98.350017)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(82.663454), approx(302.918051)]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification - almost impossible to hit at high speed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(0), approx(0)]
    assert api_fleet_stats.volley.one() == [0, 0, approx(0), approx(0)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(0), approx(0)]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(0), approx(0)]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(0), approx(0)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(0), approx(0)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(0), approx(0)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(0), approx(0)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 13623, 0), movement=(90, 0, 1))
    # Verification - no misses due to tracking, since target is moving directly away
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(89.151948), approx(326.694967)]
    assert api_fleet_stats.volley.one() == [0, 0, approx(274.588), approx(1006.220499)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(89.151948), approx(326.694967)]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(274.588), approx(1006.220499)]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(44.575974), approx(163.347484)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(137.294), approx(503.110249)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(44.575974), approx(163.347484)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(137.294), approx(503.110249)]
    # Action
    api_tgt_ship.change_ship(movement=(85, 0, 1))
    # Verification - movement is at angle, so harder to hit again
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(21.669309), approx(79.406612)]
    assert api_fleet_stats.volley.one() == [0, 0, approx(66.741471), approx(244.572366)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(21.669309), approx(79.406612)]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(66.741471), approx(244.572366)]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(10.834654), approx(39.703306)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(33.370736), approx(122.286183)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(10.834654), approx(39.703306)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(33.370736), approx(122.286183)]
    # Action
    api_src_ship.change_ship(movement=(85, 0, 1))
    # Verification - attacker attempts to transmatch, hits become a bit better
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(51.719308), approx(189.524043)]
    assert api_fleet_stats.volley.one() == [0, 0, approx(159.295469), approx(583.734052)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(51.719308), approx(189.524043)]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(159.295469), approx(583.734052)]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(25.859654), approx(94.762021)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(79.647734), approx(291.867026)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(25.859654), approx(94.762021)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(79.647734), approx(291.867026)]
    # Action
    api_src_ship.change_ship(coordinates=(0, 5000, 0))
    # Verification - attacker is moved closer to target, and starts struggling with tracking again
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.dps.one() == [0, 0, approx(25.857123), approx(94.752747)]
    assert api_fleet_stats.volley.one() == [0, 0, approx(79.639939), approx(291.83846)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dps=(True, [StatsOptionFitDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionFitVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.dps.one() == [0, 0, approx(25.857123), approx(94.752747)]
    assert api_src_fit_stats.volley.one() == [0, 0, approx(79.639939), approx(291.83846)]
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(12.928562), approx(47.376373)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(39.819969), approx(145.91923)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(12.928562), approx(47.376373)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(39.819969), approx(145.91923)]


def test_npc_prop_mode(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_optimal=3000, range_falloff=43000, tracking=4.05, sig_resolution=40000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=2550)
    eve_tgt_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, radius=35,
        speed_cruise=500, speed_chase=1000, sig_radius=100, prop_sig_radius_mult=5)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_drone = api_tgt_fit.add_drone(
        type_id=eve_tgt_drone_id,
        coordinates=(0, 25035, 0),
        movement=(0, 0, 0.5),
        prop_mode=consts.ApiNpcPropMode.cruise)
    api_src_module_proj.change_module(add_projs=[api_tgt_drone.id])
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(15.159806), approx(55.552711)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(46.692203), approx(171.10235)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(15.159806), approx(55.552711)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(46.692203), approx(171.10235)]
    # Action
    api_tgt_drone.change_drone(prop_mode=consts.ApiNpcPropMode.chase)
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(31.539887), approx(115.577085)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(97.142851), approx(355.977423)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(31.539887), approx(115.577085)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(97.142851), approx(355.977423)]
    # Action
    api_tgt_drone.change_drone(prop_mode=consts.ApiNpcPropMode.cruise)
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(15.159806), approx(55.552711)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(46.692203), approx(171.10235)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_drone.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_drone.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(15.159806), approx(55.552711)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(46.692203), approx(171.10235)]


def test_turret_effect_range_optimal_absent(client, consts):
    # No falloff range defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts, effect_range=False)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_optimal=3000, range_falloff=43000, tracking=4.05, sig_resolution=40000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=2550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=123, speed=6300, sig_radius=316)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 623, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 43623, 0))
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(18.326216), approx(67.155935)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(56.444744), approx(206.840279)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(18.326216), approx(67.155935)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(56.444744), approx(206.840279)]


def test_turret_effect_range_falloff_absent(client, consts):
    # No optimal range defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts, effect_falloff=False)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_optimal=3000, range_falloff=43000, tracking=4.05, sig_resolution=40000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=2550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=123, speed=6300, sig_radius=316)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 3622, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 3624, 0))
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_proj_stats.volley.one() == [0, 0, 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_turret_effect_tracking_absent(client, consts):
    # No tracking defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts, effect_tracking=False)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_optimal=3000, range_falloff=43000, tracking=4.05, sig_resolution=40000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=2550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=123, speed=6300, sig_radius=316)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 623, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_proj_stats.volley.one() == [0, 0, 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_turret_attr_range_optimal_absent(client, consts):
    # No optimal range defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_falloff=43000, tracking=4.05, sig_resolution=40000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=2550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=123, speed=6300, sig_radius=316)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 623, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 43623, 0))
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(18.326216), approx(67.155935)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(56.444744), approx(206.840279)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(18.326216), approx(67.155935)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(56.444744), approx(206.840279)]


def test_turret_attr_range_falloff_absent(client, consts):
    # No optimal range defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_optimal=3000, tracking=4.05, sig_resolution=40000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=2550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=123, speed=6300, sig_radius=316)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 3622, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 3624, 0))
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_proj_stats.volley.one() == [0, 0, 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_turret_attr_tracking_absent(client, consts):
    # No tracking defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_optimal=3000, range_falloff=43000, sig_resolution=40000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=2550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=123, speed=6300, sig_radius=316)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 623, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_proj_stats.volley.one() == [0, 0, 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_turret_attr_sig_res_absent(client, consts):
    # No signature resolution defined - it is considered equal to 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_optimal=3000, range_falloff=43000, tracking=4.05)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=2550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=123, speed=6300, sig_radius=316)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 623, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification - perfect application even if target is moving due to 0 sig res
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]


def test_turret_ship_attr_speed_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_optimal=3000, range_falloff=43000, tracking=4.05, sig_resolution=40000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=123, speed=6300, sig_radius=316)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_ship = api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 623, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    # Action
    api_src_ship.change_ship(movement=(0, 0, 1))
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]


def test_turret_ship_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_optimal=3000, range_falloff=43000, tracking=4.05, sig_resolution=40000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = client.alloc_item_id()
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=123, speed=6300, sig_radius=316)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_ship = api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 623, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    # Action
    api_src_ship.change_ship(movement=(0, 0, 1))
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]


def test_tgt_attr_speed_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_optimal=3000, range_falloff=43000, tracking=4.05, sig_resolution=40000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=2550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=123, sig_radius=316)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 623, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_proj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, approx(47.087774), approx(172.551909)]
    assert api_module_nonproj_stats.volley.one() == [0, 0, approx(145.030344), approx(531.459879)]


def test_tgt_attr_sig_radius_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_optimal=3000, range_falloff=43000, tracking=4.05, sig_resolution=40000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=2550)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=123, speed=6300)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 623, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification - in case angular is 0 and sig radius is 0, the lib nullifies damage
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_proj_stats.volley.one() == [0, 0, 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, 0, 0, 0]


def test_tgt_attr_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_module_id = make_eve_turret_proj(
        client=client, basic_info=eve_basic_info, dmg_mult=9.4, capacity=3, cycle_time=3080, reload_time=10000,
        range_optimal=3000, range_falloff=43000, tracking=4.05, sig_resolution=40000)
    eve_charge_id = make_eve_turret_charge_normal(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 15.2, 55.7), volume=0.025)
    eve_src_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=2550)
    eve_tgt_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 623, 0), movement=(0, 0, 0))
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification - in case angular is 0 and sig radius is 0, the lib nullifies damage
    api_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_proj_stats.volley.one() == [0, 0, 0, 0]
    api_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        dps=(True, [StatsOptionItemDps(projectee_item_id=api_tgt_ship.id)]),
        volley=(True, [StatsOptionItemVolley(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.dps.one() == [0, 0, 0, 0]
    assert api_module_nonproj_stats.volley.one() == [0, 0, 0, 0]
