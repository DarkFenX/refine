use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HResValFail {
    used: rc::AttrVal,
    output: Option<rc::AttrVal>,
    #[serde_as(as = "std::collections::HashMap<serde_with::DisplayFromStr, _>")]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    users: HashMap<rc::SolItemId, rc::AttrVal>,
}
impl From<&rc::SolResValFail> for HResValFail {
    fn from(core_val_fail: &rc::SolResValFail) -> Self {
        Self {
            used: core_val_fail.used,
            output: core_val_fail.output,
            users: core_val_fail.users.iter().map(|v| (v.item_id, v.used)).collect(),
        }
    }
}
