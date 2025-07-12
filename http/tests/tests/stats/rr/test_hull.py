from tests import approx
from tests.fw.api import FitStatsOptions


def test_state(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.structure_dmg_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_hull_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.npc_entity_remote_hull_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 60, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_drone_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 36, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_drone_effect_id],
        defeff_id=eve_drone_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module = api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rr_hull=True))
    assert api_stats.rr_hull == [approx(9.7)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.online)
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rr_hull=True))
    assert api_stats.rr_hull == [approx(0)]
    # Action
    api_module.change_module(state=consts.ApiModuleState.active)
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rr_hull=True))
    assert api_stats.rr_hull == [approx(9.7)]


def test_zero_cycle_time(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.structure_dmg_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_hull_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.npc_entity_remote_hull_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 60, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_module_effect_id],
        defeff_id=eve_module_effect_id)
    eve_drone_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 36, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_drone_effect_id],
        defeff_id=eve_drone_effect_id)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rr_hull=True))
    assert api_stats.rr_hull == [approx(0)]


def test_no_cycle_time(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.structure_dmg_amount)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_hull_repairer,
        cat_id=consts.EveEffCat.target)
    eve_drone_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.npc_entity_remote_hull_repairer,
        cat_id=consts.EveEffCat.target)
    eve_module_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 60, eve_cycle_time_attr_id: 24000},
        eff_ids=[eve_module_effect_id])
    eve_drone_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 36, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_drone_effect_id])
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_id, state=consts.ApiModuleState.active)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_stats = api_fit.get_stats(options=FitStatsOptions(rr_hull=True))
    assert api_stats.rr_hull == [approx(0)]
