from fw import approx, check_no_field
from fw.api import (
    FitStatsOptions,
    FleetStatsOptions,
    ItemStatsOptions,
    StatsOptionFitOutCps,
    StatsOptionItemOutCps,
    StatTimeBurst,
    StatTimeSim,
)


def test_state(client, consts):
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_cps=True))
    assert api_fleet_stats.outgoing_cps.one() == approx(70.2)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_cps=True))
    assert api_fit_stats.outgoing_cps.one() == approx(70.2)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_cps=True))
    assert api_module_stats.outgoing_cps.one() == approx(70.2)
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_cps=True))
    assert api_fleet_stats.outgoing_cps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_cps=True))
    assert api_fit_stats.outgoing_cps.one() == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_cps=(True, [StatsOptionItemOutCps(ignore_state=False), StatsOptionItemOutCps(ignore_state=True)])))
    assert api_module_stats.outgoing_cps == [0, approx(70.2)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_cps=True))
    assert api_fleet_stats.outgoing_cps.one() == approx(70.2)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_cps=True))
    assert api_fit_stats.outgoing_cps.one() == approx(70.2)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_cps=True))
    assert api_module_stats.outgoing_cps.one() == approx(70.2)


def test_time(client, consts):
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification - burst stats (first cycle)
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(time_options=StatTimeBurst())])))
    assert api_fleet_stats.outgoing_cps.one() == approx(70.2)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(time_options=StatTimeBurst())])))
    assert api_fit_stats.outgoing_cps.one() == approx(70.2)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_cps=(True, [StatsOptionItemOutCps(time_options=StatTimeBurst())])))
    assert api_module_stats.outgoing_cps.one() == approx(70.2)
    # Sim stats without time - loop stats are exposed
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(time_options=StatTimeSim(time=None))])))
    assert api_fleet_stats.outgoing_cps.one() == approx(70.2)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(time_options=StatTimeSim(time=None))])))
    assert api_fit_stats.outgoing_cps.one() == approx(70.2)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_cps=(True, [StatsOptionItemOutCps(time_options=StatTimeSim(time=None))])))
    assert api_module_stats.outgoing_cps.one() == approx(70.2)
    # Sim with time before transfer effect is applied (which happens at the end of cycle)
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(time_options=StatTimeSim(time=4))])))
    assert api_fleet_stats.outgoing_cps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(time_options=StatTimeSim(time=4))])))
    assert api_fit_stats.outgoing_cps.one() == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_cps=(True, [StatsOptionItemOutCps(time_options=StatTimeSim(time=4))])))
    assert api_module_stats.outgoing_cps.one() == 0
    # Sim with time just after 2nd cycle has started
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(time_options=StatTimeSim(time=6))])))
    assert api_fleet_stats.outgoing_cps.one() == approx(58.5)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(time_options=StatTimeSim(time=6))])))
    assert api_fit_stats.outgoing_cps.one() == approx(58.5)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_cps=(True, [StatsOptionItemOutCps(time_options=StatTimeSim(time=6))])))
    assert api_module_stats.outgoing_cps.one() == approx(58.5)
    # Sim with time just before completion of 2nd cycle
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(time_options=StatTimeSim(time=9))])))
    assert api_fleet_stats.outgoing_cps.one() == approx(39)
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(time_options=StatTimeSim(time=9))])))
    assert api_fit_stats.outgoing_cps.one() == approx(39)
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(
        outgoing_cps=(True, [StatsOptionItemOutCps(time_options=StatTimeSim(time=9))])))
    assert api_module_stats.outgoing_cps.one() == approx(39)


