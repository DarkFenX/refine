use crate::{ac, ad, ntt::NttEffect};

const A_EFFECT_ID: ad::AEffectId = ac::effects::MISSILE_FLIGHT_TIME;

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
        category: ac::effcats::PASSIVE,
        state: ad::AState::Offline,
        mod_build_status: ad::AEffectModBuildStatus::Custom,
        ..
    };
    let effect_id = effect.id;
    a_data.effects.insert(effect.id, effect);
    for item in a_data.items.values_mut().filter(|v| {
        v.effect_datas.contains_key(&ac::effects::MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::DEFENDER_MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::FOF_MISSILE_LAUNCHING)
            || v.effect_datas.contains_key(&ac::effects::DOT_MISSILE_LAUNCHING)
    }) {
        item.effect_datas.insert(effect_id, ad::AItemEffectData::default());
    }
}
