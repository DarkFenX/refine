use crate::{
    ac,
    ad::{AAttrId, AEffectId},
    def::AttrVal,
    util::sig_round,
};

pub(super) type TickCount = usize;
pub(super) const TICK_LIMIT: TickCount = 500;

pub(super) const RAH_EFFECT_ID: AEffectId = ac::effects::ADAPTIVE_ARMOR_HARDENER;

pub(super) const ARMOR_EM_ATTR_ID: AAttrId = ac::attrs::ARMOR_EM_DMG_RESONANCE;
pub(super) const ARMOR_THERM_ATTR_ID: AAttrId = ac::attrs::ARMOR_THERM_DMG_RESONANCE;
pub(super) const ARMOR_KIN_ATTR_ID: AAttrId = ac::attrs::ARMOR_KIN_DMG_RESONANCE;
pub(super) const ARMOR_EXPL_ATTR_ID: AAttrId = ac::attrs::ARMOR_EXPL_DMG_RESONANCE;
pub(super) const RAH_SHIFT_ATTR_ID: AAttrId = ac::attrs::RESIST_SHIFT_AMOUNT;
pub(super) const SHIELD_HP_ATTR_ID: AAttrId = ac::attrs::SHIELD_CAPACITY;
pub(super) const ARMOR_HP_ATTR_ID: AAttrId = ac::attrs::ARMOR_HP;
pub(super) const HULL_HP_ATTR_ID: AAttrId = ac::attrs::HP;

pub(super) fn rah_round(val: AttrVal) -> AttrVal {
    sig_round(val, 10)
}
