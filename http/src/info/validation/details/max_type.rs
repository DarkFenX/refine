use std::collections::HashMap;

use serde::Serialize;
use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValMaxTypeFail {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    item_types: Vec<(i32, HValMaxTypeTypeInfo)>,
}
impl From<&rc::val::ValMaxTypeFail> for HValMaxTypeFail {
    fn from(core_val_fail: &rc::val::ValMaxTypeFail) -> Self {
        Self {
            item_types: core_val_fail
                .item_types
                .iter()
                .map(|(item_type_id, item_type_info)| (item_type_id.into_i32(), item_type_info.into()))
                .collect(),
        }
    }
}

#[serde_as]
#[derive(Serialize_tuple)]
struct HValMaxTypeTypeInfo {
    item_type_count: u32,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    items: Vec<(rc::ItemId, u32)>,
}
impl From<&rc::val::ValMaxTypeTypeInfo> for HValMaxTypeTypeInfo {
    fn from(core_val_type_info: &rc::val::ValMaxTypeTypeInfo) -> Self {
        Self {
            item_type_count: core_val_type_info.item_type_count.into_u32(),
            items: core_val_type_info
                .items
                .iter()
                .map(|(k, v)| (*k, v.into_u32()))
                .collect(),
        }
    }
}
