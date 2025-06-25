use crate::{ac, ad, ntt::NttEffect};

const WEB_BUBBLE: ad::AItemId = ac::items::STASIS_WEBIFICATION_PROBE;
const A_EFFECT_ID: ad::AEffectId = ac::effects::STASIS_WEB_PROBE;

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: None,
        aid: A_EFFECT_ID,
        custom_fn_adg: Some(add_custom_effect),
        ..
    }
}

fn add_custom_effect(a_data: &mut ad::AData) {
    let effect = ad::AEffect {
        id: A_EFFECT_ID,
        category: ac::effcats::ACTIVE,
        state: ad::AState::Active,
        is_offense: true,
        range_attr_id: Some(ac::attrs::DOOMSDAY_AOE_RANGE),
        mod_build_status: ad::AEffectModBuildStatus::Custom,
        buff: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::Customized(vec![ad::AEffectBuffSrcCustom::AffectorVal(
                ac::buffs::STASIS_WEBIFICATION_BURST,
                ac::attrs::SPEED_FACTOR,
            )]),
            scope: ad::AEffectBuffScope::Everything,
        }),
        ..
    };
    let effect_id = effect.id;
    a_data.effects.insert(effect.id, effect);
    match a_data.items.get_mut(&WEB_BUBBLE) {
        Some(a_item) => {
            a_item.effect_datas.insert(effect_id, ad::AItemEffectData::default());
            a_item.defeff_id = Some(effect_id);
        }
        None => tracing::info!("web bubble {WEB_BUBBLE} is not found for customization"),
    }
}
