use serde::Serialize;
use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValMaxGroupFail {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    groups: Vec<(i32, HValMaxGroupGroupInfo)>,
}

#[serde_as]
#[derive(Serialize_tuple)]
struct HValMaxGroupGroupInfo {
    group_item_count: u32,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    items: Vec<(rc::ItemId, u32)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HValMaxGroupFail {
    pub(in crate::info::validation) fn from_core(core_val_fail: rc::val::ValMaxGroupFail) -> Self {
        Self {
            groups: core_val_fail
                .groups
                .into_iter()
                .map(|(group_id, group_info)| (group_id.into_i32(), HValMaxGroupGroupInfo::from_core(group_info)))
                .collect(),
        }
    }
}

impl HValMaxGroupGroupInfo {
    fn from_core(core_val_group_info: rc::val::ValMaxGroupGroupInfo) -> Self {
        Self {
            group_item_count: core_val_group_info.group_item_count.into_u32(),
            items: core_val_group_info
                .items
                .into_iter()
                .map(|(k, v)| (k, v.into_u32()))
                .collect(),
        }
    }
}
