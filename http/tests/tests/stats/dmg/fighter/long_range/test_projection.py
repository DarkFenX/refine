from fw import approx
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionFitDmg, StatsOptionItemDmg
from tests.stats.dmg import make_eve_bomb, make_eve_drone, make_eve_fighter_lr, make_eve_ship, setup_dmg_basics


def test_range(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=5.5,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000, sec_charge_count=5, sec_charge_rearm_time=20,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=0, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True})
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 14449, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification - within optimal of primary attack, out of bomb explosion radius
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(1397.460938)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(11179.6875)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, approx(1397.460938)]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, approx(11179.6875)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(698.730469)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(5589.84375)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(698.730469)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(5589.84375)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 14450, 0))
    # Verification - within optimal of primary attack, within bomb explosion radius
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(1525.460938)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(18859.6875)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, approx(1525.460938)]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, approx(18859.6875)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(762.730469)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(762.730469)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 39584, 0))
    # Verification - just within optimal of primary attack
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(1525.460938)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(18859.6875)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, approx(1525.460938)]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, approx(18859.6875)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(762.730469)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(762.730469)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 45549, 0))
    # Verification - outside optimal of primary attack, still within bomb range
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(1397.152467)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(17833.219732)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, approx(1397.152467)]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, approx(17833.219732)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(698.576233)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(8916.609866)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(698.576233)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(8916.609866)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 45550, 0))
    # Verification - outside optimal of primary attack, just outside bomb range
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(1269.111475)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(10152.891798)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, approx(1269.111475)]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, approx(10152.891798)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(634.555737)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(5076.445899)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(634.555737)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(5076.445899)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 55585, 0))
    # Verification - first falloff of primary attack
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(698.730469)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(5589.84375)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, approx(698.730469)]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, approx(5589.84375)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(349.365234)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(2794.921875)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(349.365234)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(2794.921875)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 71585, 0))
    # Verification - second falloff of primary attack
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(87.341309)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(698.730469)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, approx(87.341309)]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, approx(698.730469)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(43.670654)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(349.365234)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(43.670654)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(349.365234)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 87586, 0))
    # Verification - just outside third falloff of primary attack
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(2.728707)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(21.829652)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, approx(2.728707)]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, approx(21.829652)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(1.364353)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(10.914826)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(1.364353)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(10.914826)]


def test_application(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=5.5,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000, sec_charge_count=5, sec_charge_rearm_time=20,
        radius=35, sq_size=6)
    eve_tgt_ship1_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=650, speed=750, sig_radius=420)
    eve_tgt_ship2_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=75, sig_radius=1000)
    eve_tgt_ship3_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=27, speed=4200, sig_radius=32)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True})
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 30000, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification - reduced application from primary attack at full speed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(254.619141)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(8692.95313)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, approx(254.619141)]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, approx(8692.95313)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(127.309571)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(4346.476565)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(127.309571)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(4346.476565)]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 0.5))
    # Verification - increased application after ship slows down to half
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(371.61228)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(9628.898238)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, approx(371.61228)]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, approx(9628.898238)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(185.80614)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(4814.449119)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(185.80614)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(4814.449119)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification - full application from a primary attack to a bigger ship despite some speed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(1525.460938)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(18859.6875)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, approx(1525.460938)]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, approx(18859.6875)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(762.730469)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(762.730469)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(9429.84375)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship3_id)
    # Verification - poor application to a small ship at half speed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(14.454572)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(648.116575)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, approx(14.454572)]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, approx(648.116575)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(7.227286)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(324.058287)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(7.227286)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(324.058287)]
    # Action
    api_tgt_ship.change_ship(movement=(0, 0, 0))
    # Verification - still bad application despite 0 speed
    api_fleet_dmg_stats = api_fleet.get_stats(options=FleetStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_fleet_dmg_stats.dps == [0, 0, 0, approx(88.693947)]
    assert api_fleet_dmg_stats.volley == [0, 0, 0, approx(1242.031579)]
    api_src_fit_dmg_stats = api_src_fit.get_stats(options=FitStatsOptions(
        dmg=(True, [StatsOptionFitDmg(projectee_item_id=api_tgt_ship.id)]))).dmg.one()
    assert api_src_fit_dmg_stats.dps == [0, 0, 0, approx(88.693947)]
    assert api_src_fit_dmg_stats.volley == [0, 0, 0, approx(1242.031579)]
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(44.346974)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(621.015789)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(44.346974)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(621.015789)]


def test_npc_prop_mode(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=5.5,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000, sec_charge_count=5, sec_charge_rearm_time=20,
        radius=35, sq_size=6)
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
        coordinates=(0, 0, 30000),
        movement=(0, 0, 0.5),
        npc_prop=consts.ApiNpcProp.cruise)
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_drone.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(46.07986)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(368.638883)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(46.07986)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(368.638883)]
    # Action
    api_tgt_drone.change_drone(npc_prop=consts.ApiNpcProp.chase)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(109.44675)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(875.574003)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(109.44675)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(875.574003)]
    # Action
    api_tgt_drone.change_drone(npc_prop=consts.ApiNpcProp.cruise)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(46.07986)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(368.638883)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_drone.id)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(46.07986)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(368.638883)]


