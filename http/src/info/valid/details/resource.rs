use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValResFail {
    used: rc::AttrVal,
    max: Option<rc::AttrVal>,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    users: HashMap<rc::SolItemId, rc::AttrVal>,
}
impl From<&rc::SolValResFail> for HValResFail {
    fn from(core_val_fail: &rc::SolValResFail) -> Self {
        Self {
            used: core_val_fail.used,
            max: core_val_fail.max,
            users: core_val_fail.users.iter().map(|v| (v.item_id, v.used)).collect(),
        }
    }
}
