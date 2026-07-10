from fw import approx
from fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatNeutItemKinds,
    StatsOptionFitOutNps,
    StatsOptionItemOutNps,
    StatTimeBurst,
    StatTimeSim,
)


def test_state(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_amount)
    eve_neut_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_radius)
    eve_neut_sig_res_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_boson_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_cone_dot,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_effect_lance_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_beam_dot,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_effect_lance_debuff_id = client.mk_eve_effect(
        id_=consts.EveEffect.debuff_lance,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_effect_reaper_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_slash,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_boson_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 30000, eve_neut_range_attr_id: 10000,
            eve_neut_sig_res_attr_id: 20000, eve_cycle_time_attr_id: 240000},
        eff_ids=[eve_effect_boson_id],
        defeff_id=eve_effect_boson_id)
    eve_module_lance_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 32500, eve_neut_range_attr_id: 10000,
            eve_neut_sig_res_attr_id: 20000, eve_cycle_time_attr_id: 240000},
        eff_ids=[eve_effect_lance_id],
        defeff_id=eve_effect_lance_id)
    eve_module_lance_debuff_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 33750, eve_neut_range_attr_id: 10000,
            eve_neut_sig_res_attr_id: 20000, eve_cycle_time_attr_id: 300000},
        eff_ids=[eve_effect_lance_debuff_id],
        defeff_id=eve_effect_lance_debuff_id)
    eve_module_reaper_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 32500, eve_neut_range_attr_id: 10000,
            eve_neut_sig_res_attr_id: 20000, eve_cycle_time_attr_id: 240000},
        eff_ids=[eve_effect_reaper_id],
        defeff_id=eve_effect_reaper_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module_boson = api_fit.add_module(type_id=eve_module_boson_id, state=consts.ApiModuleState.active)
    api_module_lance = api_fit.add_module(type_id=eve_module_lance_id, state=consts.ApiModuleState.active)
    api_module_lance_debuff = api_fit.add_module(type_id=eve_module_lance_debuff_id, state=consts.ApiModuleState.active)
    api_module_reaper = api_fit.add_module(type_id=eve_module_reaper_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == approx(508.333333)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == approx(508.333333)
    api_module_boson_stats = api_module_boson.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_boson_stats.outgoing_nps.one() == approx(125)
    api_module_lance_stats = api_module_lance.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_lance_stats.outgoing_nps.one() == approx(135.416667)
    api_module_lance_debuff_stats = api_module_lance_debuff.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_lance_debuff_stats.outgoing_nps.one() == approx(112.5)
    api_module_reaper_stats = api_module_reaper.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_reaper_stats.outgoing_nps.one() == approx(135.416667)
    # Action
    api_module_boson.change_module(state=consts.ApiModuleState.online)
    api_module_lance.change_module(state=consts.ApiModuleState.online)
    api_module_lance_debuff.change_module(state=consts.ApiModuleState.online)
    api_module_reaper.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == 0
    api_module_boson_stats = api_module_boson.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(ignore_state=False),
        StatsOptionItemOutNps(ignore_state=True)])))
    assert api_module_boson_stats.outgoing_nps == [0, approx(125)]
    api_module_lance_stats = api_module_lance.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(ignore_state=False),
        StatsOptionItemOutNps(ignore_state=True)])))
    assert api_module_lance_stats.outgoing_nps == [0, approx(135.416667)]
    api_module_lance_debuff_stats = api_module_lance_debuff.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(ignore_state=False),
        StatsOptionItemOutNps(ignore_state=True)])))
    assert api_module_lance_debuff_stats.outgoing_nps == [0, approx(112.5)]
    api_module_reaper_stats = api_module_reaper.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(ignore_state=False),
        StatsOptionItemOutNps(ignore_state=True)])))
    assert api_module_reaper_stats.outgoing_nps == [0, approx(135.416667)]
    # Action
    api_module_boson.change_module(state=consts.ApiModuleState.active)
    api_module_lance.change_module(state=consts.ApiModuleState.active)
    api_module_lance_debuff.change_module(state=consts.ApiModuleState.active)
    api_module_reaper.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == approx(508.333333)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == approx(508.333333)
    api_module_boson_stats = api_module_boson.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_boson_stats.outgoing_nps.one() == approx(125)
    api_module_lance_stats = api_module_lance.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_lance_stats.outgoing_nps.one() == approx(135.416667)
    api_module_lance_debuff_stats = api_module_lance_debuff.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_lance_debuff_stats.outgoing_nps.one() == approx(112.5)
    api_module_reaper_stats = api_module_reaper.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_reaper_stats.outgoing_nps.one() == approx(135.416667)


