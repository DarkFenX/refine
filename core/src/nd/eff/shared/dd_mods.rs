use crate::{
    ac,
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp},
};

pub(in crate::nd::eff) fn update_effect(a_effect_id: AEffectId, a_effect: &mut AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {a_effect_id}: doomsday effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.extend([
        make_ship_mod(ac::attrs::SPEED_FACTOR, AOp::PostPerc, ac::attrs::MAX_VELOCITY),
        make_ship_mod(
            ac::attrs::SIEGE_MODE_WARP_STATUS,
            AOp::Add,
            ac::attrs::WARP_SCRAMBLE_STATUS,
        ),
        make_ship_mod(ac::attrs::CAN_CLOAK, AOp::PostAssign, ac::attrs::CAN_CLOAK),
        make_ship_mod(ac::attrs::DISALLOW_TETHERING, AOp::Add, ac::attrs::DISALLOW_TETHERING),
        make_ship_mod(ac::attrs::DISALLOW_DOCKING, AOp::Add, ac::attrs::DISALLOW_DOCKING),
    ]);
}

fn make_ship_mod(affector_attr_id: AAttrId, op: AOp, affectee_attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id,
        op,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id,
    }
}
