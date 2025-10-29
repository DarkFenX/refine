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
        id_=consts.EveEffect.entity_energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 10, eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == approx(1.666667)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == approx(1.666667)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_nps=True))
    assert api_drone_stats.remote_nps.one() == approx(1.666667)
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_nps=(True, [
        StatsOptionItemRemoteNps(ignore_state=False),
        StatsOptionItemRemoteNps(ignore_state=True)])))
    assert api_drone_stats.remote_nps == [0, approx(1.666667)]
    # Action
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == approx(1.666667)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == approx(1.666667)
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_nps=True))
    assert api_drone_stats.remote_nps.one() == approx(1.666667)


def test_range(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.entity_energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id)
    eve_drone_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 10, eve_cycle_time_attr_id: 6000,
            eve_optimal_attr_id: 7500, eve_radius_attr_id: 15},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_tgt_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 220, eve_cap_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(
        type_id=eve_drone_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0))
    api_src_drone_nonproj = api_src_fit.add_drone(
        type_id=eve_drone_id, state=consts.ApiMinionState.engaging, coordinates=(0, 0, 0))
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 7734, 0))
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(3.333333)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(3.333333)
    api_src_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_drone_proj_stats.remote_nps.one() == approx(1.666667)
    api_src_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_drone_nonproj_stats.remote_nps.one() == approx(1.666667)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 7736, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == 0
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == 0
    api_src_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_drone_proj_stats.remote_nps.one() == 0
    api_src_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_drone_nonproj_stats.remote_nps.one() == 0


def test_resist_and_cap_limit(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_resist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_warfare_resist)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.entity_energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_drone_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 10, eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_tgt_ship1_id = client.mk_eve_ship(attrs={eve_resist_attr_id: 0.8, eve_cap_attr_id: 6})
    eve_tgt_ship2_id = client.mk_eve_ship(attrs={eve_resist_attr_id: 0.4, eve_cap_attr_id: 6})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_drone_proj = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_src_drone_nonproj = api_src_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_src_drone_proj.change_drone(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(2)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(2)
    api_src_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_drone_proj_stats.remote_nps.one() == approx(1)
    api_src_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_drone_nonproj_stats.remote_nps.one() == approx(1)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.remote_nps.one() == approx(1.333333)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        remote_nps=(True, [StatsOptionFitRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.remote_nps.one() == approx(1.333333)
    api_src_drone_proj_stats = api_src_drone_proj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_drone_proj_stats.remote_nps.one() == approx(0.6666667)
    api_src_drone_nonproj_stats = api_src_drone_nonproj.get_stats(options=ItemStatsOptions(
        remote_nps=(True, [StatsOptionItemRemoteNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_drone_nonproj_stats.remote_nps.one() == approx(0.6666667)


def test_item_kind(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.entity_energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 10, eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=(True, [
        StatsOptionFitRemoteNps(),
        StatsOptionFitRemoteNps(item_kinds=StatRemoteNpsItemKinds(default=False, minion=True)),
        StatsOptionFitRemoteNps(item_kinds=StatRemoteNpsItemKinds(default=True, minion=False))])))
    assert api_fleet_stats.remote_nps == [approx(1.666667), approx(1.666667), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=(True, [
        StatsOptionFitRemoteNps(),
        StatsOptionFitRemoteNps(item_kinds=StatRemoteNpsItemKinds(default=False, minion=True)),
        StatsOptionFitRemoteNps(item_kinds=StatRemoteNpsItemKinds(default=True, minion=False))])))
    assert api_fit_stats.remote_nps == [approx(1.666667), approx(1.666667), 0]


def test_zero_cycle_time(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.entity_energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 10, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_nps=True))
    assert api_drone_stats.remote_nps.one() == 0


def test_no_cycle_time(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.entity_energy_neut_falloff, cat_id=consts.EveEffCat.target)
    eve_drone_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 10, eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(remote_nps=True))
    assert api_fleet_stats.remote_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=True))
    assert api_fit_stats.remote_nps.one() == 0
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(remote_nps=True))
    assert api_drone_stats.remote_nps.one() == 0
