from fw import approx
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionFitDmg, StatsOptionItemDmg
from tests.stats.dmg import make_eve_drone, make_eve_fighter_assault, make_eve_ship, setup_dmg_basics


def test_range(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=0, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 12584, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification - within optimal for both abilities
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(1102.479911), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(9420.46875), 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(1102.479911), 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(9420.46875), 0, 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 13084, 0))
    # Verification - damage of primary ability drops a bit, but secondary one is at full power
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(1097.882352), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(9397.480959), 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(1097.882352), 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(9397.480959), 0, 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(548.941176), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(4698.740479), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(548.941176), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(4698.740479), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 13086, 0))
    # Verification - damage of primary ability drops a bit, but secondary one is out of range
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(663.615642), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(3318.07821), 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(663.615642), 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(3318.07821), 0, 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(331.807821), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(1659.039105), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(331.807821), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(1659.039105), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 17585, 0))
    # Verification - first falloff of primary ability
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(334.125), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(1670.625), 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(334.125), 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(1670.625), 0, 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(167.0625), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(835.3125), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(167.0625), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(835.3125), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 22585, 0))
    # Verification - second falloff of primary ability
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(41.765625), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(208.828125), 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(41.765625), 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(208.828125), 0, 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(20.882813), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(104.414063), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(20.882813), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(104.414063), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 27585, 0))
    # Verification - third falloff of primary ability
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(1.305176), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(6.525879), 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(1.305176), 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(6.525879), 0, 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(0.6525879), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(3.262939), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(0.6525879), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(3.262939), 0, 0, 0]


def test_application(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_tgt_ship1_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=650, speed=1800, sig_radius=1920)
    eve_tgt_ship2_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=27, speed=4200, sig_radius=32)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification - full application vs big target at 0 speed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(1102.479911), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(9420.46875), 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(1102.479911), 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(9420.46875), 0, 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 0.5))
    # Verification - lower application at half speed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(839.543151), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(5739.354118), 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(839.543151), 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(5739.354118), 0, 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(419.771576), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(2869.677059), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(419.771576), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(2869.677059), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 1))
    # Verification - even lower at full target speed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(572.624126), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(3664.397667), 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(572.624126), 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(3664.397667), 0, 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(286.312063), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(1832.198834), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(286.312063), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(1832.198834), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification - much worse application vs fast AB frig
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(20.856452), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(111.826957), 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(20.856452), 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(111.826957), 0, 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(10.428226), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(55.913478), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(10.428226), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(55.913478), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 0))
    # Verification - non-perfect application even if frig is at 0 speed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(150.327582), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(1064.283446), 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(150.327582), 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(1064.283446), 0, 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]


def test_resist(client, consts):
    # Secondary ability of heavy assault fighters can be resisted
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_sec_resist_attr_id = client.mk_eve_attr()
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500, sec_resist_attr_id=eve_sec_resist_attr_id,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_tgt_ship1_id = make_eve_ship(
        client=client, basic_info=eve_basic_info,
        radius=650, speed=0, sig_radius=1000,
        extra_attrs={eve_sec_resist_attr_id: 1})
    eve_tgt_ship2_id = make_eve_ship(
        client=client, basic_info=eve_basic_info,
        radius=650, speed=0, sig_radius=1000,
        extra_attrs={eve_sec_resist_attr_id: 0.1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 0, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(1102.479911), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(9420.46875), 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(1102.479911), 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(9420.46875), 0, 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification - damage from secondary ability drops, primary damage stays the same
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [approx(711.672991), 0, 0, 0]
    assert api_fleet_dmg_stats.volley == [approx(3949.171875), 0, 0, 0]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [approx(711.672991), 0, 0, 0]
    assert api_src_fit_dmg_stats.volley == [approx(3949.171875), 0, 0, 0]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(355.836496), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(1974.585938), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(355.836496), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(1974.585938), 0, 0, 0]


def test_npc_prop_mode(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_tgt_drone_id = make_eve_drone(
        client=client, basic_info=eve_basic_info, radius=35,
        speed_cruise=500, speed_chase=1000, sig_radius=100, prop_sig_radius_mult=5)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
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
    assert api_fighter_proj_dmg_stats.dps == [approx(146.144538), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(889.430869), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(146.144538), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(889.430869), 0, 0, 0]
    # Action
    api_tgt_drone.change_drone(npc_prop=consts.ApiNpcProp.chase)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(273.830318), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(1746.107905), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(273.830318), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(1746.107905), 0, 0, 0]
    # Action
    api_tgt_drone.change_drone(npc_prop=consts.ApiNpcProp.cruise)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(146.144538), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(889.430869), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(146.144538), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(889.430869), 0, 0, 0]


