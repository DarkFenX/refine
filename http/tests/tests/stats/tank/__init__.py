from __future__ import annotations

import typing
from dataclasses import dataclass

if typing.TYPE_CHECKING:
    from tests.fw.client import TestClient


@dataclass(kw_only=True)
class TankBasicInfo:
    # Buffer attrs
    res_max_attr_id: int
    shield_hp_attr_id: int
    shield_res_em_attr_id: int
    shield_res_therm_attr_id: int
    shield_res_kin_attr_id: int
    shield_res_expl_attr_id: int
    armor_hp_attr_id: int
    armor_res_em_attr_id: int
    armor_res_therm_attr_id: int
    armor_res_kin_attr_id: int
    armor_res_expl_attr_id: int
    hull_hp_attr_id: int
    hull_res_em_attr_id: int
    hull_res_therm_attr_id: int
    hull_res_kin_attr_id: int
    hull_res_expl_attr_id: int
    # Other attrs
    shield_rep_amount_attr_id: int
    armor_rep_amount_attr_id: int
    armor_rep_amount_mult_attr_id: int
    rr_optimal_attr_id: int
    rr_falloff_attr_id: int
    rr_res_attr_id: int
    volume_attr_id: int
    capacity_attr_id: int
    charge_rate_attr_id: int
    max_fighter_count_attr_id: int
    # Effects
    local_asb_effect_id: int
    local_aar_effect_id: int
    remote_asb_effect_id: int
    remote_aar_effect_id: int


