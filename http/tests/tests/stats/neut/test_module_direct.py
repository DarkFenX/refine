from tests import approx
from tests.fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatRemoteNpsItemKinds,
    StatsOptionFitRemoteNps,
    StatsOptionItemRemoteNps,
)


def test_state(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == approx(25)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == approx(25)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_nps=True))
    assert api_module_stats.remote_nps.one() == approx(25)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_nps=(True, [
        StatsOptionItemRemoteNps(ignore_state=False),
        StatsOptionItemRemoteNps(ignore_state=True)])))
    assert api_module_stats.remote_nps.map(lambda i: i) == [0, approx(25)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == approx(25)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == approx(25)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_nps=True))
    assert api_module_stats.remote_nps.one() == approx(25)


def test_cap_limit_and_range(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_falloff_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id,
        falloff_attr_id=eve_falloff_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 24000,
            eve_optimal_attr_id: 20000, eve_falloff_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 550})
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_radius_attr_id: 220, eve_sig_radius_attr_id: 200, eve_cap_attr_id: 1000})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_radius_attr_id: 220, eve_sig_radius_attr_id: 200, eve_cap_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id, coordinates=(0, 20770, 0))
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification - target has high enough cap pool, so full strength is exposed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(50)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(50)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.remote_nps.one() == approx(25)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.remote_nps.one() == approx(25)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification - cap pool is lower and neut amount, so strength is reduced
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(41.666667)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(41.666667)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.remote_nps.one() == approx(20.833333)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.remote_nps.one() == approx(20.833333)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 30770, 0))
    # Verification - target now has high enough cap pool, considering amount is reduced by range
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(25)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(25)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.remote_nps.one() == approx(12.5)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.remote_nps.one() == approx(12.5)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 40770, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(3.125)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(3.125)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.remote_nps.one() == approx(1.5625)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.remote_nps.one() == approx(1.5625)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 50769, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(0.09769687)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(0.09769687)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.remote_nps.one() == approx(0.04884844)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.remote_nps.one() == approx(0.04884844)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 50771, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == 0
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == 0
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.remote_nps.one() == 0
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.remote_nps.one() == 0


def test_cap_limit_and_application(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_neut_sig_res_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_sig_res)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_prop_blow_attr_id = client.mk_eve_attr(id_=consts.EveAttr.entity_max_velocity_sig_radius_mult)
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 4400, eve_cycle_time_attr_id: 48000, eve_neut_sig_res_attr_id: 8000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_tgt_ship1_id = client.mk_eve_ship(attrs={eve_sig_radius_attr_id: 10000, eve_cap_attr_id: 2000})
    eve_tgt_ship2_id = client.mk_eve_ship(attrs={eve_sig_radius_attr_id: 200, eve_cap_attr_id: 2000})
    eve_tgt_drone_id = client.mk_eve_item(
        attrs={eve_sig_radius_attr_id: 100, eve_prop_blow_attr_id: 6, eve_cap_attr_id: 10000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id)
    api_tgt_drone = api_tgt_fit.add_drone(type_id=eve_tgt_drone_id, prop_mode=consts.ApiNpcPropMode.cruise)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id, api_tgt_drone.id])
    # Verification - application against ship is limited by cap pool
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=(True, [
        StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionFitRemoteNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_fleet_stats.remote_nps == [approx(83.333333), approx(2.291667)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(remote_nps=(True, [
        StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionFitRemoteNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_fit_stats.remote_nps == [approx(83.333333), approx(2.291667)]
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(remote_nps=(True, [
        StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemRemoteNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_proj_stats.remote_nps == [approx(41.666667), approx(1.145833)]
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(remote_nps=(True, [
        StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemRemoteNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_nonproj_stats.remote_nps == [approx(41.666667), approx(1.145833)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    api_tgt_drone.change_drone(prop_mode=consts.ApiNpcPropMode.chase)
    # Verification - application vs ship now is limited by sig and not cap pool, and drone sig is
    # blown
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=(True, [
        StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionFitRemoteNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_fleet_stats.remote_nps == [approx(4.583333), approx(13.75)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(remote_nps=(True, [
        StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionFitRemoteNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_fit_stats.remote_nps == [approx(4.583333), approx(13.75)]
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(remote_nps=(True, [
        StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemRemoteNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_proj_stats.remote_nps == [approx(2.291667), approx(6.875)]
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(remote_nps=(True, [
        StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemRemoteNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_nonproj_stats.remote_nps == [approx(2.291667), approx(6.875)]
    # Action
    api_tgt_drone.change_drone(prop_mode=consts.ApiNpcPropMode.cruise)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(2.291667)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(2.291667)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_proj_stats.remote_nps.one() == approx(1.145833)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_nonproj_stats.remote_nps.one() == approx(1.145833)


def test_cap_limit_and_resist(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_resist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_warfare_resist)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_resist_attr_id: 0.8, eve_cap_attr_id: 400, eve_sig_radius_attr_id: 200})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_resist_attr_id: 0.4, eve_cap_attr_id: 400, eve_sig_radius_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(33.333333)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(33.333333)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.remote_nps.one() == approx(16.666667)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.remote_nps.one() == approx(16.666667)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(20)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(20)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.remote_nps.one() == approx(10)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.remote_nps.one() == approx(10)


def test_item_kind(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=(True, [
        StatsOptionFitRemoteNps(),
        StatsOptionFitRemoteNps(item_kinds=StatRemoteNpsItemKinds(default=False, module=True)),
        StatsOptionFitRemoteNps(item_kinds=StatRemoteNpsItemKinds(default=True, module=False))])))
    assert api_fleet_stats.remote_nps == [approx(25), approx(25), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=(True, [
        StatsOptionFitRemoteNps(),
        StatsOptionFitRemoteNps(item_kinds=StatRemoteNpsItemKinds(default=False, module=True)),
        StatsOptionFitRemoteNps(item_kinds=StatRemoteNpsItemKinds(default=True, module=False))])))
    assert api_fit_stats.remote_nps == [approx(25), approx(25), 0]


def test_zero_cycle_time(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_nps=True))
    assert api_module_stats.remote_nps.one() == 0


def test_no_cycle_time(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.energy_neut_falloff, cat_id=consts.EveEffCat.target)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_nps=True))
    assert api_module_stats.remote_nps.one() == 0


def test_item_not_loaded(client, consts):
    eve_module_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(remote_nps=True))
    assert api_module_stats.remote_nps is None
