use crate::{ac, ad};

pub(in crate::adg::flow::custom) fn add_aar_paste_boost_effect(a_data: &mut ad::AData) {
    let effect = ad::AEffect {
        id: ac::effects::REE_AAR_PASTE_BOOST,
        category: ac::effcats::PASSIVE,
        state: ad::AState::Offline,
        is_assist: false,
        is_offense: false,
        hisec: None,
        lowsec: None,
        discharge_attr_id: None,
        duration_attr_id: None,
        range_attr_id: None,
        falloff_attr_id: None,
        track_attr_id: None,
        chance_attr_id: None,
        resist_attr_id: None,
        mod_build_status: ad::AEffectModBuildStatus::Custom,
        // No modifiers, a custom one is added
        mods: Vec::new(),
        stop_ids: Vec::new(),
        buff: None,
        charge: None,
    };
    let effect_id = effect.id;
    a_data.effects.push(effect);
    for item in a_data.items.iter_mut().filter(|v| {
        v.effect_datas.contains_key(&ac::effects::FUELED_ARMOR_REPAIR)
            || v.effect_datas.contains_key(&ac::effects::SHIP_MODULE_ARAR)
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
