use crate::{
    ac, ad,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
    },
    util::RMap,
};

const WEB_BUBBLE: ad::AItemId = ac::items::STASIS_WEBIFICATION_PROBE;
const A_EFFECT_ID: ad::AEffectId = ac::effects::STASIS_WEB_PROBE;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            get_proj_mult: Some(get_proj_mult_simple_s2s),
            ..
        },
        ..
    }
}

fn make_effect() -> ad::AEffect {
    ad::AEffect {
        id: A_EFFECT_ID,
        category: ac::effcats::ACTIVE,
        state: ad::AState::Active,
        is_offense: true,
        range_attr_id: Some(ac::attrs::DOOMSDAY_AOE_RANGE),
        buff: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::Customized(vec![ad::AEffectBuffSrcCustom::AffectorVal(
                ac::buffs::STASIS_WEBIFICATION_BURST,
                ac::attrs::SPEED_FACTOR,
            )]),
            scope: ad::AEffectBuffScope::Everything,
        }),
        ..
    }
}

fn assign_effect(a_items: &mut RMap<ad::AItemId, ad::AItem>) -> bool {
    match a_items.get_mut(&WEB_BUBBLE) {
        Some(a_item) => {
            a_item.effect_datas.insert(A_EFFECT_ID, ad::AItemEffectData::default());
            a_item.defeff_id = Some(A_EFFECT_ID);
            true
        }
        None => false,
    }
}
