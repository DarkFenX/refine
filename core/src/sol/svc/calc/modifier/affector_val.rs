use smallvec::{smallvec, SmallVec};

use crate::{
    defs::{AttrVal, EAttrId, EEffectId, SolItemId},
    sol::{
        svc::calc::{SolAffectorInfo, SolCalc},
        uad::{item::SolItem, SolUad},
    },
};

use super::custom::{aar, prop};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) enum SolAffectorValue {
    AttrId(EAttrId),
    Hardcoded(AttrVal),
    PropulsionModule,
    AncillaryArmorRep,
}
impl SolAffectorValue {
    // Simple and fast way to get affector attribute
    pub(super) fn get_affector_attr_id(&self) -> Option<EAttrId> {
        match self {
            Self::AttrId(attr_id) => Some(*attr_id),
            Self::Hardcoded(_) => None,
            Self::PropulsionModule => None,
            Self::AncillaryArmorRep => Some(aar::AAR_AFFECTOR_ATTR_ID),
        }
    }
    // More expensive, but comprehensive info about affecting items/attributes
    pub(super) fn get_affector_info(&self, uad: &SolUad, item_id: &SolItemId) -> SmallVec<SolAffectorInfo, 1> {
        match self {
            Self::AttrId(attr_id) => smallvec![SolAffectorInfo::new(*item_id, Some(*attr_id))],
            Self::Hardcoded(_) => smallvec![SolAffectorInfo::new(*item_id, None)],
            Self::PropulsionModule => prop::get_affector_info(uad, item_id),
            Self::AncillaryArmorRep => smallvec![SolAffectorInfo::new(*item_id, Some(aar::AAR_AFFECTOR_ATTR_ID))],
        }
    }
    pub(super) fn get_mod_val(
        &self,
        calc: &mut SolCalc,
        uad: &SolUad,
        item_id: &SolItemId,
        effect_id: &EEffectId,
    ) -> Option<AttrVal> {
        match self {
            Self::AttrId(attr_id) => Some(calc.get_item_attr_val(uad, item_id, attr_id).ok()?.dogma),
            Self::Hardcoded(val) => Some(*val),
            Self::PropulsionModule => prop::get_mod_val(calc, uad, item_id, effect_id),
            Self::AncillaryArmorRep => aar::get_mod_val(calc, uad, item_id),
        }
    }
    // Revision methods - define if modification value can change upon some action
    pub(super) fn revisable_on_item_add(&self) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => true,
        }
    }
    pub(super) fn revisable_on_item_remove(&self) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => true,
        }
    }
    pub(super) fn revise_on_item_add(&self, affector_item: &SolItem, changed_item: &SolItem) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => aar::revise_on_item_add_removal(affector_item, changed_item),
        }
    }
    pub(super) fn revise_on_item_remove(&self, affector_item: &SolItem, changed_item: &SolItem) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => aar::revise_on_item_add_removal(affector_item, changed_item),
        }
    }
}
