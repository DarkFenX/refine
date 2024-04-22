use crate::{
    defs::{AttrVal, EAttrId, SolItemId},
    sol::{item::SolItem, svc::SolSvcs, SolView},
    util::Result,
};

use super::custom::{aar, prop};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) enum SolAttrModSrc {
    AttrId(EAttrId),
    PropulsionModule,
    AncillaryArmorRep,
}
impl SolAttrModSrc {
    // Simple and fast way to get source attribute
    pub(super) fn get_src_attr_id(&self) -> Option<EAttrId> {
        match self {
            Self::AttrId(attr_id) => Some(*attr_id),
            Self::PropulsionModule => None,
            Self::AncillaryArmorRep => Some(aar::AAR_SRC_ATTR_ID),
        }
    }
    // More expensive, but comprehensive info about modification sources
    pub(super) fn get_srcs(&self, sol_view: &SolView, src_item_id: &SolItemId) -> Vec<(SolItemId, EAttrId)> {
        match self {
            Self::AttrId(attr_id) => vec![(*src_item_id, *attr_id)],
            Self::PropulsionModule => prop::get_srcs(sol_view, src_item_id),
            Self::AncillaryArmorRep => vec![(*src_item_id, aar::AAR_SRC_ATTR_ID)],
        }
    }
    pub(super) fn get_mod_val(&self, svc: &mut SolSvcs, sol_view: &SolView, item_id: &SolItemId) -> Result<AttrVal> {
        match self {
            Self::AttrId(attr_id) => Ok(svc.calc_get_item_attr_val(sol_view, item_id, attr_id)?.dogma),
            Self::PropulsionModule => prop::get_mod_val(svc, sol_view, item_id),
            Self::AncillaryArmorRep => aar::get_mod_val(svc, sol_view, item_id),
        }
    }
    pub(super) fn on_effect_stop(&self, svc: &mut SolSvcs, sol_view: &SolView, item_id: &SolItemId) {
        match self {
            Self::AttrId(_) => (),
            Self::PropulsionModule => prop::on_effect_stop(svc, sol_view, item_id),
            Self::AncillaryArmorRep => (),
        }
    }
    // Revision methods - define if modification value can change upon some action
    pub(super) fn revisable_on_item_add(&self) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => true,
        }
    }
    pub(super) fn revisable_on_item_remove(&self) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => true,
        }
    }
    pub(super) fn revise_on_item_add(&self, src_item: &SolItem, changed_item: &SolItem) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => aar::revise_on_item_add_removal(src_item, changed_item),
        }
    }
    pub(super) fn revise_on_item_remove(&self, src_item: &SolItem, changed_item: &SolItem) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => aar::revise_on_item_add_removal(src_item, changed_item),
        }
    }
}
