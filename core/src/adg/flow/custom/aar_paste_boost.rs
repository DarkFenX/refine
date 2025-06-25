use crate::{ac, ad};

pub(in crate::adg::flow::custom) fn add_aar_paste_boost_effect(a_data: &mut ad::AData) {
    let effect = ad::AEffect {
        id: ac::effects::REE_AAR_PASTE_BOOST,
        category: ac::effcats::PASSIVE,
        state: ad::AState::Offline,
        mod_build_status: ad::AEffectModBuildStatus::Custom,
        ..
    };
    let effect_id = effect.id;
    a_data.effects.insert(effect.id, effect);
    for item in a_data.items.values_mut().filter(|v| {
        v.effect_datas.contains_key(&ac::effects::FUELED_ARMOR_REPAIR)
            || v.effect_datas.contains_key(&ac::effects::SHIP_MODULE_RAAR)
    }) {
        item.effect_datas.insert(
            effect_id,
            ad::AItemEffectData {
                cd: None,
                charge_count: None,
                charge_reload_time: None,
            },
        );
    }
}
