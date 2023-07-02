use crate::{
    ad,
    consts::{attrs, effects, ModAfeeFilter, ModBuildStatus, ModDomain, ModOp, ModSrq},
    shr::State,
};

pub(in crate::adg) fn customize(a_data: &mut ad::AData) {
    fix_online_effect_cat(a_data);
    mk_self_skillreq_modifiers_launcher_rof(a_data);
}

fn fix_online_effect_cat(a_data: &mut ad::AData) {
    let mut fixed = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == effects::ONLINE) {
        if effect.state == State::Active {
            effect.state = State::Online;
            fixed = true;
        }
    }
    if !fixed {
        tracing::info!("\"online\" effect category did not need fixing")
    }
}

fn mk_self_skillreq_modifiers_launcher_rof(a_data: &mut ad::AData) {
    for effect in a_data.effects.iter_mut().filter(|v| v.id == effects::SELF_ROF) {
        if !effect.mods.is_empty() {
            tracing::info!("self-skillreq missile rof effect has modifiers, overwriting them");
            effect.mods.clear();
        }
        let modifier = ad::AAttrMod::new(
            attrs::ROF_BONUS,
            ModOp::PostPerc,
            ModAfeeFilter::LocSrq(ModDomain::Ship, ModSrq::SelfRef),
            attrs::SPEED,
        );
        effect.mods.push(modifier);
        effect.mod_build_status = ModBuildStatus::Custom;
    }
}
