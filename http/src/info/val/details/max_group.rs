use std::collections::HashMap;

#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::val) struct HValMaxGroupFail {
    groups: HashMap<rc::ItemGrpId, HValMaxGroupGroupInfo>,
}
impl From<&rc::val::ValMaxGroupFail> for HValMaxGroupFail {
    fn from(core_val_fail: &rc::val::ValMaxGroupFail) -> Self {
        Self {
            groups: core_val_fail
                .groups
                .iter()
                .map(|(group_id, group_info)| (*group_id, group_info.into()))
                .collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::val) struct HValMaxGroupGroupInfo {
    group_item_count: rc::Count,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    items: HashMap<rc::ItemId, rc::Count>,
}
impl From<&rc::val::ValMaxGroupGroupInfo> for HValMaxGroupGroupInfo {
    fn from(core_val_group_info: &rc::val::ValMaxGroupGroupInfo) -> Self {
        Self {
            group_item_count: core_val_group_info.group_item_count,
            items: core_val_group_info.items.clone(),
        }
    }
}
