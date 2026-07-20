"""
Ancillary reps are special in the way they handle charges - they are the only modules which can
operate while having insufficient charges loaded. In this case, extra reps scale with count of
loaded charges. Here, we test this, as well as a few other things.

Lots of functionality related to AAR is tested elsewhere, this module is for tests which are hard to
fit elsewhere, since they are too niche.
"""

from fw import approx
from fw.api import FitStatsOptions, StatsOptionFitOutRps, StatsOptionRps, StatTimeBurst, StatTimeSim


def test_chargedness_local(client, consts):
    eve_armor_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_armor_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_armor_rep_amount_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_reload_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fueled_armor_repair,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module1_id = client.mk_eve_item(
        attrs={
            eve_armor_rep_amount_attr_id: 207, eve_armor_rep_amount_mult_attr_id: 3,
            eve_cycle_time_attr_id: 9000, eve_capacity_attr_id: 0.31,
            eve_charge_rate_attr_id: 4, eve_reload_time_attr_id: 60000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(
        attrs={
            eve_armor_rep_amount_attr_id: 207, eve_armor_rep_amount_mult_attr_id: 3,
            eve_cycle_time_attr_id: 9000, eve_capacity_attr_id: 0.03,
            eve_charge_rate_attr_id: 4, eve_reload_time_attr_id: 60000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 0.01})
    eve_ship_id = client.mk_eve_ship(attrs={eve_armor_hp_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_module1_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_paste_id)
    # Verification - rep has 7 full cycles, and 3/4-charged 8th
    api_stats = api_fit.get_stats(options=FitStatsOptions(hp=True, rps=(True, [
        StatsOptionRps(time_options=StatTimeBurst()),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.on_empty)),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.disabled))])))
    assert api_stats.hp.one().armor.ancil_local == approx(4864.5)
    assert api_stats.rps.map(lambda i: i.armor.local) == [approx(69), approx(36.852273), approx(23)]
    # Action
    api_module.change_module(type_id=eve_module2_id)
    # Verification - rep has only one 3/4-charged cycle
    api_stats = api_fit.get_stats(options=FitStatsOptions(hp=True, rps=(True, [
        StatsOptionRps(time_options=StatTimeBurst()),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.on_empty)),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.disabled))])))
    assert api_stats.hp.one().armor.ancil_local == approx(517.5)
    assert api_stats.rps.map(lambda i: i.armor.local) == [approx(57.5), approx(7.5), approx(23)]


def test_chargedness_remote(client, consts):
    eve_armor_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_armor_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_armor_rep_amount_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_reload_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_ancillary_remote_armor_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_module1_id = client.mk_eve_item(
        attrs={
            eve_armor_rep_amount_attr_id: 290, eve_armor_rep_amount_mult_attr_id: 3,
            eve_cycle_time_attr_id: 6000, eve_capacity_attr_id: 0.63,
            eve_charge_rate_attr_id: 8, eve_reload_time_attr_id: 60000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_module2_id = client.mk_eve_item(
        attrs={
            eve_armor_rep_amount_attr_id: 290, eve_armor_rep_amount_mult_attr_id: 3,
            eve_cycle_time_attr_id: 6000, eve_capacity_attr_id: 0.05,
            eve_charge_rate_attr_id: 8, eve_reload_time_attr_id: 60000},
        eff_ids=[eve_effect_id],
        defeff_id=eve_effect_id)
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 0.01})
    eve_ship_id = client.mk_eve_ship(attrs={eve_armor_hp_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(
        type_id=eve_module1_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_paste_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_module.change_module(add_proj_item_ids=[api_tgt_ship.id])
    # Verification - rep has 7 full cycles, and 7/8-charged 8th
    api_src_stats = api_src_fit.get_stats(options=FitStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(time_options=StatTimeBurst()),
        StatsOptionFitOutRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.on_empty)),
        StatsOptionFitOutRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.disabled))])))
    assert api_src_stats.outgoing_rps.map(lambda i: i.armor) == [approx(145), approx(63.773148), approx(48.333333)]
    api_tgt_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True, rps=(True, [
        StatsOptionRps(time_options=StatTimeBurst()),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.on_empty)),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.disabled))])))
    assert api_tgt_stats.hp.one().armor.ancil_remote == approx(6887.5)
    assert api_tgt_stats.rps.map(lambda i: i.armor.remote) == [approx(145), approx(63.773148), approx(48.333333)]
    # Action
    api_src_module.change_module(type_id=eve_module2_id)
    # Verification - rep has only one 5/8-charged cycle
    api_src_stats = api_src_fit.get_stats(options=FitStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(time_options=StatTimeBurst()),
        StatsOptionFitOutRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.on_empty)),
        StatsOptionFitOutRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.disabled))])))
    assert api_src_stats.outgoing_rps.map(lambda i: i.armor) == [approx(108.75), approx(9.886364), approx(48.333333)]
    api_tgt_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True, rps=(True, [
        StatsOptionRps(time_options=StatTimeBurst()),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.on_empty)),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.disabled))])))
    assert api_tgt_stats.hp.one().armor.ancil_remote == approx(652.5)
    assert api_tgt_stats.rps.map(lambda i: i.armor.remote) == [approx(108.75), approx(9.886364), approx(48.333333)]