def test_ftr_effect_range_optimal_absent(client, consts):
    # No optimal range defined - it is considered equal to be 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts, effect_range=False)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=0, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 586, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification - in beginning of falloff for primary ability, out of range for secondary
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(334.124991), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(1670.624954), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(334.124991), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(1670.624954), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 5585, 0))
    # Verification - first falloff of primary ability
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(167.0625), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(835.3125), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(167.0625), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(835.3125), 0, 0, 0]


def test_ftr_effect_range_falloff_absent(client, consts):
    # No falloff range defined - it is considered equal to be 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts, effect_falloff=False)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=0, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 12584, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification - both abilities are at full power
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 12586, 0))
    # Verification - primary ability is out of range
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(217.114955), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(3039.609375), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(217.114955), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(3039.609375), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 13084, 0))
    # Verification - secondary is still within range
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(217.114955), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(3039.609375), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(217.114955), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(3039.609375), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 13086, 0))
    # Verification - both are out of range
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_ftr_attr_range_optimal_absent(client, consts):
    # No optimal range defined - it is considered equal to be 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=0, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 586, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification - in beginning of falloff for primary ability, out of range for secondary
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(334.124991), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(1670.624954), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(334.124991), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(1670.624954), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 5585, 0))
    # Verification - first falloff of primary ability
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(167.0625), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(835.3125), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(167.0625), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(835.3125), 0, 0, 0]


def test_ftr_attr_range_falloff_absent(client, consts):
    # No falloff range defined - it is considered equal to be 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=0, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 12584, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification - both abilities are at full power
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 12586, 0))
    # Verification - primary ability is out of range
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(217.114955), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(3039.609375), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(217.114955), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(3039.609375), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 13084, 0))
    # Verification - secondary is still within range
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(217.114955), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(3039.609375), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(217.114955), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(3039.609375), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 13086, 0))
    # Verification - both are out of range
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_ftr_attr_exp_radius(client, consts):
    # Absent/negative values are the same as 0 value, which means perfect application
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_absent_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_fighter_zero_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=0, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=0, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_fighter_negative_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=-185, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=-400, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=27, speed=4200, sig_radius=32)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_absent_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_absent_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0.5))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_zero_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_negative_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(551.239955), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(4710.234375), 0, 0, 0]


def test_ftr_attr_exp_speed(client, consts):
    # Absent/negative values are the same as 0 value, which means zero application
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_absent_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_fighter_zero_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=0, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=0, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_fighter_negative_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=-105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=-70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=27, speed=4200, sig_radius=32)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_absent_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_absent_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0.5))
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
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_zero_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_zero_id)
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
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_negative_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_negative_id)
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
    api_tgt_ship.change_ship(movement=(0, 0, 0))
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
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_zero_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_zero_id)
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
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_absent_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_absent_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_ftr_attr_drf(client, consts):
    # Absent/negative/zero value of first part of composite DRF results in zero value of DRF
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_absent_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_fighter_zero_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=0, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=0, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_fighter_negative_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=-3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=-5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=27, speed=4200, sig_radius=32)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_absent_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_absent_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0.5))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_zero_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_negative_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]


def test_ftr_attr_drs(client, consts):
    # Absent/negative/zero value of second part of composite DRF results in zero value of DRF
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_absent_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=3,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=5,
        sq_size=6, radius=35)
    eve_fighter_zero_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=0,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=0,
        sq_size=6, radius=35)
    eve_fighter_negative_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=-5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=-5.5,
        sq_size=6, radius=35)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=27, speed=4200, sig_radius=32)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_absent_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_absent_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 0.5))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_zero_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_negative_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]


def test_tgt_attr_speed(client, consts):
    # Absent/negative values are the same as 0 value, which means speed has no effect
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_tgt_ship_absent_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=27, sig_radius=32)
    eve_tgt_ship_zero_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=27, speed=0, sig_radius=32)
    eve_tgt_ship_negative_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, radius=27, speed=-4200, sig_radius=32)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_absent_id, coordinates=(0, 0, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_projs=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [approx(75.163791), 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [approx(532.141723), 0, 0, 0]


def test_tgt_attr_sig_radius(client, consts):
    # Absent/negative values are the same as 0 value, which means zero application
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_tgt_ship_absent_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=27, speed=4200)
    eve_tgt_ship_zero_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=27, speed=4200, sig_radius=0)
    eve_tgt_ship_negative_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, radius=27, speed=4200, sig_radius=-32)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
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


def test_tgt_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_assault(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(108, 0, 0, 0), prm_dmg_mult=2.578125, prm_cycle_time=5000,
        prm_range_optimal=12000, prm_range_falloff=5000,
        prm_exp_radius=185, prm_exp_speed=105, prm_dr_factor=3, prm_dr_sens=5.5,
        sec_dmgs=(196.5, 0, 0, 0), sec_dmg_mult=2.578125, sec_cycle_time=14000,
        sec_range=12500,
        sec_exp_radius=400, sec_exp_speed=70, sec_dr_factor=5, sec_dr_sens=5.5,
        sq_size=6, radius=35)
    eve_tgt_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.missiles_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 1))
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
