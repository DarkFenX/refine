from fw import approx
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionFitDmg, StatsOptionItemDmg
from tests.stats.dmg import make_eve_drone, make_eve_fighter_shadow, make_eve_ship, setup_dmg_basics


def test_range(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(200, 200, 0, 0), prm_dmg_mult=3.515625, prm_cycle_time=3500,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=2000, prm_exp_speed=60, prm_dr_factor=5, prm_dr_sens=5.5,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000,
        sec_range=500, sec_sig_radius=5000,
        sq_size=6, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=1700, speed=0, sig_radius=10000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 13735, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification - within optimal for primary ability
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(2410.714286), approx(2410.714286), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(8437.5), approx(8437.5), 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(2410.714286), approx(2410.714286), 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(8437.5), approx(8437.5), 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(4218.75), approx(4218.75), 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(4218.75), approx(4218.75), 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 18735, 0))
    # Verification - first falloff of the primary ability
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(4218.75), approx(4218.75), 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(4218.75), approx(4218.75), 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(602.678571), approx(602.678571), 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(2109.375), approx(2109.375), 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(602.678571), approx(602.678571), 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(2109.375), approx(2109.375), 0, 0]
    # Action
    api_src_fighter_proj.change_fighter(abilities={eve_basic_info.kamikaze_abil_id: True})
    api_src_fighter_nonproj.change_fighter(abilities={eve_basic_info.kamikaze_abil_id: True})
    api_tgt_ship.change_ship(coordinates=(0, 2234, 0))
    # Verification - within kamikaze range
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(60000), approx(60000), approx(60000), approx(60000)]
    assert api_fleet_dmg_stats.volley == [approx(600000), approx(600000), approx(600000), approx(600000)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(60000), approx(60000), approx(60000), approx(60000)]
    assert api_src_fit_dmg_stats.volley == [approx(600000), approx(600000), approx(600000), approx(600000)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_proj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 2236, 0))
    # Verification - just out of kamikaze range
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_application(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(200, 200, 0, 0), prm_dmg_mult=3.515625, prm_cycle_time=3500,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=2000, prm_exp_speed=60, prm_dr_factor=5, prm_dr_sens=5.5,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000,
        sec_range=500, sec_sig_radius=5000,
        sq_size=6)
    eve_tgt_ship1_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=1700, speed=700, sig_radius=10000)
    eve_tgt_ship2_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=1500, sig_radius=400)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification - full application vs big target at 0 speed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(2410.714286), approx(2410.714286), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(8437.5), approx(8437.5), 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(2410.714286), approx(2410.714286), 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(8437.5), approx(8437.5), 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(4218.75), approx(4218.75), 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(1205.357143), approx(1205.357143), 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(4218.75), approx(4218.75), 0, 0]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification - lower application at full speed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(1083.283292), approx(1083.283292), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(3791.491522), approx(3791.491522), 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(1083.283292), approx(1083.283292), 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(3791.491522), approx(3791.491522), 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(541.641646), approx(541.641646), 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(1895.745761), approx(1895.745761), 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(541.641646), approx(541.641646), 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(1895.745761), approx(1895.745761), 0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id, movement=(0, 0, 0))
    # Verification - non-perfect application vs a smaller ship even at 0 speed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(482.142857), approx(482.142857), 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(1687.5), approx(1687.5), 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(482.142857), approx(482.142857), 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(1687.5), approx(1687.5), 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(241.071429), approx(241.071429), 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(843.75), approx(843.75), 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(241.071429), approx(241.071429), 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(843.75), approx(843.75), 0, 0]
    # Action
    api_src_fighter_proj.change_fighter(abilities={eve_basic_info.kamikaze_abil_id: True})
    api_src_fighter_nonproj.change_fighter(abilities={eve_basic_info.kamikaze_abil_id: True})
    api_tgt_ship.change_ship(type_id=eve_tgt_ship1_id, movement=(0, 0, 0))
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(60000), approx(60000), approx(60000), approx(60000)]
    assert api_fleet_dmg_stats.volley == [approx(600000), approx(600000), approx(600000), approx(600000)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(60000), approx(60000), approx(60000), approx(60000)]
    assert api_src_fit_dmg_stats.volley == [approx(600000), approx(600000), approx(600000), approx(600000)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_proj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification - speed does not change application
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(60000), approx(60000), approx(60000), approx(60000)]
    assert api_fleet_dmg_stats.volley == [approx(600000), approx(600000), approx(600000), approx(600000)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(60000), approx(60000), approx(60000), approx(60000)]
    assert api_src_fit_dmg_stats.volley == [approx(600000), approx(600000), approx(600000), approx(600000)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_proj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id, movement=(0, 0, 0))
    # Verification - smaller target signature means worse application
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(4800), approx(4800), approx(4800), approx(4800)]
    assert api_fleet_dmg_stats.volley == [approx(48000), approx(48000), approx(48000), approx(48000)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(4800), approx(4800), approx(4800), approx(4800)]
    assert api_src_fit_dmg_stats.volley == [approx(48000), approx(48000), approx(48000), approx(48000)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(2400), approx(2400), approx(2400), approx(2400)]
    assert api_fighter_proj_dmg_stats.volley == [approx(24000), approx(24000), approx(24000), approx(24000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(2400), approx(2400), approx(2400), approx(2400)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(24000), approx(24000), approx(24000), approx(24000)]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification - speed does not change application again
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(4800), approx(4800), approx(4800), approx(4800)]
    assert api_fleet_dmg_stats.volley == [approx(48000), approx(48000), approx(48000), approx(48000)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(4800), approx(4800), approx(4800), approx(4800)]
    assert api_src_fit_dmg_stats.volley == [approx(48000), approx(48000), approx(48000), approx(48000)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(2400), approx(2400), approx(2400), approx(2400)]
    assert api_fighter_proj_dmg_stats.volley == [approx(24000), approx(24000), approx(24000), approx(24000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(2400), approx(2400), approx(2400), approx(2400)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(24000), approx(24000), approx(24000), approx(24000)]


def test_resist(client, consts):
    # Kamikaze ability of shadows can be resisted
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_sec_resist_attr_id = client.mk_eve_attr()
    eve_fighter_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(200, 200, 0, 0), prm_dmg_mult=3.515625, prm_cycle_time=3500,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=2000, prm_exp_speed=60, prm_dr_factor=5, prm_dr_sens=5.5,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000,
        sec_range=500, sec_sig_radius=5000, sec_resist_attr_id=eve_sec_resist_attr_id,
        sq_size=6)
    eve_tgt_ship1_id = make_eve_ship(
        client=client, basic_info=eve_basic_info,
        radius=650, speed=0, sig_radius=10000,
        extra_attrs={eve_sec_resist_attr_id: 1})
    eve_tgt_ship2_id = make_eve_ship(
        client=client, basic_info=eve_basic_info,
        radius=650, speed=0, sig_radius=10000,
        extra_attrs={eve_sec_resist_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.kamikaze_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.kamikaze_abil_id: True})
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(60000), approx(60000), approx(60000), approx(60000)]
    assert api_fleet_dmg_stats.volley == [approx(600000), approx(600000), approx(600000), approx(600000)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(60000), approx(60000), approx(60000), approx(60000)]
    assert api_src_fit_dmg_stats.volley == [approx(600000), approx(600000), approx(600000), approx(600000)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_proj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification - damage from kamikaze ability drops
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(6000), approx(6000), approx(6000), approx(6000)]
    assert api_fleet_dmg_stats.volley == [approx(60000), approx(60000), approx(60000), approx(60000)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(6000), approx(6000), approx(6000), approx(6000)]
    assert api_src_fit_dmg_stats.volley == [approx(60000), approx(60000), approx(60000), approx(60000)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(3000), approx(3000), approx(3000), approx(3000)]
    assert api_fighter_proj_dmg_stats.volley == [approx(30000), approx(30000), approx(30000), approx(30000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(3000), approx(3000), approx(3000), approx(3000)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(30000), approx(30000), approx(30000), approx(30000)]


def test_npc_prop_mode(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(200, 200, 0, 0), prm_dmg_mult=3.515625, prm_cycle_time=3500,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=2000, prm_exp_speed=60, prm_dr_factor=5, prm_dr_sens=5.5,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000,
        sec_range=500, sec_sig_radius=5000,
        sq_size=6, radius=35)
    eve_tgt_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, radius=35,
        speed_cruise=500, speed_chase=1000, sig_radius=100, prop_sig_radius_mult=5)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_drone = api_tgt_fit.add_drone(
        type_id=eve_tgt_drone_id,
        coordinates=(0, 0, 0),
        movement=(0, 0, 0.5),
        npc_prop=consts.ApiNpcProp.cruise)
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_drone.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(18.521979), approx(18.521979), 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(64.826927), approx(64.826927), 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(18.521979), approx(18.521979), 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(64.826927), approx(64.826927), 0, 0]
    # Action
    api_tgt_drone.change_drone(npc_prop=consts.ApiNpcProp.chase)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(43.992547), approx(43.992547), 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(153.973915), approx(153.973915), 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(43.992547), approx(43.992547), 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(153.973915), approx(153.973915), 0, 0]
    # Action
    api_src_fighter_proj.change_fighter(abilities={eve_basic_info.kamikaze_abil_id: True})
    api_src_fighter_nonproj.change_fighter(abilities={eve_basic_info.kamikaze_abil_id: True})
    api_tgt_drone.change_drone(npc_prop=consts.ApiNpcProp.cruise)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(600), approx(600), approx(600), approx(600)]
    assert api_fighter_proj_dmg_stats.volley == [approx(6000), approx(6000), approx(6000), approx(6000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(600), approx(600), approx(600), approx(600)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(6000), approx(6000), approx(6000), approx(6000)]
    # Action
    api_tgt_drone.change_drone(npc_prop=consts.ApiNpcProp.chase)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(3000), approx(3000), approx(3000), approx(3000)]
    assert api_fighter_proj_dmg_stats.volley == [approx(30000), approx(30000), approx(30000), approx(30000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(3000), approx(3000), approx(3000), approx(3000)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(30000), approx(30000), approx(30000), approx(30000)]


def test_ftr_sec_attr_range_absent(client, consts):
    # No range defined - it is considered equal to be 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000, sec_sig_radius=5000,
        sq_size=6, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=1700, speed=700, sig_radius=10000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.kamikaze_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.kamikaze_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 1734, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_proj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 1736, 0))
    # Verification - just out of kamikaze range
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_ftr_sec_attr_sig_radius(client, consts):
    # Absent/negative values are the same as 0 value, which means perfect application
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_absent_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000,
        sq_size=6)
    eve_fighter_zero_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000, sec_sig_radius=0,
        sq_size=6)
    eve_fighter_negative_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000, sec_sig_radius=-5000,
        sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=1700, speed=700, sig_radius=10000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_absent_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.kamikaze_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_absent_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.kamikaze_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_proj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_zero_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_proj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_negative_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_proj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(30000), approx(30000), approx(30000), approx(30000)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(300000), approx(300000), approx(300000), approx(300000)]


