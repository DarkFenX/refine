from fw import approx
from fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatNeutItemKinds,
    StatsOptionFitOutNps,
    StatsOptionItemOutNps,
)


def test_state(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_aoe_neut,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 4000, eve_cycle_time_attr_id: 63000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == approx(63.492063)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == approx(63.492063)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_stats.outgoing_nps.one() == approx(63.492063)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(ignore_state=False),
        StatsOptionItemOutNps(ignore_state=True)])))
    assert api_module_stats.outgoing_nps == [0, approx(63.492063)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == approx(63.492063)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == approx(63.492063)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_stats.outgoing_nps.one() == approx(63.492063)


def test_range(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_aoe_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_aoe_range)
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_aoe_neut,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 4000, eve_cycle_time_attr_id: 63000,
            eve_optimal_attr_id: 300000, eve_aoe_radius_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_src_struct_id = client.mk_eve_struct(attrs={eve_radius_attr_id: 150000})
    eve_tgt_ship_id = client.mk_eve_ship(
        attrs={eve_radius_attr_id: 220, eve_sig_radius_attr_id: 200, eve_cap_attr_id: 5000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_struct_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 310219, 0))
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    api_src_module_proj.change_module(add_projs=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(126.984127)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(126.984127)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.outgoing_nps.one() == approx(63.492063)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.outgoing_nps.one() == approx(63.492063)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 310221, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == 0
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == 0
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.outgoing_nps.one() == 0
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.outgoing_nps.one() == 0


def test_application_and_cap_limit(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_neut_sig_res_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_aoe_sig_radius)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_prop_blow_attr_id = client.mk_eve_attr(id_=consts.EveAttr.entity_max_velocity_sig_radius_mult)
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_aoe_neut,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 4000, eve_cycle_time_attr_id: 63000, eve_neut_sig_res_attr_id: 500},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_tgt_ship1_id = client.mk_eve_ship(attrs={eve_sig_radius_attr_id: 1000, eve_cap_attr_id: 3000})
    eve_tgt_ship2_id = client.mk_eve_ship(attrs={eve_sig_radius_attr_id: 200, eve_cap_attr_id: 3000})
    eve_tgt_drone_id = client.mk_eve_drone(
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
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=(True, [
        StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionFitOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_fleet_stats.outgoing_nps == [approx(95.238095), approx(25.396825)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(outgoing_nps=(True, [
        StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionFitOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_fit_stats.outgoing_nps == [approx(95.238095), approx(25.396825)]
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_proj_stats.outgoing_nps == [approx(47.619048), approx(12.698413)]
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_nonproj_stats.outgoing_nps == [approx(47.619048), approx(12.698413)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    api_tgt_drone.change_drone(prop_mode=consts.ApiNpcPropMode.chase)
    # Verification - application vs ship now is limited by sig and not cap pool, and drone sig is
    # blown
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=(True, [
        StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionFitOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_fleet_stats.outgoing_nps == [approx(50.793651), approx(126.984127)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(outgoing_nps=(True, [
        StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionFitOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_fit_stats.outgoing_nps == [approx(50.793651), approx(126.984127)]
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_proj_stats.outgoing_nps == [approx(25.396825), approx(63.492063)]
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_nonproj_stats.outgoing_nps == [approx(25.396825), approx(63.492063)]
    # Action
    api_tgt_drone.change_drone(prop_mode=consts.ApiNpcPropMode.cruise)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(25.396825)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(25.396825)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_proj_stats.outgoing_nps.one() == approx(12.698413)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_nonproj_stats.outgoing_nps.one() == approx(12.698413)


def test_resist_and_cap_limit(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_resist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_warfare_resist)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_aoe_neut,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 4000, eve_cycle_time_attr_id: 63000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_resist_attr_id: 0.8, eve_cap_attr_id: 2500, eve_sig_radius_attr_id: 200})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_resist_attr_id: 0.4, eve_cap_attr_id: 2500, eve_sig_radius_attr_id: 200})
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
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(79.365079)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(79.365079)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.outgoing_nps.one() == approx(39.68254)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.outgoing_nps.one() == approx(39.68254)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(50.793651)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(50.793651)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.outgoing_nps.one() == approx(25.396825)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.outgoing_nps.one() == approx(25.396825)


def test_item_kind(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_aoe_neut,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 4000, eve_cycle_time_attr_id: 63000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=(True, [
        StatsOptionFitOutNps(),
        StatsOptionFitOutNps(item_kinds=StatNeutItemKinds(default=False, module=True)),
        StatsOptionFitOutNps(item_kinds=StatNeutItemKinds(default=True, module=False))])))
    assert api_fleet_stats.outgoing_nps == [approx(63.492063), approx(63.492063), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=(True, [
        StatsOptionFitOutNps(),
        StatsOptionFitOutNps(item_kinds=StatNeutItemKinds(default=False, module=True)),
        StatsOptionFitOutNps(item_kinds=StatNeutItemKinds(default=True, module=False))])))
    assert api_fit_stats.outgoing_nps == [approx(63.492063), approx(63.492063), 0]


def test_zero_cycle_time(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_aoe_neut,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 4000, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_stats.outgoing_nps.one() == 0


def test_no_cycle_time(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_aoe_neut, cat_id=consts.EveEffCat.target)
    eve_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 4000, eve_cycle_time_attr_id: 63000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_stats.outgoing_nps.one() == 0