def test_ftr_prm_effect_range_optimal_absent(client, consts):
    # No optimal range defined - it is considered equal to be 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts, effect_range=False)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=5.5,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=650, speed=750, sig_radius=420)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 685, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(63.309571)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(506.476565)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(63.309571)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(506.476565)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 16685, 0))
    # Verification - first falloff of primary ability
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(31.654785)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(253.238282)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(31.654785)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(253.238282)]


def test_ftr_prm_effect_range_falloff_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts, effect_falloff=False)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=5.5,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=650, speed=750, sig_radius=420)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 39684, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(63.309571)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(506.476565)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(63.309571)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(506.476565)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 39686, 0))
    # Verification - just out of optimal range
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_ftr_prm_attr_range_optimal_absent(client, consts):
    # No optimal range defined - it is considered equal to be 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=5.5,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=650, speed=750, sig_radius=420)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 685, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(63.309571)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(506.476565)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(63.309571)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(506.476565)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 16685, 0))
    # Verification - first falloff of primary ability
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(31.654785)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(253.238282)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(31.654785)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(253.238282)]


def test_ftr_prm_attr_range_falloff_absent(client, consts):
    # No falloff range defined - it is considered equal to be 0
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=5.5,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=650, speed=750, sig_radius=420)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(type_id=eve_fighter_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 39684, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(63.309571)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(506.476565)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(63.309571)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(506.476565)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 39686, 0))
    # Verification - just out of optimal range
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_ftr_prm_attr_exp_radius(client, consts):
    # Absent/negative values are the same as 0 value, which means perfect application
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_absent_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=5.5,
        radius=35, sq_size=6)
    eve_fighter_zero_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=0, prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=5.5,
        radius=35, sq_size=6)
    eve_fighter_negative_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=-570, prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=5.5,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=650, speed=750, sig_radius=420)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_absent_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_absent_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(698.730469)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(5589.84375)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(698.730469)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(5589.84375)]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_zero_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(698.730469)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(5589.84375)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(698.730469)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(5589.84375)]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_negative_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(698.730469)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(5589.84375)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(698.730469)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(5589.84375)]


def test_ftr_prm_attr_exp_speed(client, consts):
    # Absent/negative values are the same as 0 value, which means zero application
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_absent_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_dr_factor=5, prm_dr_sens=5.5,
        radius=35, sq_size=6)
    eve_fighter_zero_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=0, prm_dr_factor=5, prm_dr_sens=5.5,
        radius=35, sq_size=6)
    eve_fighter_negative_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=-80, prm_dr_factor=5, prm_dr_sens=5.5,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=650, speed=750, sig_radius=420)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_absent_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_absent_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_zero_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_negative_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_ftr_prm_attr_drf(client, consts):
    # Absent/negative/zero value of first part of composite DRF results in zero value of DRF
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_absent_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_sens=5.5,
        radius=35, sq_size=6)
    eve_fighter_zero_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=0, prm_dr_sens=5.5,
        radius=35, sq_size=6)
    eve_fighter_negative_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=-5, prm_dr_sens=5.5,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=650, speed=750, sig_radius=420)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_absent_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_absent_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(514.85403)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(4118.832237)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(514.85403)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(4118.832237)]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_zero_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(514.85403)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(4118.832237)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(514.85403)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(4118.832237)]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_negative_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(514.85403)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(4118.832237)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(514.85403)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(4118.832237)]


def test_ftr_prm_attr_drs(client, consts):
    # Absent/negative/zero value of second part of composite DRF results in zero value of DRF
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_fighter_absent_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=5,
        radius=35, sq_size=6)
    eve_fighter_zero_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=0,
        radius=35, sq_size=6)
    eve_fighter_negative_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=-5.5,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=650, speed=750, sig_radius=420)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(type_id=eve_fighter_absent_id, state=consts.ApiMinionState.engaging)
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_absent_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 0, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(514.85403)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(4118.832237)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(514.85403)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(4118.832237)]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_zero_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(514.85403)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(4118.832237)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(514.85403)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(4118.832237)]
    # Action
    api_src_fighter_proj.change_fighter(type_id=eve_fighter_negative_id)
    api_src_fighter_nonproj.change_fighter(type_id=eve_fighter_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(514.85403)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(4118.832237)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(514.85403)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(4118.832237)]


