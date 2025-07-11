use crate::{ac, ad, def::OF, nd::NEffect};

const A_ITEM_ID: ad::AItemId = ac::items::EXOTIC_STABILITY_GENERATOR;
const A_EFFECT_ID: ad::AEffectId = ac::effects::STABILITY_GENERATOR_EXOTIC;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_custom_fn: Some(add_custom_effect),
        ..
    }
}

fn add_custom_effect(a_data: &mut ad::AData) {
    let effect = ad::AEffect {
        id: A_EFFECT_ID,
        category: ac::effcats::PASSIVE,
        state: ad::AState::Offline,
        mod_build_status: ad::AEffectModBuildStatus::Custom,
        buff: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::Customized(vec![
                ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::ADDITIONAL_BASE_WARP_SPEED, OF(2.0)),
                ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::SCAN_RESOLUTION_BONUS, OF(25.0)),
            ]),
            scope: ad::AEffectBuffScope::Ships,
        }),
        ..
    };
    let effect_id = effect.id;
    a_data.effects.insert(effect.id, effect);
    match a_data.items.get_mut(&A_ITEM_ID) {
        Some(a_item) => {
            a_item.effect_datas.insert(effect_id, ad::AItemEffectData::default());
            a_item.defeff_id = Some(effect_id);
        }
        None => tracing::info!("item {A_ITEM_ID}: exotic stability generator is not found for customization"),
    }
}