def test_range(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 5000, eve_optimal_attr_id: 7500},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 550})
    eve_tgt_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500, eve_radius_attr_id: 120})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id, coordinates=(0, 0, 0))
    api_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id, coordinates=(0, 8169, 0))
    api_module_proj.change_module(add_projs=[api_tgt_ship.id])
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_cps.one() == approx(140.4)
    api_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fit_stats.outgoing_cps.one() == approx(140.4)
    api_module_proj_stats = api_module_proj.get_stats(options=ItemStatsOptions(
        outgoing_cps=(True, [StatsOptionItemOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.outgoing_cps.one() == approx(70.2)
    api_module_nonproj_stats = api_module_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_cps=(True, [StatsOptionItemOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.outgoing_cps.one() == approx(70.2)
    # Action
    api_tgt_ship.change_ship(coordinates=(0, 8171, 0))
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_cps.one() == 0
    api_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fit_stats.outgoing_cps.one() == 0
    api_module_proj_stats = api_module_proj.get_stats(options=ItemStatsOptions(
        outgoing_cps=(True, [StatsOptionItemOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.outgoing_cps.one() == 0
    api_module_nonproj_stats = api_module_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_cps=(True, [StatsOptionItemOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.outgoing_cps.one() == 0


def test_resist_and_limit(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_optimal_attr_id = client.mk_eve_attr()
    eve_resist_attr_id = client.mk_eve_attr()
    eve_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.radius)
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id,
        range_attr_id=eve_optimal_attr_id,
        resist_attr_id=eve_resist_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 5000, eve_optimal_attr_id: 7500},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_src_ship_id = client.mk_eve_ship(attrs={eve_radius_attr_id: 550})
    eve_tgt_ship1_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 300, eve_resist_attr_id: 1, eve_radius_attr_id: 120})
    eve_tgt_ship2_id = client.mk_eve_ship(
        attrs={eve_ship_amount_attr_id: 300, eve_resist_attr_id: 0.5, eve_radius_attr_id: 120})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_fit.set_ship(type_id=eve_src_ship_id)
    api_module_proj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_module_nonproj = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship1_id)
    api_module_proj.change_module(add_projs=[api_tgt_ship.id])
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_cps.one() == approx(120)
    api_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fit_stats.outgoing_cps.one() == approx(120)
    api_module_proj_stats = api_module_proj.get_stats(options=ItemStatsOptions(
        outgoing_cps=(True, [StatsOptionItemOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.outgoing_cps.one() == approx(60)
    api_module_nonproj_stats = api_module_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_cps=(True, [StatsOptionItemOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.outgoing_cps.one() == approx(60)
    # Action
    api_tgt_ship.change_ship(type_id=eve_tgt_ship2_id)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_cps.one() == approx(70.2)
    api_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(
        outgoing_cps=(True, [StatsOptionFitOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fit_stats.outgoing_cps.one() == approx(70.2)
    api_module_proj_stats = api_module_proj.get_stats(options=ItemStatsOptions(
        outgoing_cps=(True, [StatsOptionItemOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_proj_stats.outgoing_cps.one() == approx(35.1)
    api_module_nonproj_stats = api_module_nonproj.get_stats(options=ItemStatsOptions(
        outgoing_cps=(True, [StatsOptionItemOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_module_nonproj_stats.outgoing_cps.one() == approx(35.1)


def test_zero_cycle_time(client, consts):
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_cps=True))
    assert api_fleet_stats.outgoing_cps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_cps=True))
    assert api_fit_stats.outgoing_cps.one() == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_cps=True))
    assert api_module_stats.outgoing_cps.one() == 0


def test_no_cycle_time(client, consts):
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target)
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_cps=True))
    assert api_fleet_stats.outgoing_cps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_cps=True))
    assert api_fit_stats.outgoing_cps.one() == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_cps=True))
    assert api_module_stats.outgoing_cps.one() == 0


def test_item_not_loaded(client, consts):
    eve_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_cps=True))
    assert api_fleet_stats.outgoing_cps.one() == 0
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_cps=True))
    assert api_fit_stats.outgoing_cps.one() == 0
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_cps=True))
    assert api_module_stats.outgoing_cps is None


def test_incorrect_item_kind(client, consts):
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_implant_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    # Verification - attempt to get stats of item of incorrect kind fails whole stat batch
    api_implant_stats = api_implant.get_stats(options=ItemStatsOptions(outgoing_cps=True))
    assert api_implant_stats.outgoing_cps is None


def test_incorrect_projectee(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_implant_id = client.mk_eve_item()
    eve_tgt_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 500})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_implant = api_src_fit.add_implant(type_id=eve_implant_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_tmp = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_tgt_tmp.remove()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_src_fit.id])
    # Verification - specifying incorrect projectee item IDs should fail only that specific option,
    # not whole stat batch
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_cps=(True, [
        StatsOptionFitOutCps(projectee_item_id=api_tgt_tmp.id),
        StatsOptionFitOutCps(projectee_item_id=api_implant.id),
        StatsOptionFitOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_fleet_stats.outgoing_cps == [None, None, approx(70.2)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(outgoing_cps=(True, [
        StatsOptionFitOutCps(projectee_item_id=api_tgt_tmp.id),
        StatsOptionFitOutCps(projectee_item_id=api_implant.id),
        StatsOptionFitOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_fit_stats.outgoing_cps == [None, None, approx(70.2)]
    api_src_module_stats = api_src_module.get_stats(options=ItemStatsOptions(outgoing_cps=(True, [
        StatsOptionItemOutCps(projectee_item_id=api_tgt_tmp.id),
        StatsOptionItemOutCps(projectee_item_id=api_implant.id),
        StatsOptionItemOutCps(projectee_item_id=api_tgt_ship.id)])))
    assert api_src_module_stats.outgoing_cps == [None, None, approx(70.2)]


def test_not_requested(client, consts):
    eve_transfer_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_transfer_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fleet = api_sol.create_fleet()
    api_fleet.change(add_fits=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_cps=False))
    with check_no_field():
        api_fleet_stats.outgoing_cps  # noqa: B018
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_cps=False))
    with check_no_field():
        api_fit_stats.outgoing_cps  # noqa: B018
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_cps=False))
    with check_no_field():
        api_module_stats.outgoing_cps  # noqa: B018
