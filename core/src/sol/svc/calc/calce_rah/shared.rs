use crate::{
    ac, ad,
    sol::{
        AttrVal, DpsProfile,
        uad::{Uad, fit::Fit},
    },
    util::sig_round,
};

pub(super) type TickCount = usize;
pub(super) const TICK_LIMIT: TickCount = 500;

pub(super) const RAH_EFFECT_ID: ad::AEffectId = ac::effects::ADAPTIVE_ARMOR_HARDENER;

pub(super) const ARMOR_EM_ATTR_ID: ad::AAttrId = ac::attrs::ARMOR_EM_DMG_RESONANCE;
pub(super) const ARMOR_THERM_ATTR_ID: ad::AAttrId = ac::attrs::ARMOR_THERM_DMG_RESONANCE;
pub(super) const ARMOR_KIN_ATTR_ID: ad::AAttrId = ac::attrs::ARMOR_KIN_DMG_RESONANCE;
pub(super) const ARMOR_EXPL_ATTR_ID: ad::AAttrId = ac::attrs::ARMOR_EXPL_DMG_RESONANCE;
pub(super) const RAH_SHIFT_ATTR_ID: ad::AAttrId = ac::attrs::RESIST_SHIFT_AMOUNT;
pub(super) const SHIELD_HP_ATTR_ID: ad::AAttrId = ac::attrs::SHIELD_CAPACITY;
pub(super) const ARMOR_HP_ATTR_ID: ad::AAttrId = ac::attrs::ARMOR_HP;
pub(super) const HULL_HP_ATTR_ID: ad::AAttrId = ac::attrs::HP;

pub(super) fn rah_round(val: AttrVal) -> AttrVal {
    sig_round(val, 10)
}

pub(super) fn get_fit_rah_incoming_dps(uad: &Uad, fit: &Fit) -> DpsProfile {
    match fit.rah_incoming_dps {
        Some(dps_profile) => dps_profile,
        None => uad.default_incoming_dps,
    }
}
