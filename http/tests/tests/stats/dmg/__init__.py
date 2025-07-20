from __future__ import annotations

import typing
from dataclasses import dataclass

from tests.fw.util import Default

if typing.TYPE_CHECKING:
    from tests.fw.client import TestClient


@dataclass(kw_only=True)
class DmgBasicInfo:
    dmg_em_attr_id: int
    dmg_therm_attr_id: int
    dmg_kin_attr_id: int
    dmg_expl_attr_id: int
    dmg_mult_attr_id: int
    dd_delay_attr_id: int
    dd_dmg_interval_attr_id: int
    dd_dmg_duration_attr_id: int
    capacity_attr_id: int
    volume_attr_id: int
    charge_rate_attr_id: int
    cycle_time_attr_id: int
    reload_time_attr_id: int
    turret_proj_effect_id: int
    dd_lance_debuff_effect_id: int


def setup_dmg_basics(
        *,
        client: TestClient,
        consts,  # noqa: ANN001
        effect_duration: bool = True,
) -> DmgBasicInfo:
    eve_dmg_em_attr_id = client.mk_eve_attr(id_=consts.EveAttr.em_dmg)
    eve_dmg_therm_attr_id = client.mk_eve_attr(id_=consts.EveAttr.therm_dmg)
    eve_dmg_kin_attr_id = client.mk_eve_attr(id_=consts.EveAttr.kin_dmg)
    eve_dmg_expl_attr_id = client.mk_eve_attr(id_=consts.EveAttr.expl_dmg)
    eve_dmg_mult_attr_id = client.mk_eve_attr(id_=consts.EveAttr.dmg_mult)
    eve_dd_delay_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_warning_duration)
    eve_dd_dmg_interval_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_dmg_cycle_time)
    eve_dd_dmg_duration_attr_id = client.mk_eve_attr(id_=consts.EveAttr.doomsday_dmg_duration)
    eve_capacity_attr_id = client.mk_eve_attr(id_=consts.EveAttr.capacity)
    eve_volume_attr_id = client.mk_eve_attr(id_=consts.EveAttr.volume)
    eve_charge_rate_attr_id = client.mk_eve_attr(id_=consts.EveAttr.charge_rate)
    eve_cycle_time_attr_id = client.mk_eve_attr()
    eve_reload_time_attr_id = client.mk_eve_attr(id_=consts.EveAttr.reload_time)
    eve_turret_proj_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.projectile_fired,
        cat_id=consts.EveEffCat.target,
        duration_attr_id=eve_cycle_time_attr_id if effect_duration else Default)
    eve_dd_lance_debuff_effect_id = client.mk_eve_effect(
        id_=consts.EveEffect.debuff_lance,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id if effect_duration else Default)
    # Ensure effects are not cleaned up
    client.mk_eve_item(eff_ids=[eve_turret_proj_effect_id, eve_dd_lance_debuff_effect_id])
    return DmgBasicInfo(
        dmg_em_attr_id=eve_dmg_em_attr_id,
        dmg_therm_attr_id=eve_dmg_therm_attr_id,
        dmg_kin_attr_id=eve_dmg_kin_attr_id,
        dmg_expl_attr_id=eve_dmg_expl_attr_id,
        dmg_mult_attr_id=eve_dmg_mult_attr_id,
        dd_delay_attr_id=eve_dd_delay_attr_id,
        dd_dmg_interval_attr_id=eve_dd_dmg_interval_attr_id,
        dd_dmg_duration_attr_id=eve_dd_dmg_duration_attr_id,
        cycle_time_attr_id=eve_cycle_time_attr_id,
        volume_attr_id=eve_volume_attr_id,
        capacity_attr_id=eve_capacity_attr_id,
        charge_rate_attr_id=eve_charge_rate_attr_id,
        reload_time_attr_id=eve_reload_time_attr_id,
        turret_proj_effect_id=eve_turret_proj_effect_id,
        dd_lance_debuff_effect_id=eve_dd_lance_debuff_effect_id)


def make_eve_turret_proj(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmg_mult: float | None = None,
        cycle_time: float | None = None,
        capacity: float | None = None,
        reload_time: float | None = None,
) -> int:
    attrs = {basic_info.charge_rate_attr_id: 1.0}
    conditional_insert(attrs=attrs, attr_id=basic_info.dmg_mult_attr_id, value=dmg_mult)
    conditional_insert(attrs=attrs, attr_id=basic_info.cycle_time_attr_id, value=cycle_time)
    conditional_insert(attrs=attrs, attr_id=basic_info.capacity_attr_id, value=capacity)
    conditional_insert(attrs=attrs, attr_id=basic_info.reload_time_attr_id, value=reload_time)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.turret_proj_effect_id],
        defeff_id=basic_info.turret_proj_effect_id)


def make_eve_turret_proj_charge(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        volume: float | None = None,
) -> int:
    attrs = {}
    if dmgs is not None:
        dmg_attr_ids = (
            basic_info.dmg_em_attr_id,
            basic_info.dmg_therm_attr_id,
            basic_info.dmg_kin_attr_id,
            basic_info.dmg_expl_attr_id)
        attrs.update({k: v for k, v in zip(dmg_attr_ids, dmgs, strict=True) if v is not None})
    conditional_insert(attrs=attrs, attr_id=basic_info.volume_attr_id, value=volume)
    return client.mk_eve_item(attrs=attrs)


def make_eve_dd_lance_debuff(
        *,
        client: TestClient,
        basic_info: DmgBasicInfo,
        dmgs: tuple[float | None, float | None, float | None, float | None] | None = None,
        cycle_time: float | None = None,
        delay: float | None = None,
        dmg_interval: float | None = None,
        dmg_duration: float | None = None,
) -> int:
    attrs = {}
    if dmgs is not None:
        dmg_attr_ids = (
            basic_info.dmg_em_attr_id,
            basic_info.dmg_therm_attr_id,
            basic_info.dmg_kin_attr_id,
            basic_info.dmg_expl_attr_id)
        attrs.update({k: v for k, v in zip(dmg_attr_ids, dmgs, strict=True) if v is not None})
    conditional_insert(attrs=attrs, attr_id=basic_info.cycle_time_attr_id, value=cycle_time)
    conditional_insert(attrs=attrs, attr_id=basic_info.dd_delay_attr_id, value=delay)
    conditional_insert(attrs=attrs, attr_id=basic_info.dd_dmg_interval_attr_id, value=dmg_interval)
    conditional_insert(attrs=attrs, attr_id=basic_info.dd_dmg_duration_attr_id, value=dmg_duration)
    return client.mk_eve_item(
        attrs=attrs,
        eff_ids=[basic_info.dd_lance_debuff_effect_id],
        defeff_id=basic_info.dd_lance_debuff_effect_id)


def conditional_insert(*, attrs: dict[int, float], attr_id: int, value: float | None) -> None:
    if value is not None:
        attrs[attr_id] = value
