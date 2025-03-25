use smallvec::{SmallVec, smallvec};

use crate::{
    ad,
    sol::{
        AttrVal, ItemId,
        svc::calc::{AffectorInfo, Calc},
        uad::{Uad, item::Item},
    },
};

use super::custom::{aar, prop};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) enum AffectorValue {
    AttrId(ad::AAttrId),
    Hardcoded(ad::AAttrVal),
    PropulsionModule,
    AncillaryArmorRep,
}
impl AffectorValue {
    // Simple and fast way to get affector attribute
    pub(super) fn get_affector_a_attr_id(&self) -> Option<ad::AAttrId> {
        match self {
            Self::AttrId(attr_id) => Some(*attr_id),
            Self::Hardcoded(_) => None,
            Self::PropulsionModule => None,
            Self::AncillaryArmorRep => Some(aar::AAR_MULTIPLIER),
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
            Self::PropulsionModule => prop::get_affector_info(uad, item_id),
            Self::AncillaryArmorRep => smallvec![AffectorInfo {
                item_id: *item_id,
                attr_id: Some(aar::AAR_MULTIPLIER)
            }],
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
            Self::PropulsionModule => prop::get_mod_val(calc, uad, item_id, a_effect_id),
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
    pub(super) fn revise_on_item_add(&self, affector_item: &Item, changed_item: &Item) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => aar::revise_on_item_add_removal(affector_item, changed_item),
        }
    }
    pub(super) fn revise_on_item_remove(&self, affector_item: &Item, changed_item: &Item) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::Hardcoded(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => aar::revise_on_item_add_removal(affector_item, changed_item),
        }
    }
}
