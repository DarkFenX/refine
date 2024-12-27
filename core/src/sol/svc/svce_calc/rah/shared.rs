use crate::{defs::EAttrId, ec, EEffectId};

pub(super) const RAH_EFFECT_ID: EEffectId = ec::effects::ADAPTIVE_ARMOR_HARDENER;

// List all armor resonance attributes and also define default sorting order. When equal damage is
// received across several damage types, those which come earlier in this list will be picked as
// donors
pub(super) const RES_ATTR_IDS: [EAttrId; 4] = [
    ec::attrs::ARMOR_EM_DMG_RESONANCE,
    ec::attrs::ARMOR_EXPL_DMG_RESONANCE,
    ec::attrs::ARMOR_KIN_DMG_RESONANCE,
    ec::attrs::ARMOR_THERM_DMG_RESONANCE,
];
