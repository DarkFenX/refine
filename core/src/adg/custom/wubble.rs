use crate::{ad, ec};

pub(in crate::adg::custom) fn add_wubble_effect(a_data: &mut ad::AData) {
    let effect = ad::AEffect::new(
        ec::effects::REE_STASIS_WEB_PROBE,
        ec::effcats::ACTIVE,
        ad::AState::Active,
        false,
        true,
        None,
        None,
        Some(ec::attrs::DOOMSDAY_AOE_RANGE),
        None,
        None,
        None,
        None,
        None,
        None,
        ad::AEffectModBuildStatus::Custom,
        Vec::new(),
        Vec::new(),
        Some(ad::AEffectBuffInfo::new(
            ad::AEffectBuffSrc::Customized(vec![ad::AEffectBuffSrcCustom::AffectorVal(
                ec::buffs::STASIS_WEBIFICATION_BURST,
                ec::attrs::SPEED_FACTOR,
            )]),
            ad::AEffectBuffScope::Everything,
        )),
        None,
    );
    let effect_id = effect.id;
    a_data.effects.push(effect);
    for item in a_data.items.iter_mut().filter(|v| v.id == effect_id) {
        item.effect_datas
            .insert(effect_id, ad::AItemEffectData::new(None, None, None));
        item.defeff_id = Some(effect_id);
    }
}
