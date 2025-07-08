// WDFG effect seems to have lots of special handling in EVE. In the library, effects are adjusted
// to work within regular dogma framework. It includes:
// - blocking MJD with both focused scripts. According to CCP Kestrel, scripts decide that they
// block MJD based on its graphical effect, not based on regular dogma group ID or skill
// requirement. In the library, it's changed to use conventional attributes;
// - attributes used by focused scripts are defined on parent item, not on script itself, while
// dogma (at least in the lib's implementation) assumes that source attributes are always defined
// on item which carries the effect;
// - range used by focused scripts uses maxRange attribute which is defined on parent item; unlike
// other attributes, it's transferred over by an existing WDFG effect, into maxRangeHidden
// attribute. Here, we switch scripts to use this attribute, instead of transferring maxRange as
// well (although transferring would also work).
// Script effects are defined in other files.

use crate::{
    ac, ad, ec, ed,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectHc},
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::WARP_DISRUPT_SPHERE;
const A_EFFECT_ID: ad::AEffectId = ac::effects::WARP_DISRUPT_SPHERE;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_custom_fn: Some(update_effect),
        hc: NEffectHc {
            // TODO: add comment that charge needs to be specified to run on-charge effects
            charge: Some(NEffectCharge::Loaded(NEffectChargeDepl::None)),
            ..
        },
        ..
    }
}

fn update_effect(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&A_EFFECT_ID) {
        Some(effect) => {
            if !effect.mods.is_empty() {
                tracing::info!("effect {A_EFFECT_ID}: WDFG bubble effect has modifiers, overwriting them");
                effect.mods.clear();
            }
            // Signature radius
            effect.mods.push(ad::AEffectModifier {
                affector_attr_id: ac::attrs::SIG_RADIUS_BONUS,
                op: ad::AOp::PostPerc,
                affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Ship),
                affectee_attr_id: ac::attrs::SIG_RADIUS,
            });
            // Disallow assistance
            effect.mods.push(ad::AEffectModifier {
                affector_attr_id: ac::attrs::DISALLOW_ASSISTANCE,
                op: ad::AOp::PostAssign,
                affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Ship),
                affectee_attr_id: ac::attrs::DISALLOW_ASSISTANCE,
            });
            // Transfer warp core scram strength to script
            effect.mods.push(ad::AEffectModifier {
                affector_attr_id: ac::attrs::WARP_SCRAMBLE_STRENGTH,
                op: ad::AOp::PreAssign,
                affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Other),
                affectee_attr_id: ac::attrs::WARP_SCRAMBLE_STRENGTH,
            });
            // Transfer activation block strength to script
            effect.mods.push(ad::AEffectModifier {
                affector_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
                op: ad::AOp::PreAssign,
                affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Other),
                affectee_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
            });
            effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        }
        None => tracing::info!("effect {A_EFFECT_ID}: WDFG bubble effect is not found for customization"),
    }
}
