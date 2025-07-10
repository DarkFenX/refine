from tests import approx
from tests.fw.api import StatsOptions


def test_state(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_ancil_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_normal_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_armor_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_ancil_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_ancillary_remote_armor_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_spool_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_armor_mutadaptive_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.npc_entity_remote_armor_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_normal_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 376, eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_module_normal_effect_id],
        defeff_id=eve_module_normal_effect_id)
    eve_module_ancil_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 145, eve_cycle_time_attr_id: 6000, eve_ancil_mult_attr_id: 3},
        eff_ids=[eve_module_ancil_effect_id],
        defeff_id=eve_module_ancil_effect_id)
    eve_module_spool_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 512, eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_module_spool_effect_id],
        defeff_id=eve_module_spool_effect_id)
    eve_drone_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 72, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_drone_effect_id],
        defeff_id=eve_drone_effect_id)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_module_normal = api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_module_ancil = api_fit.add_module(
        type_id=eve_module_ancil_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_paste_id)
    api_module_spool = api_fit.add_module(type_id=eve_module_spool_id, state=consts.ApiModuleState.active)
    api_drone = api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(rr_armor=True))
    assert api_stats.rr_armor == approx(234.9)
    # Action
    api_module_normal.change_module(state=consts.ApiModuleState.online)
    api_module_ancil.change_module(state=consts.ApiModuleState.online)
    api_module_spool.change_module(state=consts.ApiModuleState.online)
    api_drone.change_drone(state=consts.ApiMinionState.in_space)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(rr_armor=True))
    assert api_stats.rr_armor == approx(0)
    # Action
    api_module_normal.change_module(state=consts.ApiModuleState.active)
    api_module_ancil.change_module(state=consts.ApiModuleState.active)
    api_module_spool.change_module(state=consts.ApiModuleState.active)
    api_drone.change_drone(state=consts.ApiMinionState.engaging)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(rr_armor=True))
    assert api_stats.rr_armor == approx(234.9)


def test_zero_cycle_time(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_ancil_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_normal_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_armor_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_ancil_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_ancillary_remote_armor_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_spool_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_armor_mutadaptive_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_drone_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.npc_entity_remote_armor_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module_normal_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 376, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_module_normal_effect_id],
        defeff_id=eve_module_normal_effect_id)
    eve_module_ancil_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 145, eve_cycle_time_attr_id: 0, eve_ancil_mult_attr_id: 3},
        eff_ids=[eve_module_ancil_effect_id],
        defeff_id=eve_module_ancil_effect_id)
    eve_module_spool_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 512, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_module_spool_effect_id],
        defeff_id=eve_module_spool_effect_id)
    eve_drone_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 72, eve_cycle_time_attr_id: 0},
        eff_ids=[eve_drone_effect_id],
        defeff_id=eve_drone_effect_id)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_fit.add_module(
        type_id=eve_module_ancil_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_paste_id)
    api_fit.add_module(type_id=eve_module_spool_id, state=consts.ApiModuleState.active)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(rr_armor=True))
    assert api_stats.rr_armor == approx(0)


def test_no_cycle_time(client, consts):
    eve_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_ancil_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_module_normal_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_armor_repairer,
        cat_id=consts.EveEffCat.target)
    eve_module_ancil_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_ancillary_remote_armor_repairer,
        cat_id=consts.EveEffCat.target)
    eve_module_spool_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_remote_armor_mutadaptive_repairer,
        cat_id=consts.EveEffCat.target)
    eve_drone_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.npc_entity_remote_armor_repairer,
        cat_id=consts.EveEffCat.target)
    eve_module_normal_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 376, eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_module_normal_effect_id],
        defeff_id=eve_module_normal_effect_id)
    eve_module_ancil_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 145, eve_cycle_time_attr_id: 6000, eve_ancil_mult_attr_id: 3},
        eff_ids=[eve_module_ancil_effect_id],
        defeff_id=eve_module_ancil_effect_id)
    eve_module_spool_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 512, eve_cycle_time_attr_id: 6000},
        eff_ids=[eve_module_spool_effect_id],
        defeff_id=eve_module_spool_effect_id)
    eve_drone_id = client.mk_eve_item(
        attrs={eve_rep_amount_attr_id: 72, eve_cycle_time_attr_id: 5000},
        eff_ids=[eve_drone_effect_id],
        defeff_id=eve_drone_effect_id)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste)
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.add_module(type_id=eve_module_normal_id, state=consts.ApiModuleState.active)
    api_fit.add_module(
        type_id=eve_module_ancil_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_paste_id)
    api_fit.add_module(type_id=eve_module_spool_id, state=consts.ApiModuleState.active)
    api_fit.add_drone(type_id=eve_drone_id, state=consts.ApiMinionState.engaging)
    # Verification
    api_stats = api_fit.get_stats(options=StatsOptions(rr_armor=True))
    assert api_stats.rr_armor == approx(0)
