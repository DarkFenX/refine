use crate::{ItemId, ProjRange};

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[derive(Copy, Clone)]
pub struct ProjInfo {
    pub projectee_item_id: ItemId,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct RangedProjInfo {
    pub projectee_item_id: ItemId,
    pub range: Option<ProjRange>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ProjInfo {
    pub(in crate::info::item) fn from_core(core_proj: rc::Proj) -> Self {
        Self {
            projectee_item_id: core_proj.get_projectee_item_id(),
        }
    }
}

impl RangedProjInfo {
    pub(in crate::info::item) fn from_core(core_ranged_proj: rc::RangedProj) -> Self {
        Self {
            projectee_item_id: core_ranged_proj.get_projectee_item_id(),
            range: core_ranged_proj.get_range(),
        }
    }
}
