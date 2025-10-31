from tests import approx
from tests.fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatNeutItemKinds,
    StatsOptionFitRemoteNps,
    StatsOptionItemRemoteNps,
)


def test_state(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reactivation_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.module_reactivation_delay)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.use_missiles,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_bomb_effect_id = client.mk_eve_effect(id_=consts.EveEffect.bomb_launching, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_cycle_time_attr_id: 10000, eve_reactivation_time_attr_id: 67500, eve_capacity_attr_id: 300},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 1800, eve_volume_attr_id: 75},
        eff_ids=[eve_bomb_effect_id],
        defeff_id=eve_bomb_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == approx(23.225806)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == approx(23.225806)
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(remote_nps=True))
    assert api_charge_stats.remote_nps.one() == approx(23.225806)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == 0
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(remote_nps=(True, [
        StatsOptionItemRemoteNps(ignore_state=False),
        StatsOptionItemRemoteNps(ignore_state=True)])))
    assert api_charge_stats.remote_nps == [0, approx(23.225806)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == approx(23.225806)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == approx(23.225806)
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(remote_nps=True))
    assert api_charge_stats.remote_nps.one() == approx(23.225806)


def test_range_and_cap_limit(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reactivation_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.module_reactivation_delay)
    eve_flight_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_mass_attr_id = client.mk_eve_attr(id_=consts.EveAttr.mass)
    eve_agility_attr_id = client.mk_eve_attr(id_=consts.EveAttr.agility)
    eve_expl_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.emp_field_range)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.use_missiles,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_bomb_effect_id = client.mk_eve_effect(id_=consts.EveEffect.bomb_launching, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_cycle_time_attr_id: 10000, eve_reactivation_time_attr_id: 67500, eve_capacity_attr_id: 300},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_charge_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 1800, eve_volume_attr_id: 75, eve_flight_speed_attr_id: 4000,
            eve_flight_time_attr_id: 7500, eve_mass_attr_id: 1000, eve_agility_attr_id: 0.0000251,
            eve_expl_range_attr_id: 15000},
        eff_ids=[eve_bomb_effect_id],
        defeff_id=eve_bomb_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 20.5})
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_radius_attr_id: 220, eve_sig_radius_attr_id: 200, eve_cap_attr_id: 2500})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_radius_attr_id: 220, eve_sig_radius_attr_id: 200, eve_cap_attr_id: 1500})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 12700, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == 0
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == 0
    api_src_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_proj_stats.remote_nps.one() == 0
    api_src_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_nonproj_stats.remote_nps.one() == 0
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 12800, 0))
    # Verification - 50% chance to hit, limit is ineffective because target has more cap than bomb
    # removes
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(23.225806)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(23.225806)
    api_src_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_proj_stats.remote_nps.one() == approx(11.612903)
    api_src_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_nonproj_stats.remote_nps.one() == approx(11.612903)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification - limit is effective because target cap is now below how much bomb removes, but
    # chance to hit is 50%, so post-limit value is reduced
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(19.354839)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(19.354839)
    api_src_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_proj_stats.remote_nps.one() == approx(9.677419)
    api_src_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_nonproj_stats.remote_nps.one() == approx(9.677419)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 16700, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(19.354839)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(19.354839)
    api_src_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_proj_stats.remote_nps.one() == approx(9.677419)
    api_src_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_nonproj_stats.remote_nps.one() == approx(9.677419)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 16800, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(38.709677)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(38.709677)
    api_src_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_proj_stats.remote_nps.one() == approx(19.354839)
    api_src_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_nonproj_stats.remote_nps.one() == approx(19.354839)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship1_id)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(46.451613)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(46.451613)
    api_src_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_proj_stats.remote_nps.one() == approx(23.225806)
    api_src_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_nonproj_stats.remote_nps.one() == approx(23.225806)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 43200, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(46.451613)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(46.451613)
    api_src_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_proj_stats.remote_nps.one() == approx(23.225806)
    api_src_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_nonproj_stats.remote_nps.one() == approx(23.225806)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 43300, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(23.225806)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(23.225806)
    api_src_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_proj_stats.remote_nps.one() == approx(11.612903)
    api_src_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_nonproj_stats.remote_nps.one() == approx(11.612903)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 47200, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(23.225806)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(23.225806)
    api_src_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_proj_stats.remote_nps.one() == approx(11.612903)
    api_src_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_nonproj_stats.remote_nps.one() == approx(11.612903)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 47300, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == 0
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == 0
    api_src_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_proj_stats.remote_nps.one() == 0
    api_src_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_nonproj_stats.remote_nps.one() == 0


