use crate::{
    ac,
    ad::{
        AEffect, AEffectBuffInfo, AEffectBuffScope, AEffectBuffSrc, AEffectBuffSrcCustom, AEffectId, AItem,
        AItemEffectData, AItemId, AState,
    },
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_c2s},
    },
    util::RMap,
};

const WEB_BUBBLE: AItemId = ac::items::STASIS_WEBIFICATION_PROBE;
const A_EFFECT_ID: AEffectId = ac::effects::STASIS_WEB_PROBE;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        modifier_proj_attrs_getter: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_proj_mult_simple_c2s),
            ..
        },
        ..
    }
}

fn make_effect() -> AEffect {
    AEffect {
        id: A_EFFECT_ID,
        category: ac::effcats::ACTIVE,
        state: AState::Active,
        is_offense: true,
        range_attr_id: Some(ac::attrs::DOOMSDAY_AOE_RANGE),
        buff_info: Some(AEffectBuffInfo {
            source: AEffectBuffSrc::Customized(vec![AEffectBuffSrcCustom::AffectorVal(
                ac::buffs::STASIS_WEBIFICATION_BURST,
                ac::attrs::SPEED_FACTOR,
            )]),
            scope: AEffectBuffScope::Everything,
        }),
        ..
    }
}

fn assign_effect(a_items: &mut RMap<AItemId, AItem>) -> bool {
    match a_items.get_mut(&WEB_BUBBLE) {
        Some(a_item) => {
            a_item.effect_datas.insert(A_EFFECT_ID, AItemEffectData::default());
            a_item.defeff_id = Some(A_EFFECT_ID);
            true
        }
        None => false,
    }
}
