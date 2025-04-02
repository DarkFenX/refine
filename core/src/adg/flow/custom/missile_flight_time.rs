use crate::{ac, ad};

pub(in crate::adg::flow::custom) fn add_missile_flight_time(a_data: &mut ad::AData) {
    let effect = ad::AEffect {
        id: ac::effects::REE_MISSILE_FLIGHT_TIME,
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
    a_data.effects.insert(effect.id, effect);
    for item in a_data.items.values_mut().filter(|v| {
        v.effect_datas.contains_key(&ac::effects::MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::DEFENDER_MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::FOF_MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::DOT_MISSILE_LAUNCHING)
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
