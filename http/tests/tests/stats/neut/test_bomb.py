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
        StatsOptionFitRemoteNps(item_kinds=StatRemoteNpsItemKinds(default=False, bomb=True)),
        StatsOptionFitRemoteNps(item_kinds=StatRemoteNpsItemKinds(default=True, bomb=False))])))
    assert api_fleet_stats.remote_nps == [approx(23.225806), approx(23.225806), 0]
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(remote_nps=(True, [
        StatsOptionFitRemoteNps(),
        StatsOptionFitRemoteNps(item_kinds=StatRemoteNpsItemKinds(default=False, bomb=True)),
        StatsOptionFitRemoteNps(item_kinds=StatRemoteNpsItemKinds(default=True, bomb=False))])))
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
