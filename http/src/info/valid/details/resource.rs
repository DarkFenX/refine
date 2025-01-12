use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HResValFail {
    total_use: rc::AttrVal,
    output: rc::AttrVal,
    #[serde_as(as = "std::collections::HashMap<serde_with::DisplayFromStr, _>")]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    users: HashMap<rc::SolItemId, rc::AttrVal>,
}
impl From<&rc::SolResValFail> for HResValFail {
    fn from(core_val_fail: &rc::SolResValFail) -> Self {
        Self {
            total_use: core_val_fail.total_use,
            output: core_val_fail.output,
            users: core_val_fail.users.iter().map(|v| (v.item_id, v.used)).collect(),
        }
    }
}
