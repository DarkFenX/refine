import typing
from dataclasses import dataclass
from itertools import chain

from fw.util import Default

if typing.TYPE_CHECKING:
    from fw.client import TestClient
    from fw.eve import EveObjects


@dataclass(kw_only=True)
class RahBasicInfo:
    shield_hp_attr_id: int | None
    armor_hp_attr_id: int | None
    hull_hp_attr_id: int | None
    res_max_attr_id: int | None
    res_em_attr_id: int | None
    res_therm_attr_id: int | None
    res_kin_attr_id: int | None
    res_expl_attr_id: int | None
    res_shift_attr_id: int | None
    cycle_time_attr_id: int | None
    cycle_time_bonus_attr_id: int | None
    rah_effect_id: int
    heat_effect_id: int


def setup_rah_basics(
        *,
        client: TestClient,
        consts,  # noqa: ANN001
        datas: list[EveObjects] | type[Default] = Default,
        attr_shield_hp: type[Default] | None = Default,
        attr_armor_hp: type[Default] | None = Default,
        attr_hull_hp: type[Default] | None = Default,
        attr_res_em: type[Default] | None = Default,
        attr_res_therm: type[Default] | None = Default,
        attr_res_kin: type[Default] | None = Default,
        attr_res_expl: type[Default] | None = Default,
        attr_shift: type[Default] | None = Default,
        attr_cycle_time: int | type[Default] | None = Default,
) -> RahBasicInfo:
    # Attributes
    eve_shield_hp_attr_id = _make_optional_attr(
        client=client, datas=datas,
        id_=attr_armor_hp, default_id=consts.EveAttr.shield_capacity)
    eve_armor_hp_attr_id = _make_optional_attr(
        client=client, datas=datas,
        id_=attr_shield_hp, default_id=consts.EveAttr.armor_hp)
    eve_hull_hp_attr_id = _make_optional_attr(
        client=client, datas=datas,
        id_=attr_hull_hp, default_id=consts.EveAttr.hp)
    eve_res_max_attr_id = client.mk_eve_attr(datas=datas, id_=consts.EveAttr.armor_max_dmg_resonance, def_val=1)
    eve_res_em_attr_id = _make_optional_attr(
        client=client, datas=datas,
        id_=attr_res_em, default_id=consts.EveAttr.armor_em_dmg_resonance,
        stackable=False, max_attr_id=eve_res_max_attr_id)
    eve_res_therm_attr_id = _make_optional_attr(
        client=client, datas=datas,
        id_=attr_res_therm, default_id=consts.EveAttr.armor_therm_dmg_resonance,
        stackable=False, max_attr_id=eve_res_max_attr_id)
    eve_res_kin_attr_id = _make_optional_attr(
        client=client, datas=datas,
        id_=attr_res_kin, default_id=consts.EveAttr.armor_kin_dmg_resonance,
        stackable=False, max_attr_id=eve_res_max_attr_id)
    eve_res_expl_attr_id = _make_optional_attr(
        client=client, datas=datas,
        id_=attr_res_expl, default_id=consts.EveAttr.armor_expl_dmg_resonance,
        stackable=False, max_attr_id=eve_res_max_attr_id)
    eve_res_shift_attr_id = _make_optional_attr(
        client=client, datas=datas,
        id_=attr_shift, default_id=consts.EveAttr.resist_shift_amount, stackable=True)
    eve_cycle_time_attr_id = _make_optional_attr(
        client=client, datas=datas,
        id_=attr_cycle_time, default_id=consts.EveAttr.duration)
    eve_cycle_time_bonus_attr_id = client.mk_eve_attr(datas=datas, id_=consts.EveAttr.overload_self_duration_bonus)
    # Effects
    eve_rah_effect_id = client.mk_eve_effect(
        datas=datas,
        id_=consts.EveEffect.adaptive_armor_hardener,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_heat_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        loc=consts.EveModLoc.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_cycle_time_bonus_attr_id,
        affectee_attr_id=eve_cycle_time_attr_id)
    eve_heat_effect_id = client.mk_eve_effect(
        datas=datas,
        id_=consts.EveEffect.overload_self_duration_bonus,
        cat_id=consts.EveEffCat.overload,
        mod_info=[eve_heat_mod])
    return RahBasicInfo(
        shield_hp_attr_id=eve_shield_hp_attr_id,
        armor_hp_attr_id=eve_armor_hp_attr_id,
        hull_hp_attr_id=eve_hull_hp_attr_id,
        res_max_attr_id=eve_res_max_attr_id,
        res_em_attr_id=eve_res_em_attr_id,
        res_therm_attr_id=eve_res_therm_attr_id,
        res_kin_attr_id=eve_res_kin_attr_id,
        res_expl_attr_id=eve_res_expl_attr_id,
        cycle_time_attr_id=eve_cycle_time_attr_id,
        cycle_time_bonus_attr_id=eve_cycle_time_bonus_attr_id,
        res_shift_attr_id=eve_res_shift_attr_id,
        rah_effect_id=eve_rah_effect_id,
        heat_effect_id=eve_heat_effect_id)