def test_ftr_sec_attr_speed_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10,
        flight_time=12000, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000, sec_charge_count=5, sec_charge_rearm_time=20,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=0, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.atkm_abil_id: False, eve_basic_info.bomb_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.atkm_abil_id: False, eve_basic_info.bomb_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, -15551, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, -15549, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 15549, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 15551, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_ftr_sec_attr_flight_time_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10,
        speed=2500, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000, sec_charge_count=5, sec_charge_rearm_time=20,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=0, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.atkm_abil_id: False, eve_basic_info.bomb_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.atkm_abil_id: False, eve_basic_info.bomb_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, -15551, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, -15549, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 15549, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 15551, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_ftr_sec_attr_mass_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10,
        speed=2500, flight_time=12000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000, sec_charge_count=5, sec_charge_rearm_time=20,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=0, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.atkm_abil_id: False, eve_basic_info.bomb_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.atkm_abil_id: False, eve_basic_info.bomb_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 14449, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 14451, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 45549, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 45551, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_ftr_sec_attr_ability_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10,
        speed=2500, flight_time=12000, mass=1000, exp_range=15000, exp_radius=400)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000, sec_charge_count=5, sec_charge_rearm_time=20,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=0, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.atkm_abil_id: False, eve_basic_info.bomb_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.atkm_abil_id: False, eve_basic_info.bomb_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 14449, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 14451, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 45549, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 45551, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_ftr_sec_attr_exp_range_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_radius=400)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000, sec_charge_count=5, sec_charge_rearm_time=20,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=0, sig_radius=1000)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.atkm_abil_id: False, eve_basic_info.bomb_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.atkm_abil_id: False, eve_basic_info.bomb_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 29449, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 29451, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 30549, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 30551, 0))
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_ftr_sec_attr_exp_radius_absent(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_range=15000)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000, sec_charge_count=5, sec_charge_rearm_time=20,
        radius=35, sq_size=6)
    eve_tgt_ship_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=550, speed=0, sig_radius=100)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.atkm_abil_id: False, eve_basic_info.bomb_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.atkm_abil_id: False, eve_basic_info.bomb_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 30000, 0), movement=(0, 0, 0))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification - full damage is taken
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(3840)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(64)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(3840)]


def test_tgt_attr_speed(client, consts):
    # Absent/negative values are the same as 0 value, which means speed has no effect
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=5.5,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000, sec_charge_count=5, sec_charge_rearm_time=20,
        radius=35, sq_size=6)
    eve_tgt_ship_absent_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=650, sig_radius=420)
    eve_tgt_ship_zero_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=650, speed=0, sig_radius=420)
    eve_tgt_ship_negative_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, radius=650, speed=-750, sig_radius=420)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_absent_id, coordinates=(0, 30000, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(578.85403)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(7958.832237)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(578.85403)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(7958.832237)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(578.85403)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(7958.832237)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(578.85403)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(7958.832237)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, approx(578.85403)]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, approx(7958.832237)]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, approx(578.85403)]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, approx(7958.832237)]


def test_tgt_attr_sig_radius(client, consts):
    # Absent/negative values are the same as 0 value, which means zero application
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=5.5,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000, sec_charge_count=5, sec_charge_rearm_time=20,
        radius=35, sq_size=6)
    eve_tgt_ship_absent_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=650, speed=750)
    eve_tgt_ship_zero_id = make_eve_ship(client=client, basic_info=eve_basic_info, radius=650, speed=750, sig_radius=0)
    eve_tgt_ship_negative_id = make_eve_ship(
        client=client, basic_info=eve_basic_info, radius=650, speed=750, sig_radius=-420)
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_absent_id, coordinates=(0, 30000, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_zero_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship_negative_id)
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]


def test_tgt_not_loaded(client, consts):
    eve_basic_info = setup_dmg_basics(client=client, consts=consts)
    eve_bomb_id = make_eve_bomb(
        client=client, basic_info=eve_basic_info, dmgs=(0, 0, 0, 640), volume=10,
        speed=2500, flight_time=12000, mass=1000, agility=0.0275, exp_range=15000, exp_radius=400)
    eve_fighter_id = make_eve_fighter_lr(
        client=client, basic_info=eve_basic_info,
        prm_dmgs=(0, 0, 0, 265), prm_dmg_mult=3.515625, prm_cycle_time=8000,
        prm_range_optimal=39000, prm_range_falloff=16000,
        prm_exp_radius=570, prm_exp_speed=80, prm_dr_factor=5, prm_dr_sens=5.5,
        sec_bomb_type_id=eve_bomb_id, sec_cycle_time=60000, sec_charge_count=5, sec_charge_rearm_time=20,
        radius=35, sq_size=6)
    eve_tgt_ship_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fighter_proj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True})
    api_src_fighter_nonproj = api_src_fit.add_fighter(
        type_id=eve_fighter_id,
        state=consts.ApiMinionState.engaging,
        abilities={eve_basic_info.bomb_abil_id: True})
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 30000, 0), movement=(0, 0, 1))
    api_src_fighter_proj.change_fighter(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fighter_proj_dmg_stats = api_src_fighter_proj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_proj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_proj_dmg_stats.volley == [0, 0, 0, 0]
    api_fighter_nonproj_dmg_stats = api_src_fighter_nonproj.get_stats(options=ItemStatsOptions(
        dmg=(True, [StatsOptionItemDmg(projectee_item_id=api_tgt_ship.id, include_charges=True)]))).dmg.one()
    assert api_fighter_nonproj_dmg_stats.dps == [0, 0, 0, 0]
    assert api_fighter_nonproj_dmg_stats.volley == [0, 0, 0, 0]