def setup_tank_basics(
        *,
        client: TestClient,
        consts,  # noqa: ANN001
) -> TankBasicInfo:
    # Tanking attrs
    eve_res_max_attr_id = client.mk_eve_attr(def_val=1)
    eve_shield_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_capacity)
    eve_shield_em_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.shield_em_dmg_resonance,
        def_val=1,
        max_attr_id=eve_res_max_attr_id)
    eve_shield_therm_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.shield_therm_dmg_resonance,
        def_val=1,
        max_attr_id=eve_res_max_attr_id)
    eve_shield_kin_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.shield_kin_dmg_resonance,
        def_val=1,
        max_attr_id=eve_res_max_attr_id)
    eve_shield_expl_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.shield_expl_dmg_resonance,
        def_val=1,
        max_attr_id=eve_res_max_attr_id)
    eve_armor_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_hp)
    eve_armor_em_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_em_dmg_resonance,
        def_val=1,
        max_attr_id=eve_res_max_attr_id)
    eve_armor_therm_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_therm_dmg_resonance,
        def_val=1,
        max_attr_id=eve_res_max_attr_id)
    eve_armor_kin_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_kin_dmg_resonance,
        def_val=1,
        max_attr_id=eve_res_max_attr_id)
    eve_armor_expl_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.armor_expl_dmg_resonance,
        def_val=1,
        max_attr_id=eve_res_max_attr_id)
    eve_hull_hp_attr_id = client.mk_eve_attr(id_=consts.EveAttr.hp)
    eve_hull_em_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.em_dmg_resonance,
        def_val=1,
        max_attr_id=eve_res_max_attr_id)
    eve_hull_therm_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.therm_dmg_resonance,
        def_val=1,
        max_attr_id=eve_res_max_attr_id)
    eve_hull_kin_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.kin_dmg_resonance,
        def_val=1,
        max_attr_id=eve_res_max_attr_id)
    eve_hull_expl_attr_id = client.mk_eve_attr(
        id_=consts.EveAttr.expl_dmg_resonance,
        def_val=1,
        max_attr_id=eve_res_max_attr_id)
    # Rep attributes
    eve_shield_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.shield_bonus)
    eve_armor_rep_amount_attr_id = client.mk_eve_attr(id_=consts.EveAttr.armor_dmg_amount)
    eve_armor_rep_amount_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charged_armor_dmg_mult)
    eve_rr_optimal_attr_id = client.mk_eve_attr()
    eve_rr_falloff_attr_id = client.mk_eve_attr()
    eve_rr_res_attr_id = client.mk_eve_attr(def_val=1)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    # Fighter-specific attribute
    eve_max_fighter_count_attr_id = client.mk_eve_attr(id_=consts.EveAttr.ftr_sq_max_size)
    # Effects
    eve_local_asb_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fueled_shield_boosting,
        cat_id=consts.EveEffCat.active)
    eve_local_aar_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.fueled_armor_repair,
        cat_id=consts.EveEffCat.active)
    eve_remote_asb_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_ancillary_remote_shield_booster,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_rr_optimal_attr_id,
        falloff_attr_id=eve_rr_falloff_attr_id,
        resist_attr_id=eve_rr_res_attr_id)
    eve_remote_aar_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.ship_mod_ancillary_remote_armor_repairer,
        cat_id=consts.EveEffCat.target,
        range_attr_id=eve_rr_optimal_attr_id,
        falloff_attr_id=eve_rr_falloff_attr_id,
        resist_attr_id=eve_rr_res_attr_id)
    # Ensure effects are not cleaned up
    client.mk_eve_item(eff_ids=[
        eve_local_asb_effect_id,
        eve_local_aar_effect_id,
        eve_remote_asb_effect_id,
        eve_remote_aar_effect_id])
    return TankBasicInfo(
        res_max_attr_id=eve_res_max_attr_id,
        shield_hp_attr_id=eve_shield_hp_attr_id,
        shield_res_em_attr_id=eve_shield_em_attr_id,
        shield_res_therm_attr_id=eve_shield_therm_attr_id,
        shield_res_kin_attr_id=eve_shield_kin_attr_id,
        shield_res_expl_attr_id=eve_shield_expl_attr_id,
        armor_hp_attr_id=eve_armor_hp_attr_id,
        armor_res_em_attr_id=eve_armor_em_attr_id,
        armor_res_therm_attr_id=eve_armor_therm_attr_id,
        armor_res_kin_attr_id=eve_armor_kin_attr_id,
        armor_res_expl_attr_id=eve_armor_expl_attr_id,
        hull_hp_attr_id=eve_hull_hp_attr_id,
        hull_res_em_attr_id=eve_hull_em_attr_id,
        hull_res_therm_attr_id=eve_hull_therm_attr_id,
        hull_res_kin_attr_id=eve_hull_kin_attr_id,
        hull_res_expl_attr_id=eve_hull_expl_attr_id,
        max_fighter_count_attr_id=eve_max_fighter_count_attr_id,
        shield_rep_amount_attr_id=eve_shield_rep_amount_attr_id,
        armor_rep_amount_attr_id=eve_armor_rep_amount_attr_id,
        armor_rep_amount_mult_attr_id=eve_armor_rep_amount_mult_attr_id,
        rr_optimal_attr_id=eve_rr_optimal_attr_id,
        rr_falloff_attr_id=eve_rr_falloff_attr_id,
        rr_res_attr_id=eve_rr_res_attr_id,
        volume_attr_id=eve_volume_attr_id,
        capacity_attr_id=eve_capacity_attr_id,
        charge_rate_attr_id=eve_charge_rate_attr_id,
        local_asb_effect_id=eve_local_asb_effect_id,
        local_aar_effect_id=eve_local_aar_effect_id,
        remote_asb_effect_id=eve_remote_asb_effect_id,
        remote_aar_effect_id=eve_remote_aar_effect_id)