def test_time(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_amount)
    eve_neut_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_radius)
    eve_neut_sig_res_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_sig_radius)
    eve_delay_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_warning_duration)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_cone_dot,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 30000, eve_neut_range_attr_id: 10000, eve_neut_sig_res_attr_id: 20000,
            eve_cycle_time_attr_id: 240000, eve_delay_attr_id: 10000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification - burst stats (first cycle)
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeBurst())])))
    assert api_fleet_stats.outgoing_nps.one() == approx(125)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeBurst())])))
    assert api_fit_stats.outgoing_nps.one() == approx(125)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(time_options=StatTimeBurst())])))
    assert api_module_stats.outgoing_nps.one() == approx(125)
    # Sim stats without time - loop stats are exposed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=None))])))
    assert api_fleet_stats.outgoing_nps.one() == approx(125)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=None))])))
    assert api_fit_stats.outgoing_nps.one() == approx(125)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(time_options=StatTimeSim(time=None))])))
    assert api_module_stats.outgoing_nps.one() == approx(125)
    # Sim with time 1 second after first cycle has started. Despite there being delay attribute set,
    # its value is not used for neuting - neuting is applied immediately
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=1))])))
    assert api_fleet_stats.outgoing_nps.one() == approx(30000)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=1))])))
    assert api_fit_stats.outgoing_nps.one() == approx(30000)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(time_options=StatTimeSim(time=1))])))
    assert api_module_stats.outgoing_nps.one() == approx(30000)
    # Sim with time before end of first cycle
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=239))])))
    assert api_fleet_stats.outgoing_nps.one() == approx(125.523013)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=239))])))
    assert api_fit_stats.outgoing_nps.one() == approx(125.523013)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(time_options=StatTimeSim(time=239))])))
    assert api_module_stats.outgoing_nps.one() == approx(125.523013)
    # Sim with time after start of second cycle
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=241))])))
    assert api_fleet_stats.outgoing_nps.one() == approx(248.962656)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(time_options=StatTimeSim(time=241))])))
    assert api_fit_stats.outgoing_nps.one() == approx(248.962656)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(time_options=StatTimeSim(time=241))])))
    assert api_module_stats.outgoing_nps.one() == approx(248.962656)


