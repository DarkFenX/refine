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
impl From<&rc::val::ValMaxGroupFail> for HValMaxGroupFail {
    fn from(core_val_fail: &rc::val::ValMaxGroupFail) -> Self {
        Self {
            groups: core_val_fail
                .groups
                .iter()
                .map(|(group_id, group_info)| (group_id.into_i32(), group_info.into()))
                .collect(),
        }
    }
}

#[serde_as]
#[derive(Serialize_tuple)]
struct HValMaxGroupGroupInfo {
    group_item_count: u32,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    items: Vec<(rc::ItemId, u32)>,
}
impl From<&rc::val::ValMaxGroupGroupInfo> for HValMaxGroupGroupInfo {
    fn from(core_val_group_info: &rc::val::ValMaxGroupGroupInfo) -> Self {
        Self {
            group_item_count: core_val_group_info.group_item_count.into_u32(),
            items: core_val_group_info
                .items
                .iter()
                .map(|(k, v)| (*k, v.into_u32()))
                .collect(),
        }
    }
}