def test_application_and_cap_limit(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reactivation_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.module_reactivation_delay)
    eve_flight_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_expl_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.emp_field_range)
    eve_expl_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.aoe_cloud_size)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.use_missiles,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_bomb_effect_id = client.mk_eve_effect(id_=consts.EveEffect.bomb_launching, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_cycle_time_attr_id: 10000, eve_reactivation_time_attr_id: 67500, eve_capacity_attr_id: 300},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_charge_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 1800, eve_volume_attr_id: 75, eve_flight_speed_attr_id: 4000,
            eve_flight_time_attr_id: 7500, eve_expl_range_attr_id: 15000, eve_expl_radius_attr_id: 400},
        eff_ids=[eve_bomb_effect_id],
        defeff_id=eve_bomb_effect_id)
    eve_src_ship_id = client.mk_eve_ship()
    eve_tgt_ship1_id = client.mk_eve_ship(attrs={eve_sig_radius_attr_id: 200, eve_cap_attr_id: 1200})
    eve_tgt_ship2_id = client.mk_eve_ship(attrs={eve_sig_radius_attr_id: 200, eve_cap_attr_id: 600})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 30000, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(23.225806)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(23.225806)
    api_src_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_proj_stats.remote_nps.one() == approx(11.612903)
    api_src_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_nonproj_stats.remote_nps.one() == approx(11.612903)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(15.483871)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(15.483871)
    api_src_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_proj_stats.remote_nps.one() == approx(7.741935)
    api_src_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_nonproj_stats.remote_nps.one() == approx(7.741935)


def test_resist_and_cap_limit(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reactivation_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.module_reactivation_delay)
    eve_flight_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.max_velocity)
    eve_flight_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.explosion_delay)
    eve_expl_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.emp_field_range)
    eve_resist_attr_id = client.mk_eve_attr()
    eve_resist_def_attr_id = client.mk_eve_attr(id_=consts.EveAttr.remote_resistance_id)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.use_missiles,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_bomb_effect_id = client.mk_eve_effect(id_=consts.EveEffect.bomb_launching, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_cycle_time_attr_id: 10000, eve_reactivation_time_attr_id: 67500, eve_capacity_attr_id: 300},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_charge_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 1800, eve_volume_attr_id: 75, eve_flight_speed_attr_id: 4000,
            eve_flight_time_attr_id: 7500, eve_expl_range_attr_id: 15000, eve_resist_def_attr_id: eve_resist_attr_id},
        eff_ids=[eve_bomb_effect_id],
        defeff_id=eve_bomb_effect_id)
    eve_src_ship_id = client.mk_eve_ship()
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_sig_radius_attr_id: 200, eve_resist_attr_id: 0.8, eve_cap_attr_id: 1000})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_sig_radius_attr_id: 200, eve_resist_attr_id: 0.4, eve_cap_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 30000, 0))
    api_src_module_proj = api_src_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_src_module_nonproj = api_src_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(25.806452)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(25.806452)
    api_src_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_proj_stats.remote_nps.one() == approx(12.903226)
    api_src_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_nonproj_stats.remote_nps.one() == approx(12.903226)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(18.580645)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(18.580645)
    api_src_charge_proj_stats = api_src_module_proj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_proj_stats.remote_nps.one() == approx(9.290323)
    api_src_charge_nonproj_stats = api_src_module_nonproj.charge.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_charge_nonproj_stats.remote_nps.one() == approx(9.290323)


