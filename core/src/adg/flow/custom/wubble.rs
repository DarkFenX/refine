use crate::{ac, ad};

const WEB_BUBBLE: ad::AItemId = ac::items::STASIS_WEBIFICATION_PROBE;

pub(in crate::adg::flow::custom) fn add_wubble_effect(a_data: &mut ad::AData) {
    let effect = ad::AEffect {
        id: ac::effects::REE_STASIS_WEB_PROBE,
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
            a_item.effect_datas.insert(
                effect_id,
                ad::AItemEffectData {
                    cd: None,
                    charge_count: None,
                    charge_reload_time: None,
                },
            );
            a_item.defeff_id = Some(effect_id);
        }
        None => tracing::info!("web bubble {WEB_BUBBLE} is not found for customization"),
    }
}
