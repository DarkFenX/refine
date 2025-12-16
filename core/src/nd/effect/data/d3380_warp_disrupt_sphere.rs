// WDFG effect seems to have lots of special handling in EVE. In the library, a bunch of tools
// specific to the library are used to make it work in a similar way. It includes:
// - in EVE, upon warping/jumping ships seem to check if they are in a bubble. Here, it is handled
//   via custom/hardcoded attribute, which is modified via custom/hardcoded buff, which affects
//   carrying ship, as well as targets it is projected upon. When a WDFG script is loaded, it does
//   not remove the buff, but makes its strength 0;
// - blocking MJD with both focused scripts. According to CCP Kestrel, scripts decide that they
//   block MJD based on its graphical effect, not based on regular dogma group ID or skill
//   requirement. In the library, it's changed to use conventional attributes;
// - attributes used by focused scripts are defined on parent item, not on script itself, while
//   dogma (at least in the lib's implementation) assumes that source attributes are always defined
//   on item which carries the effect;
// - range used by focused scripts uses maxRange attribute which is defined on parent item; unlike
//   other attributes, it's transferred over by an existing WDFG effect, into maxRangeHidden
//   attribute. Here, we switch scripts to use this attribute, instead of transferring maxRange as
//   well (although transferring would also work).
// Script effects are defined in other files.

use crate::{
    ac,
    ad::{
        AEffect, AEffectAffecteeFilter, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope,
        AEffectBuffStrength, AEffectId, AEffectLocation, AEffectModifier, AOp,
    },
    ec,
    ed::EEffectId,
    nd::{NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc},
};

const E_EFFECT_ID: EEffectId = ec::effects::WARP_DISRUPT_SPHERE;
const A_EFFECT_ID: AEffectId = ac::effects::WARP_DISRUPT_SPHERE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff: Some(AEffectBuff {
            full: vec![
                // Disallows warping and jumping for everything in range, including self
                AEffectBuffFull {
                    buff_id: ac::buffs::DISALLOW_WARP_JUMP,
                    strength: AEffectBuffStrength::Attr(ac::attrs::DISALLOW_WARPING_JUMPING),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ac::buffs::DISALLOW_WARP_JUMP,
                    strength: AEffectBuffStrength::Attr(ac::attrs::DISALLOW_WARPING_JUMPING),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Carrier,
                },
            ],
            ..
        }),
        adg_update_effect_fn: Some(update_effect),
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::None),
            activates_charge: true,
        }),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {A_EFFECT_ID}: WDFG bubble effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        // Signature radius
        AEffectModifier {
            affector_attr_id: ac::attrs::SIG_RADIUS_BONUS,
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: ac::attrs::SIG_RADIUS,
        },
        // Disallow assistance
        AEffectModifier {
            affector_attr_id: ac::attrs::DISALLOW_ASSISTANCE,
            op: AOp::PostAssign,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: ac::attrs::DISALLOW_ASSISTANCE,
        },
        // Transfer warp core scram strength to script
        AEffectModifier {
            affector_attr_id: ac::attrs::WARP_SCRAMBLE_STRENGTH,
            op: AOp::PreAssign,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Other),
            affectee_attr_id: ac::attrs::WARP_SCRAMBLE_STRENGTH,
        },
        // Transfer activation block strength to script
        AEffectModifier {
            affector_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
            op: AOp::PreAssign,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Other),
            affectee_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
        },
        // Modifiers which have been "disabled" by setting appropriate attributes to 0, but
        // modifiers themselves seem to stay according to effect code in decompiled client
        AEffectModifier {
            affector_attr_id: ac::attrs::IMPLANT_BONUS_VELOCITY,
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: ac::attrs::MAX_VELOCITY,
        },
        AEffectModifier {
            affector_attr_id: ac::attrs::MASS_BONUS_PERCENTAGE,
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: ac::attrs::MASS,
        },
        AEffectModifier {
            affector_attr_id: ac::attrs::SPEED_FACTOR_BONUS,
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::LocGrp(AEffectLocation::Ship, ac::itemgrps::PROPULSION_MODULE),
            affectee_attr_id: ac::attrs::SPEED_FACTOR,
        },
        AEffectModifier {
            affector_attr_id: ac::attrs::SPEED_BOOST_FACTOR_BONUS,
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::LocGrp(AEffectLocation::Ship, ac::itemgrps::PROPULSION_MODULE),
            affectee_attr_id: ac::attrs::SPEED_BOOST_FACTOR,
        },
    ]);
}
