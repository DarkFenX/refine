use smallvec::{SmallVec, smallvec};

use crate::{
    ad,
    sol::{
        AttrVal, ItemId,
        svc::calc::{AffectorInfo, Calc},
        uad::{Uad, item::Item},
    },
};

use super::custom::{aar_rep_amount, missile_flight_time, prop_speed_boost};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) enum AffectorValue {
    AttrId(ad::AAttrId),
    Hardcoded(ad::AAttrVal),
    PropSpeedBoost,
    AarRepAmount,
    MissileFlightTime,
}
impl AffectorValue {
    // Simple and fast way to get affector attribute
    pub(super) fn get_affector_a_attr_id(&self) -> Option<ad::AAttrId> {
        match self {
            Self::AttrId(attr_id) => Some(*attr_id),
            Self::Hardcoded(_) => None,
            Self::PropSpeedBoost => None,
            Self::AarRepAmount => Some(aar_rep_amount::AAR_MULTIPLIER),
            Self::MissileFlightTime => None,
        }
    }
    // More expensive, but comprehensive info about affecting items/attributes
    pub(super) fn get_affector_info(&self, uad: &Uad, item_id: &ItemId) -> SmallVec<AffectorInfo, 1> {
        match self {
            Self::AttrId(attr_id) => smallvec![AffectorInfo {
                item_id: *item_id,
                attr_id: Some(*attr_id)
            }],
            Self::Hardcoded(_) => smallvec![AffectorInfo {
                item_id: *item_id,
                attr_id: None
            }],
            Self::PropSpeedBoost => prop_speed_boost::get_affector_info(uad, item_id),
            Self::AarRepAmount => smallvec![AffectorInfo {
                item_id: *item_id,
                attr_id: Some(aar_rep_amount::AAR_MULTIPLIER)
            }],
            Self::MissileFlightTime => missile_flight_time::get_affector_info(uad, item_id),
        }
    }
    pub(super) fn get_mod_val(
        &self,
        calc: &mut Calc,
        uad: &Uad,
        item_id: &ItemId,
        a_effect_id: &ad::AEffectId,
    ) -> Option<AttrVal> {
        match self {
            Self::AttrId(a_attr_id) => Some(calc.get_item_attr_val_full(uad, item_id, a_attr_id).ok()?.dogma),
            Self::Hardcoded(a_val) => Some(*a_val),
            Self::PropSpeedBoost => prop_speed_boost::get_mod_val(calc, uad, item_id, a_effect_id),
            Self::AarRepAmount => aar_rep_amount::get_mod_val(calc, uad, item_id),
            Self::MissileFlightTime => missile_flight_time::get_mod_val(calc, uad, item_id, a_effect_id),
        }
    }
    // Revision methods - define if modification value can change upon some action
    pub(super) fn revisable_on_item_add(&self) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::PropSpeedBoost => false,
            Self::AarRepAmount => true,
            Self::MissileFlightTime => true,
        }
    }
    pub(super) fn revisable_on_item_remove(&self) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::PropSpeedBoost => false,
            Self::AarRepAmount => true,
            Self::MissileFlightTime => true,
        }
    }
    pub(super) fn revise_on_item_add(&self, affector_item: &Item, changed_item: &Item) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::PropSpeedBoost => false,
            Self::AarRepAmount => aar_rep_amount::revise_on_item_add_removal(affector_item, changed_item),
            Self::MissileFlightTime => missile_flight_time::revise_on_item_add_removal(affector_item, changed_item),
        }
    }
    pub(super) fn revise_on_item_remove(&self, affector_item: &Item, changed_item: &Item) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::PropSpeedBoost => false,
            Self::AarRepAmount => aar_rep_amount::revise_on_item_add_removal(affector_item, changed_item),
            Self::MissileFlightTime => missile_flight_time::revise_on_item_add_removal(affector_item, changed_item),
        }
    }
}
