use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValUnusableResFail {
    max: Option<rc::AttrVal>,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    users: HashMap<rc::SolItemId, rc::AttrVal>,
}
impl From<&rc::SolValUnusableResFail> for HValUnusableResFail {
    fn from(core_val_fail: &rc::SolValUnusableResFail) -> Self {
        Self {
            max: core_val_fail.max,
            users: core_val_fail.users.iter().map(|v| (v.item_id, v.used)).collect(),
        }
    }
}
