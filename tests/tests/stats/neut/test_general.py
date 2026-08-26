from fw import approx, check_no_field
from fw.api import FitStatsOptions, FleetStatsOptions, ItemStatsOptions, StatsOptionFitOutNps, StatsOptionItemOutNps


def test_item_not_loaded(client, consts):
    eve_module_id = client.alloc_item_id()
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
    assert api_module_stats.outgoing_nps is None


def test_incorrect_item_kind(client, consts):
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_implant_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_implant = api_fit.add_implant(type_id=eve_implant_id)
    # Verification
    api_implant_stats = api_implant.get_stats(options=ItemStatsOptions(outgoing_nps=True))
    assert api_implant_stats.outgoing_nps is None


def test_incorrect_projectee(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_src_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_implant_id = client.mk_eve_item()
    eve_tgt_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 1000, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(type_id=eve_src_module_id, state=consts.ApiModuleState.active)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_tmp = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_tgt_tmp.remove()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_tgt_ship_id)
    api_implant = api_tgt_fit.add_implant(type_id=eve_implant_id)
    api_fleet = api_sol.create_fleet(fit_ids=[api_src_fit.id])
    # Verification - specifying incorrect projectee item IDs should fail only that specific option,
    # not whole stat batch
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=[
        StatsOptionFitOutNps(projectee_item_id=api_tgt_tmp.id),
        StatsOptionFitOutNps(projectee_item_id=api_implant.id),
        StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)]))
    assert api_fleet_stats.outgoing_nps == [None, None, approx(25)]
    api_src_fit_stats = api_src_fit.get_stats(options=FitStatsOptions(outgoing_nps=[
        StatsOptionFitOutNps(projectee_item_id=api_tgt_tmp.id),
        StatsOptionFitOutNps(projectee_item_id=api_implant.id),
        StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)]))
    assert api_src_fit_stats.outgoing_nps == [None, None, approx(25)]
    api_src_module_stats = api_src_module.get_stats(options=ItemStatsOptions(outgoing_nps=[
        StatsOptionItemOutNps(projectee_item_id=api_tgt_tmp.id),
        StatsOptionItemOutNps(projectee_item_id=api_implant.id),
        StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)]))
    assert api_src_module_stats.outgoing_nps == [None, None, approx(25)]


def test_incorrect_projectee_backref_sol(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_src_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_tgt_ship_id = client.mk_eve_ship(attrs={eve_ship_amount_attr_id: 1000, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    with api_sol.batch() as api_sol_batch:
        api_src_fit = api_sol_batch.create_fit()
        api_src_module = api_sol_batch.add_module(
            fit_id=api_src_fit.id, type_id=eve_src_module_id, state=consts.ApiModuleState.active)
        api_tgt_fit = api_sol_batch.create_fit()
        api_tgt_ship = api_sol_batch.set_ship(fit_id=api_tgt_fit.id, type_id=eve_tgt_ship_id)
        api_fleet = api_sol_batch.create_fleet(fit_ids=[api_src_fit.id])
        api_fleet_stats = api_sol_batch.get_fleet_stats(
            fleet_id=api_fleet.id,
            fleet_options=FleetStatsOptions(outgoing_nps=[
                StatsOptionFitOutNps(projectee_item_id='#0'),
                StatsOptionFitOutNps(projectee_item_id='#10'),
                StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)]))
        api_src_fit_stats = api_sol_batch.get_fit_stats(
            fit_id=api_src_fit.id,
            fit_options=FitStatsOptions(outgoing_nps=[
                StatsOptionFitOutNps(projectee_item_id='#0'),
                StatsOptionFitOutNps(projectee_item_id='#10'),
                StatsOptionFitOutNps(projectee_item_id=api_tgt_ship.id)]))
        api_src_module_stats = api_sol_batch.get_item_stats(
            item_id=api_src_module.id,
            item_options=ItemStatsOptions(outgoing_nps=[
                StatsOptionItemOutNps(projectee_item_id='#0'),
                StatsOptionItemOutNps(projectee_item_id='#10'),
                StatsOptionItemOutNps(projectee_item_id=api_tgt_ship.id)]))
    # Verification - specifying incorrect projectee item backref should fail only that specific
    # option, not whole stat batch or even request
    assert api_fleet_stats.fleet.outgoing_nps == [None, None, approx(25)]
    assert api_src_fit_stats.fit.outgoing_nps == [None, None, approx(25)]
    assert api_src_module_stats.item.outgoing_nps == [None, None, approx(25)]


def test_incorrect_projectee_backref_fit(client, consts):
    eve_ship_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacitor_capacity)
    eve_neut_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.energy_neut_amount)
    eve_sig_radius_attr_id = client.mk_eve_attr(id_=consts.EveAttr.sig_radius)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.energy_neut_falloff,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_src_module_id = client.mk_eve_item(
        attrs={eve_neut_amount_attr_id: 600, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_tgt_drone_id = client.mk_eve_drone(attrs={eve_ship_amount_attr_id: 1000, eve_sig_radius_attr_id: 1})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    with api_fit.batch() as api_fit_batch:
        api_src_module = api_fit_batch.add_module(type_id=eve_src_module_id, state=consts.ApiModuleState.active)
        api_tgt_drone = api_fit_batch.add_drone(type_id=eve_tgt_drone_id)
        api_fit_stats = api_fit_batch.get_fit_stats(
            fit_options=FitStatsOptions(outgoing_nps=[
                StatsOptionFitOutNps(projectee_item_id='#0'),
                StatsOptionFitOutNps(projectee_item_id='#10'),
                StatsOptionFitOutNps(projectee_item_id=api_tgt_drone.id)]))
        api_src_module_stats = api_fit_batch.get_item_stats(
            item_id=api_src_module.id,
            item_options=ItemStatsOptions(outgoing_nps=[
                StatsOptionItemOutNps(projectee_item_id='#0'),
                StatsOptionItemOutNps(projectee_item_id='#10'),
                StatsOptionItemOutNps(projectee_item_id=api_tgt_drone.id)]))
    # Verification - specifying incorrect projectee item backref should fail only that specific
    # option, not whole stat batch or even request
    assert api_fit_stats.fit.outgoing_nps == [None, None, approx(25)]
    assert api_src_module_stats.item.outgoing_nps == [None, None, approx(25)]


def test_not_requested(client, consts):
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
    api_fleet = api_sol.create_fleet(fit_ids=[api_fit.id])
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_nps=False))
    with check_no_field():
        api_fleet_stats.outgoing_nps  # ruff:ignore[useless-expression]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_nps=False))
    with check_no_field():
        api_fit_stats.outgoing_nps  # ruff:ignore[useless-expression]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_nps=False))
    with check_no_field():
        api_module_stats.outgoing_nps  # ruff:ignore[useless-expression]
