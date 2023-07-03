use crate::{
    ad, ec,
    shr::{ModDomain, ModOp},
};

pub(in crate::adg::custom) fn mk_self_skillreq_modifiers_launcher_rof(a_data: &mut ad::AData) {
    for effect in a_data.effects.iter_mut().filter(|v| v.id == ec::effects::SELF_ROF) {
        if !effect.mods.is_empty() {
            tracing::info!("self-skillreq missile rof effect has modifiers, overwriting them");
            effect.mods.clear();
        }
        let modifier = ad::AEffectAttrMod::new(
            ec::attrs::ROF_BONUS,
            ModOp::PostPerc,
            ad::AModTgtFilter::LocSrq(ModDomain::Ship, ad::AModSrq::SelfRef),
            ec::attrs::SPEED,
        );
        effect.mods.push(modifier);
        effect.mod_build_status = ad::AModBuildStatus::Custom;
    }
}
