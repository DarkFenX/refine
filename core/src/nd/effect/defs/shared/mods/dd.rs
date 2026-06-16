use super::cloak::mk_cannot_cloak_mod_hardcoded;
use crate::ad::{
    AAttrId, ABuffId, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectModStrength, AEffectModifier,
    AValue,
};

pub(in crate::nd::effect::defs) fn make_dd_self_debuffs() -> impl ExactSizeIterator<Item = AEffectBuffFull> {
    [
        // Short debuffs - as of 2026-06-13, includes only warp status debuff, and does not include
        // max velocity on doomsdays (tested on Singularity)
        // Visible in status bar
        AEffectBuffFull {
            buff_id: ABuffId::WARP_PENALTY,
            strength: AEffectModStrength::Attr(AAttrId::SIEGE_MODE_WARP_STATUS),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_IMMOBILITY_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        // Long debuffs
        // Visible in status bar
        AEffectBuffFull {
            buff_id: ABuffId::DISALLOW_CLOAK,
            strength: AEffectModStrength::Hardcoded(AValue::from_f64(1.0)),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_NO_JUMP_OR_CLOAK_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        // Visible in status bar
        AEffectBuffFull {
            buff_id: ABuffId::DISALLOW_TETHER,
            strength: AEffectModStrength::Attr(AAttrId::DISALLOW_TETHERING),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_NO_JUMP_OR_CLOAK_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        // DD effects seem to block a bunch of actions via custom code (custom error messages
        // instead of "external factors" one). The block lasts for 5 minutes (longer than DD cycle
        // duration with rapid firing trained), so it acts similarly to regular debuff.
        // Blocks docking, drive jumping and gate jumping
        AEffectBuffFull {
            buff_id: ABuffId::DISALLOW_DOCK_JUMP,
            strength: AEffectModStrength::Hardcoded(AValue::from_f64(1.0)),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_NO_JUMP_OR_CLOAK_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
    ]
    .into_iter()
}

pub(in crate::nd::effect::defs) fn make_burst_proj_self_mods() -> impl ExactSizeIterator<Item = AEffectModifier> {
    // Burst projectors stop only cloaking for the duration of the module, the rest is either
    // blocked by aggro, or not blocked altogether
    std::iter::once(mk_cannot_cloak_mod_hardcoded())
}
