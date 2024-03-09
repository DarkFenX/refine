use crate::{
    defs::{AttrVal, EAttrId, SsItemId},
    ss::{item::SsItem, svc::SsSvcs, SsView},
    util::Result,
};

use super::custom::{aar, prop};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(super) enum SsAttrModSrc {
    AttrId(EAttrId),
    PropulsionModule,
    AncillaryArmorRep,
}
impl SsAttrModSrc {
    pub(super) fn get_src_attr_id(&self) -> Option<EAttrId> {
        match self {
            Self::AttrId(attr_id) => Some(*attr_id),
            Self::PropulsionModule => None,
            Self::AncillaryArmorRep => Some(aar::AAR_SRC_ATTR_ID),
        }
    }
    pub(super) fn get_mod_val(&self, svc: &mut SsSvcs, ss_view: &SsView, item_id: &SsItemId) -> Result<AttrVal> {
        match self {
            Self::AttrId(attr_id) => Ok(svc.calc_get_item_attr_val(ss_view, item_id, attr_id)?.dogma),
            Self::PropulsionModule => prop::get_mod_val(svc, ss_view, item_id),
            Self::AncillaryArmorRep => aar::get_mod_val(svc, ss_view, item_id),
        }
    }
    pub(super) fn on_effect_stop(&self, svc: &mut SsSvcs, ss_view: &SsView, item_id: &SsItemId) {
        match self {
            Self::AttrId(_) => (),
            Self::PropulsionModule => prop::on_effect_stop(svc, ss_view, item_id),
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
    pub(super) fn revise_on_item_add(&self, src_item: &SsItem, changed_item: &SsItem) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => aar::revise_on_item_add_removal(src_item, changed_item),
        }
    }
    pub(super) fn revise_on_item_remove(&self, src_item: &SsItem, changed_item: &SsItem) -> bool {
        match self {
            Self::AttrId(_) => false,
            Self::PropulsionModule => false,
            Self::AncillaryArmorRep => aar::revise_on_item_add_removal(src_item, changed_item),
        }
    }
}
