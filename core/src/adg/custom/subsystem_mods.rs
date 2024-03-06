use crate::{
    ad,
    defs::{EAttrId, EEffectId},
    ec,
    shr::{ModDomain, ModOp},
};

const SLOT_EFFECT: EEffectId = ec::effects::SLOT_MODIFIER;
const HARDPOINT_EFFECT: EEffectId = ec::effects::HARDPOINT_MODIFIER_EFFECT;

pub(in crate::adg::custom) fn add_subsystem_modifiers(a_data: &mut ad::AData) {
    add_slot_modifiers(a_data);
    add_hardpoint_modifiers(a_data);
}

fn add_slot_modifiers(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == SLOT_EFFECT) {
        if !effect.mods.is_empty() {
            tracing::info!("slot modifier effect {SLOT_EFFECT} has modifiers, overwriting them");
            effect.mods.clear();
        }
        effect
            .mods
            .push(mk_modifier(ec::attrs::HI_SLOT_MODIFIER, ec::attrs::HI_SLOTS));
        effect
            .mods
            .push(mk_modifier(ec::attrs::MED_SLOT_MODIFIER, ec::attrs::MED_SLOTS));
        effect
            .mods
            .push(mk_modifier(ec::attrs::LOW_SLOT_MODIFIER, ec::attrs::LOW_SLOTS));
        effect.mod_build_status = ad::AModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("slot modifier effect {SLOT_EFFECT} isn't found for customization");
    }
}

fn add_hardpoint_modifiers(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == HARDPOINT_EFFECT) {
        if !effect.mods.is_empty() {
            tracing::info!("hardpoint modifier effect {HARDPOINT_EFFECT} has modifiers, overwriting them");
            effect.mods.clear();
        }
        effect.mods.push(mk_modifier(
            ec::attrs::TURRET_HARDPOINT_MODIFIER,
            ec::attrs::TURRET_SLOTS_LEFT,
        ));
        effect.mods.push(mk_modifier(
            ec::attrs::LAUNCHER_HARDPOINT_MODIFIER,
            ec::attrs::LAUNCHER_SLOTS_LEFT,
        ));
        effect.mod_build_status = ad::AModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("hardpoint modifier effect {HARDPOINT_EFFECT} isn't found for customization");
    }
}

fn mk_modifier(src_attr_id: EAttrId, tgt_attr_id: EAttrId) -> ad::AEffectAttrMod {
    ad::AEffectAttrMod::new(
        src_attr_id,
        ModOp::Add,
        ad::AModTgtFilter::Direct(ModDomain::Ship),
        tgt_attr_id,
    )
}
