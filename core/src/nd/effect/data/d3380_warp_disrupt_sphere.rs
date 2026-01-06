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
    ad::{
        AAttrId, ABuffId, AEffect, AEffectAffecteeFilter, AEffectBuff, AEffectBuffDuration, AEffectBuffFull,
        AEffectBuffScope, AEffectBuffStrength, AEffectId, AEffectLocation, AEffectModifier, AItemGrpId, AItemListId,
        AOp,
    },
    ed::EEffectId,
    nd::{
        NEffect, NEffectCharge, NEffectChargeDepl, NEffectChargeLoc,
        effect::data::shared::proj_mult::{get_simple_mod_proj_attrs, get_simple_s2s_noapp_proj_mult},
    },
};

const EFFECT_EID: EEffectId = EEffectId::WARP_DISRUPT_SPHERE;
const EFFECT_AID: AEffectId = AEffectId::WARP_DISRUPT_SPHERE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![
                // Disallows warping and jumping for everything in range, including self
                AEffectBuffFull {
                    buff_id: ABuffId::DISALLOW_WARP_JUMP,
                    strength: AEffectBuffStrength::Attr(AAttrId::DISALLOW_WARPING_JUMPING),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::DISALLOW_WARP_JUMP,
                    strength: AEffectBuffStrength::Attr(AAttrId::DISALLOW_WARPING_JUMPING),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Carrier,
                },
            ],
            ..
        }),
        adg_update_effect_fn: Some(update_effect),
        modifier_proj_attrs_getter: Some(get_simple_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_simple_s2s_noapp_proj_mult),
        charge: Some(NEffectCharge {
            location: NEffectChargeLoc::Loaded(NEffectChargeDepl::Undepletable),
            activates_charge: true,
        }),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: WDFG bubble effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        // Signature radius
        AEffectModifier {
            affector_attr_id: AAttrId::SIG_RADIUS_BONUS,
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: AAttrId::SIG_RADIUS,
        },
        // Disallow assistance
        AEffectModifier {
            affector_attr_id: AAttrId::DISALLOW_ASSISTANCE,
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: AAttrId::DISALLOW_ASSISTANCE,
        },
        // Transfer warp core scram strength to script
        AEffectModifier {
            affector_attr_id: AAttrId::WARP_SCRAMBLE_STRENGTH,
            op: AOp::PreAssign,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Other),
            affectee_attr_id: AAttrId::WARP_SCRAMBLE_STRENGTH,
        },
        // Transfer activation block strength to script
        AEffectModifier {
            affector_attr_id: AAttrId::ACTIVATION_BLOCKED_STRENGTH,
            op: AOp::PreAssign,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Other),
            affectee_attr_id: AAttrId::ACTIVATION_BLOCKED_STRENGTH,
        },
        // Modifiers which have been "disabled" by setting appropriate attributes to 0, but
        // modifiers themselves seem to stay according to effect code in decompiled client
        AEffectModifier {
            affector_attr_id: AAttrId::IMPLANT_BONUS_VELOCITY,
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: AAttrId::MAX_VELOCITY,
        },
        AEffectModifier {
            affector_attr_id: AAttrId::MASS_BONUS_PERCENTAGE,
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: AAttrId::MASS,
        },
        AEffectModifier {
            affector_attr_id: AAttrId::SPEED_FACTOR_BONUS,
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::LocGrp(AEffectLocation::Ship, AItemGrpId::PROPULSION_MODULE),
            affectee_attr_id: AAttrId::SPEED_FACTOR,
        },
        AEffectModifier {
            affector_attr_id: AAttrId::SPEED_BOOST_FACTOR_BONUS,
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::LocGrp(AEffectLocation::Ship, AItemGrpId::PROPULSION_MODULE),
            affectee_attr_id: AAttrId::SPEED_BOOST_FACTOR,
        },
    ]);
}
