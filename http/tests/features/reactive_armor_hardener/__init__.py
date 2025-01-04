from dataclasses import dataclass
from typing import Union

from tests.support.util import Default


@dataclass(kw_only=True)
class RahBasicInfo:
    res_max_attr_id: int
    res_em_attr_id: int
    res_therm_attr_id: int
    res_kin_attr_id: int
    res_expl_attr_id: int
    cycle_time_attr_id: int
    cycle_time_bonus_attr_id: int
    res_shift_attr_id: int
    rah_effect_id: int
    heat_effect_id: int


def setup_rah_basics(
        *,
        client,
        consts,
        datas=Default,
        attr_res_em: Union[int, None, type[Default]] = Default,
        attr_res_therm: Union[int, None, type[Default]] = Default,
        attr_res_kin: Union[int, None, type[Default]] = Default,
        attr_res_expl: Union[int, None, type[Default]] = Default,
        attr_cycle_time: Union[int, type[Default]] = Default,
) -> RahBasicInfo:
    eve_res_max_attr_id = client.mk_eve_attr(
        datas=datas,
        id_=consts.EveAttr.armor_max_dmg_resonance,
        def_val=1)
    if attr_res_em is None:
        eve_res_em_attr_id = None
    else:
        eve_res_em_attr_id = client.mk_eve_attr(
            datas=datas,
            id_=consts.EveAttr.armor_em_dmg_resonance if attr_res_em is Default else attr_res_em,
            stackable=False,
            max_attr_id=eve_res_max_attr_id)
    if attr_res_therm is None:
        eve_res_therm_attr_id = None
    else:
        eve_res_therm_attr_id = client.mk_eve_attr(
            datas=datas,
            id_=consts.EveAttr.armor_therm_dmg_resonance if attr_res_therm is Default else attr_res_therm,
            stackable=False,
            max_attr_id=eve_res_max_attr_id)
    if attr_res_kin is None:
        eve_res_kin_attr_id = None
    else:
        eve_res_kin_attr_id = client.mk_eve_attr(
            datas=datas,
            id_=consts.EveAttr.armor_kin_dmg_resonance if attr_res_kin is Default else attr_res_kin,
            stackable=False,
            max_attr_id=eve_res_max_attr_id)
    if attr_res_expl is None:
        eve_res_expl_attr_id = None
    else:
        eve_res_expl_attr_id = client.mk_eve_attr(
            datas=datas,
            id_=consts.EveAttr.armor_expl_dmg_resonance if attr_res_expl is Default else attr_res_expl,
            stackable=False,
            max_attr_id=eve_res_max_attr_id)
    eve_cycle_time_attr_id = client.mk_eve_attr(
        datas=datas,
        id_=consts.EveAttr.duration if attr_cycle_time is Default else attr_cycle_time)
    eve_cycle_time_bonus_attr_id = client.mk_eve_attr(datas=datas, id_=consts.EveAttr.overload_self_duration_bonus)
    eve_res_shift_attr_id = client.mk_eve_attr(datas=datas, id_=consts.EveAttr.resist_shift_amount)
    eve_rah_effect_id = client.mk_eve_effect(
        datas=datas,
        id_=consts.EveEffect.adaptive_armor_hardener,
        cat_id=consts.EveEffCat.active,
        duration_attr_id=eve_cycle_time_attr_id)
    eve_heat_mod = client.mk_eve_effect_mod(
        func=consts.EveModFunc.item,
        dom=consts.EveModDom.item,
        op=consts.EveModOp.post_percent,
        affector_attr_id=eve_cycle_time_bonus_attr_id,
        affectee_attr_id=eve_cycle_time_attr_id)
    eve_heat_effect_id = client.mk_eve_effect(
        datas=datas,
        id_=consts.EveEffect.overload_self_duration_bonus,
        cat_id=consts.EveEffCat.overload,
        mod_info=[eve_heat_mod])
    return RahBasicInfo(
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
        client,
        datas=Default,
        basic_info: RahBasicInfo,
        id_: Union[int, type[Default]] = Default,
        grp_id: Union[int, type[Default]] = Default,
        resos: tuple[float, float, float, float],
        shift_amount: float,
        cycle_time: float = 10000,
        heat_cycle_mod: float = -15,
):
    eve_rah_id = client.mk_eve_item(
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
                 heat_cycle_mod))
            if k is not None},
        eff_ids=[basic_info.rah_effect_id, basic_info.heat_effect_id],
        defeff_id=basic_info.rah_effect_id)
    return eve_rah_id


def make_eve_ship(
        *,
        client,
        datas=Default,
        basic_info: RahBasicInfo,
        id_: Union[int, type[Default]] = Default,
        resos: tuple[float, float, float, float]):
    eve_ship_id = client.mk_eve_ship(datas=datas, id_=id_, attrs={
        k: v  for k, v in zip(
            (basic_info.res_em_attr_id,
             basic_info.res_therm_attr_id,
             basic_info.res_kin_attr_id,
             basic_info.res_expl_attr_id),
            resos)
        if k is not None})
    return eve_ship_id
