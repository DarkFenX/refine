"""
Ancillary reps are special in the way they handle charges - they are the only modules which can
operate while having insufficient charges loaded. In this case, extra reps scale with count of
loaded charges. Here, we test this, as well as a few other things.
"""

import typing
from dataclasses import dataclass

from fw import approx
from fw.api import FitStatsOptions, StatsOptionFitOutRps, StatsOptionRps, StatTimeBurst, StatTimeSim
from fw.util import Absent, Default, conditional_insert

if typing.TYPE_CHECKING:
    from fw.client import TestClient


@dataclass(kw_only=True)
class AarBasicInfo:
    # Attrs
    armor_hp_attr_id: int
    armor_rep_amount_attr_id: int
    armor_rep_amount_mult_attr_id: int
    cycle_time_attr_id: int
    rr_optimal_attr_id: int
    rr_falloff_attr_id: int
    rr_res_attr_id: int
    volume_attr_id: int
    capacity_attr_id: int
    charge_rate_attr_id: int
    reload_time_attr_id: int
    # Effects
    local_aar_effect_id: int
    remote_aar_effect_id: int
    # items
    paste_id: int


def setup_aar_basics(
        *,
        client: TestClient,
        consts,
        effect_duration: bool = True,
) -> AarBasicInfo:
    # Attributes
    eve_armor_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_armor_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_armor_rep_amount_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_rr_optimal_attr_id = client.mk_eve_attr()
    eve_rr_falloff_attr_id = client.mk_eve_attr()
    eve_rr_res_attr_id = client.mk_eve_attr(def_val=1)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_reload_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    # Effects
    eve_local_aar_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fueled_armor_repair,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id if effect_duration else Default)
    eve_remote_aar_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_ancillary_remote_armor_repairer,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id if effect_duration else Default,
        range_attr_id=eve_rr_optimal_attr_id,
        falloff_attr_id=eve_rr_falloff_attr_id,
        resist_attr_id=eve_rr_res_attr_id)
    client.mk_eve_item(eff_ids=[eve_local_aar_effect_id, eve_remote_aar_effect_id])
    eve_paste_id = client.mk_eve_item(id_=consts.EveItem.nanite_repair_paste, attrs={eve_volume_attr_id: 0.01})
    return AarBasicInfo(
        armor_hp_attr_id=eve_armor_hp_attr_id,
        armor_rep_amount_attr_id=eve_armor_rep_amount_attr_id,
        armor_rep_amount_mult_attr_id=eve_armor_rep_amount_mult_attr_id,
        cycle_time_attr_id=eve_cycle_time_attr_id,
        rr_optimal_attr_id=eve_rr_optimal_attr_id,
        rr_falloff_attr_id=eve_rr_falloff_attr_id,
        rr_res_attr_id=eve_rr_res_attr_id,
        volume_attr_id=eve_volume_attr_id,
        capacity_attr_id=eve_capacity_attr_id,
        charge_rate_attr_id=eve_charge_rate_attr_id,
        local_aar_effect_id=eve_local_aar_effect_id,
        remote_aar_effect_id=eve_remote_aar_effect_id,
        reload_time_attr_id=eve_reload_time_attr_id,
        paste_id=eve_paste_id)


def make_eve_local_aar(
        *,
        client: TestClient,
        basic_info: AarBasicInfo,
        rep_amount: float | type[Absent] = Absent,
        cycle_time: float | type[Absent] = Absent,
        capacity: float | type[Absent] = Absent,
        charge_rate: float | type[Absent] = Absent,
        reload_time: float | type[Absent] = Absent,
) -> int:
    attrs = {basic_info.armor_rep_amount_mult_attr_id: 3.0}
    conditional_insert(container=attrs, path=[basic_info.armor_rep_amount_attr_id], value=rep_amount)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.capacity_attr_id], value=capacity)
    conditional_insert(container=attrs, path=[basic_info.charge_rate_attr_id], value=charge_rate)
    conditional_insert(container=attrs, path=[basic_info.reload_time_attr_id], value=reload_time)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.local_aar_effect_id],
        defeff_id=basic_info.local_aar_effect_id)


def make_eve_remote_aar(
        *,
        client: TestClient,
        basic_info: AarBasicInfo,
        rep_amount: float | type[Absent] = Absent,
        cycle_time: float | type[Absent] = Absent,
        capacity: float | type[Absent] = Absent,
        charge_rate: float | type[Absent] = Absent,
        optimal_range: float | type[Absent] = Absent,
        falloff_range: float | type[Absent] = Absent,
        reload_time: float | type[Absent] = Absent,
) -> int:
    attrs = {basic_info.armor_rep_amount_mult_attr_id: 3.0}
    conditional_insert(container=attrs, path=[basic_info.armor_rep_amount_attr_id], value=rep_amount)
    conditional_insert(container=attrs, path=[basic_info.cycle_time_attr_id], value=cycle_time)
    conditional_insert(container=attrs, path=[basic_info.capacity_attr_id], value=capacity)
    conditional_insert(container=attrs, path=[basic_info.charge_rate_attr_id], value=charge_rate)
    conditional_insert(container=attrs, path=[basic_info.rr_optimal_attr_id], value=optimal_range)
    conditional_insert(container=attrs, path=[basic_info.rr_falloff_attr_id], value=falloff_range)
    conditional_insert(container=attrs, path=[basic_info.reload_time_attr_id], value=reload_time)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.remote_aar_effect_id],
        defeff_id=basic_info.remote_aar_effect_id)


