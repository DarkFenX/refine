use crate::{ad, defs::EBuffId, ec};

const BUFF_EFFECT: EBuffId = ec::effects::DOOMSDAY_AOE_WEB;

pub(in crate::adg::custom) fn add_buff_info(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == BUFF_EFFECT) {
        effect.buff = Some(ad::AEffectBuffInfo::new(
            ad::AEffectBuffDataSrc::HardcodedId(ec::buffs::STASIS_WEBIFICATION_BURST, ec::attrs::SPEED_FACTOR),
            ad::AEffectBuffScope::Everything,
        ));
        applied = true;
    }
    if !applied {
        tracing::info!("web burst effect {BUFF_EFFECT} is not found for customization");
    }
}
