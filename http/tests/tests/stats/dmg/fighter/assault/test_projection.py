from fw import approx
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionFitDmg, StatsOptionItemDmg
from tests.stats.dmg import make_eve_fighter_assault, make_eve_ship, setup_dmg_basics


def test_projection(client, consts):
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
