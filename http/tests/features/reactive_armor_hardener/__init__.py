from dataclasses import dataclass
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


def setup_rah_basics(*, client, consts, datas=Default) -> RahBasicInfo:
    eve_res_max_attr_id = client.mk_eve_attr(
        datas=datas,
        id_=consts.EveAttr.armor_max_dmg_resonance,
        def_val=1)
    eve_res_em_attr_id = client.mk_eve_attr(
        datas=datas,
        id_=consts.EveAttr.armor_em_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_therm_attr_id = client.mk_eve_attr(
        datas=datas,
        id_=consts.EveAttr.armor_therm_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_kin_attr_id = client.mk_eve_attr(
        datas=datas,
        id_=consts.EveAttr.armor_kin_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_res_expl_attr_id = client.mk_eve_attr(
        datas=datas,
        id_=consts.EveAttr.armor_expl_dmg_resonance,
        stackable=False,
        max_attr_id=eve_res_max_attr_id)
    eve_cycle_time_attr_id = client.mk_eve_attr(datas=datas, id_=consts.EveAttr.duration)
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
        resos,
        shift_amount,
        cycle_time=10000,
        heat_cycle_mod=-15):
    eve_rah_id = client.mk_eve_item(
        datas=datas,
        attrs=dict(zip(
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
             heat_cycle_mod))),
        eff_ids=[basic_info.rah_effect_id, basic_info.heat_effect_id],
        defeff_id=basic_info.rah_effect_id)
    return eve_rah_id


def make_eve_ship(
        *,
        client,
        datas=Default,
        basic_info: RahBasicInfo,
        resos):
    eve_ship_id = client.mk_eve_ship(datas=datas, attrs=dict(zip(
        (basic_info.res_em_attr_id,
         basic_info.res_therm_attr_id,
         basic_info.res_kin_attr_id,
         basic_info.res_expl_attr_id),
        resos)))
    return eve_ship_id
