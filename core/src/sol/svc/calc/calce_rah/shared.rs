use crate::{ac, ad, sol::AttrVal, util::sig_round};

pub(super) type TickCount = usize;
pub(super) const TICK_LIMIT: TickCount = 500;

pub(super) const RAH_A_EFFECT_ID: ad::AEffectId = ac::effects::ADAPTIVE_ARMOR_HARDENER;

pub(super) const EM_A_ATTR_ID: ad::AAttrId = ac::attrs::ARMOR_EM_DMG_RESONANCE;
pub(super) const THERM_A_ATTR_ID: ad::AAttrId = ac::attrs::ARMOR_THERM_DMG_RESONANCE;
pub(super) const KIN_A_ATTR_ID: ad::AAttrId = ac::attrs::ARMOR_KIN_DMG_RESONANCE;
pub(super) const EXPL_A_ATTR_ID: ad::AAttrId = ac::attrs::ARMOR_EXPL_DMG_RESONANCE;
pub(super) const SHIFT_A_ATTR_ID: ad::AAttrId = ac::attrs::RESIST_SHIFT_AMOUNT;

pub(super) fn rah_round(val: AttrVal) -> AttrVal {
    sig_round(val, 10)
}
