use crate::{ad, ec};

pub(in crate::adg::custom) fn add_aar_paste_boost_effect(a_data: &mut ad::AData) {
    let effect = ad::AEffect::new(
        ec::effects::REE_AAR_PASTE_BOOST,
        ec::effcats::PASSIVE,
        ad::AState::Offline,
        false,
        false,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        ad::AEffectModBuildStatus::Custom,
        // No modifiers, a custom one is added
        Vec::new(),
        Vec::new(),
        None,
        None,
    );
    let effect_id = effect.id;
    a_data.effects.push(effect);
    for item in a_data.items.iter_mut().filter(|v| {
        v.effect_datas.contains_key(&ec::effects::FUELED_ARMOR_REPAIR)
            || v.effect_datas.contains_key(&ec::effects::SHIP_MODULE_ARAR)
    }) {
        item.effect_datas
            .insert(effect_id, ad::AItemEffectData::new(None, None, None));
    }
}
