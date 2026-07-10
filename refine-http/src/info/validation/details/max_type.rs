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

#[serde_as]
#[derive(Serialize_tuple)]
struct HValMaxTypeTypeInfo {
    item_type_count: u32,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    items: Vec<(rc::ItemId, u32)>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HValMaxTypeFail {
    pub(in crate::info::validation) fn from_core(core_val_fail: rc::val::ValMaxTypeFail) -> Self {
        Self {
            item_types: core_val_fail
                .item_types
                .into_iter()
                .map(|(item_type_id, item_type_info)| {
                    (item_type_id.into_i32(), HValMaxTypeTypeInfo::from_core(item_type_info))
                })
                .collect(),
        }
    }
}

impl HValMaxTypeTypeInfo {
    fn from_core(core_val_type_info: rc::val::ValMaxTypeTypeInfo) -> Self {
        Self {
            item_type_count: core_val_type_info.item_type_count.into_u32(),
            items: core_val_type_info
                .items
                .into_iter()
                .map(|(k, v)| (k, v.into_u32()))
                .collect(),
        }
    }
}
