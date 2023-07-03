use crate::{ad, ec, shr::ModOp};

pub(in crate::adg::custom) fn mk_self_skillreq_drone_dmg(a_data: &mut ad::AData) {
    for effect in a_data
        .effects
        .iter_mut()
        .filter(|v| v.id == ec::effects::DRONE_DMG_BONUS)
    {
        if !effect.mods.is_empty() {
            tracing::info!("self-skillreq drone dmg effect has modifiers, overwriting them");
            effect.mods.clear();
        }
        let modifier = ad::AEffectAttrMod::new(
            ec::attrs::DMG_MULT_BONUS,
            ModOp::PostPerc,
            ad::AModTgtFilter::OwnSrq(ad::AModSrq::SelfRef),
            ec::attrs::DMG_MULT,
        );
        effect.mods.push(modifier);
        effect.mod_build_status = ad::AModBuildStatus::Custom;
    }
}
