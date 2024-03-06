use crate::{
    defs::{AttrVal, EAttrId, SsItemId},
    ss::{svc::SsSvcs, SsView},
    util::Result,
};

use super::custom::aar;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::ss::svc::calc::modifier) enum SsAttrModSrc {
    AttrId(EAttrId),
    PropulsionModule,
    AncillaryArmorRep,
}
impl SsAttrModSrc {
    pub(in crate::ss::svc::calc::modifier) fn get_src_attr_id(&self) -> Option<EAttrId> {
        match self {
            Self::AttrId(attr_id) => Some(*attr_id),
            Self::PropulsionModule => None,
            Self::AncillaryArmorRep => None,
        }
    }
    pub(in crate::ss::svc::calc::modifier) fn get_mod_val(
        &self,
        svc: &mut SsSvcs,
        ss_view: &SsView,
        item_id: &SsItemId,
    ) -> Result<AttrVal> {
        match self {
            Self::AttrId(attr_id) => Ok(svc.calc_get_item_attr_val(ss_view, item_id, attr_id)?.dogma),
            Self::PropulsionModule => Ok(1.0),
            Self::AncillaryArmorRep => aar::get_mod_val(svc, ss_view, item_id),
        }
    }
}
