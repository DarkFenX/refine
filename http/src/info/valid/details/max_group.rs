use std::collections::HashMap;

#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HValMaxGroupFail {
    #[serde(flatten)]
    data: HashMap<rc::ItemGrpId, HValMaxGroupGroupInfo>,
}
impl HValMaxGroupFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::val::ValMaxGroupFail>> for HValMaxGroupFail {
    fn from(core_val_fails: &Vec<rc::val::ValMaxGroupFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.group_id, v.into())).collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValMaxGroupGroupInfo {
    count: rc::Count,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    items: HashMap<rc::ItemId, rc::Count>,
}
impl From<&rc::val::ValMaxGroupFail> for HValMaxGroupGroupInfo {
    fn from(core_val_fail: &rc::val::ValMaxGroupFail) -> Self {
        Self {
            count: core_val_fail.count,
            items: core_val_fail
                .items
                .iter()
                .map(|v| (v.item_id, v.max_allowed_count))
                .collect(),
        }
    }
}
