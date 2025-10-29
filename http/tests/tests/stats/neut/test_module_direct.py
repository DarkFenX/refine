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
