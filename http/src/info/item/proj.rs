use serde::Serialize;
use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::item) struct HProjInfo {
    #[serde_as(as = "DisplayFromStr")]
    projectee_item_id: rc::ItemId,
}

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::item) struct HRangedProjInfo {
    #[serde_as(as = "DisplayFromStr")]
    projectee_item_id: rc::ItemId,
    range: Option<(f64, f64)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HProjInfo {
    pub(in crate::info::item) fn from_core(core_proj: rc::Proj) -> Self {
        Self {
            projectee_item_id: core_proj.get_projectee_item_id(),
        }
    }
}

impl HRangedProjInfo {
    pub(in crate::info::item) fn from_core(core_ranged_proj: rc::RangedProj) -> Self {
        Self {
            projectee_item_id: core_ranged_proj.get_projectee_item_id(),
            range: core_ranged_proj
                .get_range()
                .map(|proj_range| (proj_range.c2c.into_f64(), proj_range.s2s.into_f64())),
        }
    }
}