def test_local_chargedness(client, consts):
    eve_basic_info = setup_aar_basics(client=client, consts=consts)
    eve_module1_id = make_eve_local_aar(
        client=client, basic_info=eve_basic_info,
        rep_amount=207, cycle_time=9000, capacity=0.31, charge_rate=4, reload_time=60000)
    eve_module2_id = make_eve_local_aar(
        client=client, basic_info=eve_basic_info,
        rep_amount=207, cycle_time=9000, capacity=0.03, charge_rate=4, reload_time=60000)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.armor_hp_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_fit = api_sol.create_fit()
    api_fit.set_ship(type_id=eve_ship_id)
    api_module = api_fit.add_module(
        type_id=eve_module1_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_basic_info.paste_id)
    # Verification - rep has 7 full cycles, and 3/4-charged 8th
    api_stats = api_fit.get_stats(options=FitStatsOptions(hp=True, rps=(True, [
        StatsOptionRps(time_options=StatTimeBurst()),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.on_empty)),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.disabled))])))
    assert api_stats.hp.armor.ancil_local == approx(4864.5)
    assert api_stats.rps.map(lambda i: i.armor.local) == [approx(69), approx(36.852273), approx(23)]
    # Action
    api_module.change_module(type_id=eve_module2_id)
    # Verification - rep has only one 3/4-charged cycle
    api_stats = api_fit.get_stats(options=FitStatsOptions(hp=True, rps=(True, [
        StatsOptionRps(time_options=StatTimeBurst()),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.on_empty)),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.disabled))])))
    assert api_stats.hp.armor.ancil_local == approx(517.5)
    assert api_stats.rps.map(lambda i: i.armor.local) == [approx(57.5), approx(7.5), approx(23)]


def test_remote_chargedness(client, consts):
    eve_basic_info = setup_aar_basics(client=client, consts=consts)
    eve_module1_id = make_eve_remote_aar(
        client=client, basic_info=eve_basic_info, rep_amount=290, cycle_time=6000, capacity=0.63, charge_rate=8)
    eve_module2_id = make_eve_remote_aar(
        client=client, basic_info=eve_basic_info, rep_amount=290, cycle_time=6000, capacity=0.05, charge_rate=8)
    eve_ship_id = client.mk_eve_ship(attrs={eve_basic_info.armor_hp_attr_id: 1000})
    client.create_sources()
    api_sol = client.create_sol()
    api_src_fit = api_sol.create_fit()
    api_src_module = api_src_fit.add_module(
        type_id=eve_module1_id,
        state=consts.ApiModuleState.active,
        charge_type_id=eve_basic_info.paste_id)
    api_tgt_fit = api_sol.create_fit()
    api_tgt_ship = api_tgt_fit.set_ship(type_id=eve_ship_id)
    api_src_module.change_module(add_projs=[api_tgt_ship.id])
    # Verification - rep has 7 full cycles, and 7/8-charged 8th
    api_src_stats = api_src_fit.get_stats(options=FitStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(time_options=StatTimeBurst()),
        StatsOptionFitOutRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.on_empty)),
        StatsOptionFitOutRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.disabled))])))
    assert api_src_stats.outgoing_rps.map(lambda i: i.armor) == [approx(145), approx(140.561224), approx(48.333333)]
    api_tgt_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True, rps=(True, [
        StatsOptionRps(time_options=StatTimeBurst()),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.on_empty)),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.disabled))])))
    assert api_tgt_stats.hp.armor.ancil_remote == approx(6887.5)
    assert api_tgt_stats.rps.map(lambda i: i.armor.remote) == [approx(145), approx(140.561224), approx(48.333333)]
    # Action
    api_src_module.change_module(type_id=eve_module2_id)
    # Verification - rep has only one 5/8-charged cycle
    api_src_stats = api_src_fit.get_stats(options=FitStatsOptions(outgoing_rps=(True, [
        StatsOptionFitOutRps(time_options=StatTimeBurst()),
        StatsOptionFitOutRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.on_empty)),
        StatsOptionFitOutRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.disabled))])))
    assert api_src_stats.outgoing_rps.map(lambda i: i.armor) == [approx(108.75), approx(93.214286), approx(48.333333)]
    api_tgt_stats = api_tgt_fit.get_stats(options=FitStatsOptions(hp=True, rps=(True, [
        StatsOptionRps(time_options=StatTimeBurst()),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.on_empty)),
        StatsOptionRps(time_options=StatTimeSim(optional_reloads=consts.ApiOptionalReload.disabled))])))
    assert api_tgt_stats.hp.armor.ancil_remote == approx(652.5)
    assert api_tgt_stats.rps.map(lambda i: i.armor.remote) == [approx(108.75), approx(93.214286), approx(48.333333)]
