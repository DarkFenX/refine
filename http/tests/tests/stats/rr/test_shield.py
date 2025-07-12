from tests import approx
from tests.fw.api import FitStatsOptions, ItemStatsOptions, StatsOptionItemRr


def test_state(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_normal_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_shield_booster,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_ancil_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_ancillary_remote_shield_booster,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.npc_entity_remote_shield_booster,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_normal_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 508, eve_cycle_time_attr_id: 8000},
        eff_ids=[eve_module_normal_effect_id],
        defeff_id=eve_module_normal_effect_id)
    eve_module_ancil_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 950, eve_cycle_time_attr_id: 8000},
        eff_ids=[eve_module_ancil_effect_id],
        defeff_id=eve_module_ancil_effect_id)
    eve_drone_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 72, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_drone_effect_id],
        defeff_id=eve_drone_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module_normal = api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_module_ancil = api_fit.add_module(type_id=eve_module_ancil_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(rr_shield=True))
    assert api_fit_stats.rr_shield == [approx(196.65)]
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(rr_shield=True))
    assert api_module_normal_stats.rr_shield == [approx(63.5)]
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(rr_shield=True))
    assert api_module_ancil_stats.rr_shield == [approx(118.75)]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(rr_shield=True))
    assert api_drone_stats.rr_shield == [approx(14.4)]
    # Action
    api_module_normal.change_module(state=consts.ApiModuleState.online)
    api_module_ancil.change_module(state=consts.ApiModuleState.online)
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(rr_shield=True))
    assert api_fit_stats.rr_shield == [0]
    api_stat_options = [StatsOptionItemRr(ignore_state=False), StatsOptionItemRr(ignore_state=True)]
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(rr_shield=(True, api_stat_options)))
    assert api_module_normal_stats.rr_shield == [0, approx(63.5)]
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(rr_shield=(True, api_stat_options)))
    assert api_module_ancil_stats.rr_shield == [0, approx(118.75)]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(rr_shield=(True, api_stat_options)))
    assert api_drone_stats.rr_shield == [0, approx(14.4)]
    # Action
    api_module_normal.change_module(state=consts.ApiModuleState.active)
    api_module_ancil.change_module(state=consts.ApiModuleState.active)
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(rr_shield=True))
    assert api_fit_stats.rr_shield == [approx(196.65)]
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(rr_shield=True))
    assert api_module_normal_stats.rr_shield == [approx(63.5)]
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(rr_shield=True))
    assert api_module_ancil_stats.rr_shield == [approx(118.75)]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(rr_shield=True))
    assert api_drone_stats.rr_shield == [approx(14.4)]


def test_zero_cycle_time(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_normal_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_shield_booster,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_ancil_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_ancillary_remote_shield_booster,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.npc_entity_remote_shield_booster,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_normal_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 508, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_module_normal_effect_id],
        defeff_id=eve_module_normal_effect_id)
    eve_module_ancil_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 950, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_module_ancil_effect_id],
        defeff_id=eve_module_ancil_effect_id)
    eve_drone_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 72, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_drone_effect_id],
        defeff_id=eve_drone_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module_normal = api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_module_ancil = api_fit.add_module(type_id=eve_module_ancil_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(rr_shield=True))
    assert api_fit_stats.rr_shield == [0]
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(rr_shield=True))
    assert api_module_normal_stats.rr_shield == [0]
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(rr_shield=True))
    assert api_module_ancil_stats.rr_shield == [0]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(rr_shield=True))
    assert api_drone_stats.rr_shield == [0]


def test_no_cycle_time(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_bonus)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_normal_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_shield_booster,
        cat_id=consts.EveEffCat.target)
    eve_module_ancil_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_ancillary_remote_shield_booster,
        cat_id=consts.EveEffCat.target)
    eve_drone_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.npc_entity_remote_shield_booster,
        cat_id=consts.EveEffCat.target)
    eve_module_normal_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 508, eve_cycle_time_attr_id: 8000},
        eff_ids=[eve_module_normal_effect_id],
        defeff_id=eve_module_normal_effect_id)
    eve_module_ancil_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 950, eve_cycle_time_attr_id: 8000},
        eff_ids=[eve_module_ancil_effect_id],
        defeff_id=eve_module_ancil_effect_id)
    eve_drone_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 72, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_drone_effect_id],
        defeff_id=eve_drone_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module_normal = api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_module_ancil = api_fit.add_module(type_id=eve_module_ancil_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_fit_stats = api_fit.get_stats(options=FitStatsOptions(rr_shield=True))
    assert api_fit_stats.rr_shield == [0]
    api_module_normal_stats = api_module_normal.get_stats(options=ItemStatsOptions(rr_shield=True))
    assert api_module_normal_stats.rr_shield == [0]
    api_module_ancil_stats = api_module_ancil.get_stats(options=ItemStatsOptions(rr_shield=True))
    assert api_module_ancil_stats.rr_shield == [0]
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(rr_shield=True))
    assert api_drone_stats.rr_shield == [0]


def test_item_not_loaded(client, consts):
    eve_item_id = client.alloc_item_id()
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_item_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_item_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rr_shield=True))
    assert api_stats.rr_shield == [0]
    api_module_stats = api_module.get_stats(options=ItemStatsOptions(rr_shield=True))
    assert api_module_stats.rr_shield is None
    api_drone_stats = api_drone.get_stats(options=ItemStatsOptions(rr_shield=True))
    assert api_drone_stats.rr_shield is None
