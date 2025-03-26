use crate::{ac, ad};

const WEB_BUBBLE: ad::AItemId = ac::items::STASIS_WEBIFICATION_PROBE;

pub(in crate::adg::flow::custom) fn add_wubble_effect(a_data: &mut ad::AData) {
    let effect = ad::AEffect {
        id: ac::effects::REE_STASIS_WEB_PROBE,
        category: ac::effcats::ACTIVE,
        state: ad::AState::Active,
        is_assist: false,
        is_offense: true,
        hisec: None,
        lowsec: None,
        discharge_attr_id: None,
        duration_attr_id: None,
        range_attr_id: Some(ac::attrs::DOOMSDAY_AOE_RANGE),
        falloff_attr_id: None,
        track_attr_id: None,
        chance_attr_id: None,
        resist_attr_id: None,
        mod_build_status: ad::AEffectModBuildStatus::Custom,
        mods: Vec::new(),
        stop_ids: Vec::new(),
        buff: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::Customized(vec![ad::AEffectBuffSrcCustom::AffectorVal(
                ac::buffs::STASIS_WEBIFICATION_BURST,
                ac::attrs::SPEED_FACTOR,
            )]),
            scope: ad::AEffectBuffScope::Everything,
        }),
        charge: None,
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