def make_eve_rah(
        *,
        client: TestClient,
        datas: list[EveObjects] | type[Default] = Default,
        basic_info: RahBasicInfo,
        id_: int | type[Default] = Default,
        grp_id: int | type[Default] = Default,
        resos: tuple[float | None, float | None, float | None, float | None],
        shift_amount: float | None = 6,
        cycle_time: float | None = 10000,
        heat_cycle_mod: float | None = -15,
) -> int:
    return client.mk_eve_item(
        datas=datas,
        id_=id_,
        grp_id=grp_id,
        attrs={
            k: v for k, v in zip(
                (basic_info.res_em_attr_id,
                 basic_info.res_therm_attr_id,
                 basic_info.res_kin_attr_id,
                 basic_info.res_expl_attr_id,
                 basic_info.res_shift_attr_id,
                 basic_info.cycle_time_attr_id,
                 basic_info.cycle_time_bonus_attr_id),
                (*resos,
                 shift_amount,
                 cycle_time,
                 heat_cycle_mod),
                strict=True)
            if k is not None and v is not None},
        eff_ids=[basic_info.rah_effect_id, basic_info.heat_effect_id],
        defeff_id=basic_info.rah_effect_id)


def make_eve_ship(
        *,
        client: TestClient,
        datas: list[EveObjects] | type[Default] = Default,
        basic_info: RahBasicInfo,
        id_: int | type[Default] = Default,
        resos: tuple[float | None, float | None, float | None, float | None],
        hps: tuple[float | None, float | None, float | None] | type[Default] = Default,
) -> int:
    if hps is Default:
        hps = (0, 0, 0)
    return client.mk_eve_ship(datas=datas, id_=id_, attrs={
        k: v for k, v in chain(
            zip(
                (basic_info.res_em_attr_id,
                 basic_info.res_therm_attr_id,
                 basic_info.res_kin_attr_id,
                 basic_info.res_expl_attr_id),
                resos,
                strict=True),
            zip(
                (basic_info.shield_hp_attr_id,
                 basic_info.armor_hp_attr_id,
                 basic_info.hull_hp_attr_id),
                hps,
                strict=True))
        if k is not None and v is not None})


def _make_optional_attr(
        *,
        client: TestClient,
        datas: list[EveObjects] | type[Default] = Default,
        id_: int | type[Default] | None,
        default_id: int,
        stackable: bool | type[Default] = Default,
        max_attr_id: int | type[Default] = Default,
) -> int | None:
    if id_ is None:
        return None
    if id_ is Default:
        id_ = default_id
    return client.mk_eve_attr(datas=datas, id_=id_, stackable=stackable, max_attr_id=max_attr_id)
