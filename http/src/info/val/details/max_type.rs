use std::collections::HashMap;

#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::val) struct HValMaxTypeFail {
    item_types: HashMap<rc::ItemTypeId, HValMaxTypeTypeInfo>,
}
impl From<&rc::val::ValMaxTypeFail> for HValMaxTypeFail {
    fn from(core_val_fail: &rc::val::ValMaxTypeFail) -> Self {
        Self {
            item_types: core_val_fail
                .item_types
                .iter()
                .map(|(item_type_id, item_type_info)| (*item_type_id, item_type_info.into()))
                .collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
struct HValMaxTypeTypeInfo {
    item_type_count: rc::Count,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    items: HashMap<rc::ItemId, rc::Count>,
}
impl From<&rc::val::ValMaxTypeTypeInfo> for HValMaxTypeTypeInfo {
    fn from(core_val_type_info: &rc::val::ValMaxTypeTypeInfo) -> Self {
        Self {
            item_type_count: core_val_type_info.item_type_count,
            items: core_val_type_info.items.clone(),
        }
    }
}
