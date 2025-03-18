use std::collections::HashMap;

#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HValMaxTypeFail {
    #[serde(flatten)]
    data: HashMap<rc::EItemId, HValMaxTypeTypeInfo>,
}
impl HValMaxTypeFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::SolValMaxTypeFail>> for HValMaxTypeFail {
    fn from(core_val_fails: &Vec<rc::SolValMaxTypeFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.type_id, v.into())).collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValMaxTypeTypeInfo {
    count: rc::Count,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    items: HashMap<rc::SolItemId, rc::Count>,
}
impl From<&rc::SolValMaxTypeFail> for HValMaxTypeTypeInfo {
    fn from(core_val_fail: &rc::SolValMaxTypeFail) -> Self {
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