def test_item_kind(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reactivation_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.module_reactivation_delay)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.use_missiles,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_bomb_effect_id = client.mk_eve_effect(id_=consts.EveEffect.bomb_launching, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_cycle_time_attr_id: 10000, eve_reactivation_time_attr_id: 67500, eve_capacity_attr_id: 300},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 1800, eve_volume_attr_id: 75},
        eff_ids=[eve_bomb_effect_id],
        defeff_id=eve_bomb_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active, charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=(True, [
        StatsOptionFitRemoteNps(),
        StatsOptionFitRemoteNps(item_kinds=StatNeutItemKinds(default=False, bomb=True)),
        StatsOptionFitRemoteNps(item_kinds=StatNeutItemKinds(default=True, bomb=False))])))
    assert api_fleet_stats.remote_nps == [approx(23.225806), approx(23.225806), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=(True, [
        StatsOptionFitRemoteNps(),
        StatsOptionFitRemoteNps(item_kinds=StatNeutItemKinds(default=False, bomb=True)),
        StatsOptionFitRemoteNps(item_kinds=StatNeutItemKinds(default=True, bomb=False))])))
    assert api_fit_stats.remote_nps == [approx(23.225806), approx(23.225806), 0]


def test_include_charges(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reactivation_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.module_reactivation_delay)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.use_missiles,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_bomb_effect_id = client.mk_eve_effect(id_=consts.EveEffect.bomb_launching, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_cycle_time_attr_id: 10000, eve_reactivation_time_attr_id: 67500, eve_capacity_attr_id: 300},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 1800, eve_volume_attr_id: 75},
        eff_ids=[eve_bomb_effect_id],
        defeff_id=eve_bomb_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification - need to include charges for module to show neuts, since it's on-charge effect
    # which deals neutralizes cap. For charges, this option doesn't do anything
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_nps=(True, [
        StatsOptionItemRemoteNps(),
        StatsOptionItemRemoteNps(include_charges=False),
        StatsOptionItemRemoteNps(include_charges=True)])))
    assert api_module_stats.remote_nps == [0, 0, approx(23.225806)]
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(remote_nps=(True, [
        StatsOptionItemRemoteNps(),
        StatsOptionItemRemoteNps(include_charges=False),
        StatsOptionItemRemoteNps(include_charges=True)])))
    assert api_charge_stats.remote_nps == [approx(23.225806), approx(23.225806), approx(23.225806)]


def test_zero_cycle_time(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reactivation_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.module_reactivation_delay)
    eve_launcher_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.use_missiles,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_bomb_effect_id = client.mk_eve_effect(id_=consts.EveEffect.bomb_launching, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_cycle_time_attr_id: 0, eve_reactivation_time_attr_id: 67500, eve_capacity_attr_id: 300},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 1800, eve_volume_attr_id: 75},
        eff_ids=[eve_bomb_effect_id],
        defeff_id=eve_bomb_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == 0
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(remote_nps=True))
    assert api_charge_stats.remote_nps.one() == 0


def test_no_cycle_time(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_reactivation_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.module_reactivation_delay)
    eve_launcher_effect_id = client.mk_eve_effect(id_=consts.EveEffect.use_missiles, cat_id=consts.EveEffCat.active)
    eve_bomb_effect_id = client.mk_eve_effect(id_=consts.EveEffect.bomb_launching, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={eve_cycle_time_attr_id: 10000, eve_reactivation_time_attr_id: 67500, eve_capacity_attr_id: 300},
        eff_ids=[eve_launcher_effect_id],
        defeff_id=eve_launcher_effect_id)
    eve_charge_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 1800, eve_volume_attr_id: 75},
        eff_ids=[eve_bomb_effect_id],
        defeff_id=eve_bomb_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(
        type_id=eve_module_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_charge_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == 0
    api_charge_stats = api_module.charge.get_stats(options=ItemStatsOptions(remote_nps=True))
    assert api_charge_stats.remote_nps.one() == 0