def test_range(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_amount)
    eve_neut_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_radius)
    eve_neut_sig_res_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_cone_dot,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 30000, eve_neut_range_attr_id: 10000,
            eve_neut_sig_res_attr_id: 20000, eve_cycle_time_attr_id: 240000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 6800})
    eve_tgt_ship_id = client.mk_eve_ship(
        attrs={eve_radius_attr_id: 500, eve_sig_radius_attr_id: 100000, eve_cap_attr_id: 50000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 6299, 0))
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_src_module_proj.change_module(add_proj_item_ids=[api_tgt_ship.id])
    # Verification - check that ships which hide inside attacking ship radius are still impacted
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(250)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(250)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.outgoing_nps.one() == approx(125)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.outgoing_nps.one() == approx(125)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 17299, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(250)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(250)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.outgoing_nps.one() == approx(125)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.outgoing_nps.one() == approx(125)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 17301, 0))
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
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_amount)
    eve_neut_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_radius)
    eve_neut_sig_res_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cruise_speed_attr_id = client.mk_eve_attr(id_=consts.EveAttr.entity_cruise_speed)
    eve_prop_blow_attr_id = client.mk_eve_attr(id_=consts.EveAttr.entity_max_velocity_sig_radius_mult)
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_cone_dot,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 30000, eve_neut_range_attr_id: 10000,
            eve_neut_sig_res_attr_id: 20000, eve_cycle_time_attr_id: 240000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_tgt_ship1_id = client.mk_eve_ship(attrs={eve_sig_radius_attr_id: 100000, eve_cap_attr_id: 12000})
    eve_tgt_ship2_id = client.mk_eve_ship(attrs={eve_sig_radius_attr_id: 5000, eve_cap_attr_id: 12000})
    eve_tgt_drone_id = client.mk_eve_drone(attrs={
        eve_sig_radius_attr_id: 100, eve_prop_blow_attr_id: 6, eve_cap_attr_id: 10000, eve_cruise_speed_attr_id: 200})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id)
    api_tgt_drone = api_tgt_fit.add_drone(type_id=eve_tgt_drone_id, npc_prop=consts.ApiNpcProp.cruise)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_src_module_proj.change_module(add_proj_item_ids=[api_tgt_ship.id, api_tgt_drone.id])
    # Verification - application against ship is limited by cap pool
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=(True, [
        StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionFitOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_fleet_stats.outgoing_nps == [approx(100), approx(1.25)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(outgoing_nps=(True, [
        StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionFitOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_fit_stats.outgoing_nps == [approx(100), approx(1.25)]
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_proj_stats.outgoing_nps == [approx(50), approx(0.625)]
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_nonproj_stats.outgoing_nps == [approx(50), approx(0.625)]
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    api_tgt_drone.change_drone(npc_prop=consts.ApiNpcProp.chase)
    # Verification - application vs ship now is limited by sig and not cap pool, and drone sig is
    # blown
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=(True, [
        StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionFitOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_fleet_stats.outgoing_nps == [approx(62.5), approx(7.5)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(outgoing_nps=(True, [
        StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionFitOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_fit_stats.outgoing_nps == [approx(62.5), approx(7.5)]
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_proj_stats.outgoing_nps == [approx(31.25), approx(3.75)]
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(outgoing_nps=(True, [
        StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id),
        StatsOptionItemOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_nonproj_stats.outgoing_nps == [approx(31.25), approx(3.75)]
    # Action
    api_tgt_drone.change_drone(npc_prop=consts.ApiNpcProp.cruise)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(1.25)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(1.25)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_proj_stats.outgoing_nps.one() == approx(0.625)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_drone.id)])))
    assert api_src_module_nonproj_stats.outgoing_nps.one() == approx(0.625)


def test_resist_and_cap_limit(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_amount)
    eve_neut_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_radius)
    eve_neut_sig_res_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_sig_radius)
    eve_neut_resist_ref_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_resist_id)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cap_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_resist_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_warfare_resist)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_cone_dot,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 30000, eve_neut_range_attr_id: 10000, eve_neut_sig_res_attr_id: 20000,
            eve_neut_resist_ref_attr_id: eve_resist_attr_id, eve_cycle_time_attr_id: 240000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_resist_attr_id: 0.8, eve_cap_attr_id: 18000, eve_sig_radius_attr_id: 100000})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_resist_attr_id: 0.4, eve_cap_attr_id: 18000, eve_sig_radius_attr_id: 100000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_src_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    api_src_module_proj.change_module(add_proj_item_ids=[api_tgt_ship.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(150)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(150)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.outgoing_nps.one() == approx(75)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.outgoing_nps.one() == approx(75)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_nps.one() == approx(100)
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_nps=(True, [StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_nps.one() == approx(100)
    api_src_module_proj_stats = api_src_module_proj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_proj_stats.outgoing_nps.one() == approx(50)
    api_src_module_nonproj_stats = api_src_module_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_nps=(True, [StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_nonproj_stats.outgoing_nps.one() == approx(50)


def test_item_kind(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_amount)
    eve_neut_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_radius)
    eve_neut_sig_res_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_cone_dot,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 30000, eve_neut_range_attr_id: 10000,
            eve_neut_sig_res_attr_id: 20000, eve_cycle_time_attr_id: 240000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=(True, [
        StatsOptionFitOutNps(),
        StatsOptionFitOutNps(item_kinds=StatNeutItemKinds(default=False, side_effect=True)),
        StatsOptionFitOutNps(item_kinds=StatNeutItemKinds(default=True, side_effect=False))])))
    assert api_fleet_stats.outgoing_nps == [approx(125), approx(125), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=(True, [
        StatsOptionFitOutNps(),
        StatsOptionFitOutNps(item_kinds=StatNeutItemKinds(default=False, side_effect=True)),
        StatsOptionFitOutNps(item_kinds=StatNeutItemKinds(default=True, side_effect=False))])))
    assert api_fit_stats.outgoing_nps == [approx(125), approx(125), 0]


def test_zero_cycle_time(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_amount)
    eve_neut_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_radius)
    eve_neut_sig_res_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.doomsday_cone_dot,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 30000, eve_neut_range_attr_id: 10000,
            eve_neut_sig_res_attr_id: 20000, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_stats.outgoing_nps.one() == 0


def test_no_cycle_time(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_amount)
    eve_neut_range_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_radius)
    eve_neut_sig_res_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_energy_neut_sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(id_=consts.EveEffect.doomsday_cone_dot, cat_id=consts.EveEffCat.active)
    eve_module_id = client.mk_eve_item(
        attrs={
            eve_neut_amount_attr_id: 30000, eve_neut_range_attr_id: 10000,
            eve_neut_sig_res_attr_id: 20000, eve_cycle_time_attr_id: 240000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=True))
    assert api_fleet_stats.outgoing_nps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=True))
    assert api_fit_stats.outgoing_nps.one() == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_module_stats.outgoing_nps.one() == 0
