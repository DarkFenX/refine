use crate::{defs::AttrVal, ec, util::sig_round, EEffectId};

pub(super) const TICK_LIMIT: usize = 500;

pub(super) const RAH_EFFECT_ID: EEffectId = ec::effects::ADAPTIVE_ARMOR_HARDENER;

pub(super) const EM_ATTR_ID: EEffectId = ec::attrs::ARMOR_EM_DMG_RESONANCE;
pub(super) const THERM_ATTR_ID: EEffectId = ec::attrs::ARMOR_THERM_DMG_RESONANCE;
pub(super) const KIN_ATTR_ID: EEffectId = ec::attrs::ARMOR_KIN_DMG_RESONANCE;
pub(super) const EXPL_ATTR_ID: EEffectId = ec::attrs::ARMOR_EXPL_DMG_RESONANCE;
pub(super) const SHIFT_ATTR_ID: EEffectId = ec::attrs::RESIST_SHIFT_AMOUNT;

pub(super) fn rah_round(val: AttrVal) -> AttrVal {
    sig_round(val, 10)
}
