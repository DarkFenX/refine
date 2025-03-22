use crate::{ac, ad};

const SLOT_EFFECT: ad::AEffectId = ac::effects::SLOT_MODIFIER;
const HARDPOINT_EFFECT: ad::AEffectId = ac::effects::HARDPOINT_MODIFIER_EFFECT;

pub(in crate::adg::flow::custom) fn add_subsystem_modifiers(a_data: &mut ad::AData) {
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
            .push(mk_modifier(ac::attrs::HI_SLOT_MODIFIER, ac::attrs::HI_SLOTS));
        effect
            .mods
            .push(mk_modifier(ac::attrs::MED_SLOT_MODIFIER, ac::attrs::MED_SLOTS));
        effect
            .mods
            .push(mk_modifier(ac::attrs::LOW_SLOT_MODIFIER, ac::attrs::LOW_SLOTS));
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("slot modifier effect {SLOT_EFFECT} is not found for customization");
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
            ac::attrs::TURRET_HARDPOINT_MODIFIER,
            ac::attrs::TURRET_SLOTS_LEFT,
        ));
        effect.mods.push(mk_modifier(
            ac::attrs::LAUNCHER_HARDPOINT_MODIFIER,
            ac::attrs::LAUNCHER_SLOTS_LEFT,
        ));
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("hardpoint modifier effect {HARDPOINT_EFFECT} is not found for customization");
    }
}

fn mk_modifier(src_attr_id: ad::AAttrId, affectee_attr_id: ad::AAttrId) -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id: src_attr_id,
        op: ad::AOp::Add,
        affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Ship),
        affectee_attr_id,
    }
}
