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
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
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
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_cps=(True, [
        StatsOptionItemOutCps(ignore_state=False),
        StatsOptionItemOutCps(ignore_state=True)])))
    assert api_module_stats.outgoing_cps == [0, approx(70.2)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    # Verification
    api_fleet_stats = api_fleet.get_stats(options=FleetStatsOptions(outgoing_cps=(True, [
        StatsOptionFitOutCps(time_options=StatTimeBurst()),
        StatsOptionFitOutCps(time_options=StatTimeSim(time=None)),
        StatsOptionFitOutCps(time_options=StatTimeSim(time=4)),
        StatsOptionFitOutCps(time_options=StatTimeSim(time=6)),
        StatsOptionFitOutCps(time_options=StatTimeSim(time=9))])))
    assert api_fleet_stats.outgoing_cps == [approx(70.2), approx(70.2), 0, approx(58.5), approx(39)]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(outgoing_cps=(True, [
        StatsOptionFitOutCps(time_options=StatTimeBurst()),
        StatsOptionFitOutCps(time_options=StatTimeSim(time=None)),
        StatsOptionFitOutCps(time_options=StatTimeSim(time=4)),
        StatsOptionFitOutCps(time_options=StatTimeSim(time=6)),
        StatsOptionFitOutCps(time_options=StatTimeSim(time=9))])))
    assert api_fit_stats.outgoing_cps == [approx(70.2), approx(70.2), 0, approx(58.5), approx(39)]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(outgoing_cps=(True, [
        StatsOptionItemOutCps(time_options=StatTimeBurst()),
        StatsOptionItemOutCps(time_options=StatTimeSim(time=None)),
        StatsOptionItemOutCps(time_options=StatTimeSim(time=4)),
        StatsOptionItemOutCps(time_options=StatTimeSim(time=6)),
        StatsOptionItemOutCps(time_options=StatTimeSim(time=9))])))
    assert api_module_stats.outgoing_cps == [approx(70.2), approx(70.2), 0, approx(58.5), approx(39)]


def test_time(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
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


def test_zero_cycle_time(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 351, eve_cycle_time_attr_id: 0},
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
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target)
    eve_module_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
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


def test_not_requested(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.power_transfer_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_capacitor_transmitter,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 351, eve_cycle_time_attr_id: 5000},
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