def test_tgt_attr_speed(client, consts):
    # Absent/negative values are the same as 0 value, which means speed has no effect
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(200, 200, 0, 0), prm_dmg_mult=3.515625, prm_cycle_time=3500,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=2000, prm_exp_speed=60, prm_dr_factor=5, prm_dr_sens=5.5,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000,
        sec_range=500, sec_sig_radius=5000,
        sq_size=6)
    eve_tgt_ship_absent_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, sig_radius=400)
    eve_tgt_ship_zero_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=500, speed=0, sig_radius=400)
    eve_tgt_ship_negative_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, radius=500, speed=-1500, sig_radius=400)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_absent_id, coordinates=(0, 0, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(241.071429), approx(241.071429), 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(843.75), approx(843.75), 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(241.071429), approx(241.071429), 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(843.75), approx(843.75), 0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(241.071429), approx(241.071429), 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(843.75), approx(843.75), 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(241.071429), approx(241.071429), 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(843.75), approx(843.75), 0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(241.071429), approx(241.071429), 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(843.75), approx(843.75), 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(241.071429), approx(241.071429), 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(843.75), approx(843.75), 0, 0]
    # Action
    api_src_fighter_proj.change_fighter(abilities={eve_basic_info.kamikaze_abil_id: True})
    api_src_fighter_nonproj.change_fighter(abilities={eve_basic_info.kamikaze_abil_id: True})
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_absent_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(2400), approx(2400), approx(2400), approx(2400)]
    assert api_fighter_proj_dmg_stats.volley == [approx(24000), approx(24000), approx(24000), approx(24000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(2400), approx(2400), approx(2400), approx(2400)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(24000), approx(24000), approx(24000), approx(24000)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(2400), approx(2400), approx(2400), approx(2400)]
    assert api_fighter_proj_dmg_stats.volley == [approx(24000), approx(24000), approx(24000), approx(24000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(2400), approx(2400), approx(2400), approx(2400)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(24000), approx(24000), approx(24000), approx(24000)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(2400), approx(2400), approx(2400), approx(2400)]
    assert api_fighter_proj_dmg_stats.volley == [approx(24000), approx(24000), approx(24000), approx(24000)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(2400), approx(2400), approx(2400), approx(2400)]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(24000), approx(24000), approx(24000), approx(24000)]


def test_tgt_attr_sig_radius(client, consts):
    # Absent/negative values are the same as 0 value, which means zero application from either
    # ability
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(200, 200, 0, 0), prm_dmg_mult=3.515625, prm_cycle_time=3500,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=2000, prm_exp_speed=60, prm_dr_factor=5, prm_dr_sens=5.5,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000,
        sec_range=500, sec_sig_radius=5000,
        sq_size=6)
    eve_tgt_ship_absent_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=1700, speed=700)
    eve_tgt_ship_zero_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=1700, speed=700, sig_radius=0)
    eve_tgt_ship_negative_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, radius=1700, speed=700, sig_radius=-10000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_absent_id, coordinates=(0, 0, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_src_fighter_proj.change_fighter(abilities={eve_basic_info.kamikaze_abil_id: True})
    api_src_fighter_nonproj.change_fighter(abilities={eve_basic_info.kamikaze_abil_id: True})
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_absent_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_tgt_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_shadow(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(200, 200, 0, 0), prm_dmg_mult=3.515625, prm_cycle_time=3500,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=2000, prm_exp_speed=60, prm_dr_factor=5, prm_dr_sens=5.5,
        sec_dmgs=(50000, 50000, 50000, 50000), sec_cycle_time=10000,
        sec_range=500, sec_sig_radius=5000,
        sq_size=6)
    eve_tgt_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_src_fighter_proj.change_fighter(abilities={eve_basic_info.kamikaze_abil_id: True})
    api_src_fighter_nonproj.change_fighter(abilities={eve_basic_info.kamikaze_abil_id: True})
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