def make_eve_tankable(
        *,
        client: TestClient,
        basic_info: TankBasicInfo,
        hps: tuple[float | None, float | None, float | None] | None = None,
        resos_shield: tuple[float | None, float | None, float | None, float | None] | None = None,
        resos_armor: tuple[float | None, float | None, float | None, float | None] | None = None,
        resos_hull: tuple[float | None, float | None, float | None, float | None] | None = None,
        rr_resist: float | None = None,
        fighter_count: float | None = None,
        ship: bool = False,
) -> int:
    attrs = {}
    if hps is not None:
        hp_attr_ids = (basic_info.shield_hp_attr_id, basic_info.armor_hp_attr_id, basic_info.hull_hp_attr_id)
        attrs.update({k: v for k, v in zip(hp_attr_ids, hps, strict=True) if v is not None})
    if resos_shield is not None:
        shield_res_attr_ids = (
            basic_info.shield_res_em_attr_id,
            basic_info.shield_res_therm_attr_id,
            basic_info.shield_res_kin_attr_id,
            basic_info.shield_res_expl_attr_id)
        attrs.update({k: v for k, v in zip(shield_res_attr_ids, resos_shield, strict=True) if v is not None})
    if resos_armor is not None:
        armor_res_attr_ids = (
            basic_info.armor_res_em_attr_id,
            basic_info.armor_res_therm_attr_id,
            basic_info.armor_res_kin_attr_id,
            basic_info.armor_res_expl_attr_id)
        attrs.update({k: v for k, v in zip(armor_res_attr_ids, resos_armor, strict=True) if v is not None})
    if resos_hull is not None:
        hull_res_attr_ids = (
            basic_info.hull_res_em_attr_id,
            basic_info.hull_res_therm_attr_id,
            basic_info.hull_res_kin_attr_id,
            basic_info.hull_res_expl_attr_id)
        attrs.update({k: v for k, v in zip(hull_res_attr_ids, resos_hull, strict=True) if v is not None})
    if rr_resist is not None:
        attrs[basic_info.rr_res_attr_id] = rr_resist
    if fighter_count is not None:
        attrs[basic_info.max_fighter_count_attr_id] = fighter_count
    maker = client.mk_eve_ship if ship else client.mk_eve_item
    return maker(attrs=attrs)


def make_eve_local_asb(
        *,
        client: TestClient,
        basic_info: TankBasicInfo,
        rep_amount: float | None = None,
        capacity: float | None = None,
) -> int:
    attrs = {basic_info.charge_rate_attr_id: 1.0}
    if rep_amount is not None:
        attrs[basic_info.shield_rep_amount_attr_id] = rep_amount
    if capacity is not None:
        attrs[basic_info.capacity_attr_id] = capacity
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.local_asb_effect_id],
        defeff_id=basic_info.local_asb_effect_id)


def make_eve_local_aar(
        *,
        client: TestClient,
        basic_info: TankBasicInfo,
        rep_amount: float | None = None,
        capacity: float | None = None,
        charge_rate: float | None = None,
) -> int:
    attrs = {basic_info.armor_rep_amount_mult_attr_id: 3.0}
    if rep_amount is not None:
        attrs[basic_info.armor_rep_amount_attr_id] = rep_amount
    if capacity is not None:
        attrs[basic_info.capacity_attr_id] = capacity
    if charge_rate is not None:
        attrs[basic_info.charge_rate_attr_id] = charge_rate
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.local_aar_effect_id],
        defeff_id=basic_info.local_aar_effect_id)


def make_eve_remote_asb(
        *,
        client: TestClient,
        basic_info: TankBasicInfo,
        rep_amount: float | None = None,
        capacity: float | None = None,
        optimal_range: float | None = None,
        falloff_range: float | None = None,
) -> int:
    attrs = {basic_info.charge_rate_attr_id: 1.0}
    if rep_amount is not None:
        attrs[basic_info.shield_rep_amount_attr_id] = rep_amount
    if capacity is not None:
        attrs[basic_info.capacity_attr_id] = capacity
    if optimal_range is not None:
        attrs[basic_info.rr_optimal_attr_id] = optimal_range
    if falloff_range is not None:
        attrs[basic_info.rr_falloff_attr_id] = falloff_range
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.remote_asb_effect_id],
        defeff_id=basic_info.remote_asb_effect_id)


def make_eve_remote_aar(
        *,
        client: TestClient,
        basic_info: TankBasicInfo,
        rep_amount: float | None = None,
        capacity: float | None = None,
        charge_rate: float | None = None,
        optimal_range: float | None = None,
        falloff_range: float | None = None,
) -> int:
    attrs = {basic_info.armor_rep_amount_mult_attr_id: 3.0}
    if rep_amount is not None:
        attrs[basic_info.armor_rep_amount_attr_id] = rep_amount
    if capacity is not None:
        attrs[basic_info.capacity_attr_id] = capacity
    if charge_rate is not None:
        attrs[basic_info.charge_rate_attr_id] = charge_rate
    if optimal_range is not None:
        attrs[basic_info.rr_optimal_attr_id] = optimal_range
    if falloff_range is not None:
        attrs[basic_info.rr_falloff_attr_id] = falloff_range
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.remote_aar_effect_id],
        defeff_id=basic_info.remote_aar_effect_id)
