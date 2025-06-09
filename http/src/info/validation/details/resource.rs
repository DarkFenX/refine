use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::validation) struct HValResFail {
    used: rc::AttrVal,
    max: Option<rc::AttrVal>,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    users: HashMap<rc::ItemId, rc::AttrVal>,
}
impl From<&rc::val::ValResFail> for HValResFail {
    fn from(core_val_fail: &rc::val::ValResFail) -> Self {
        Self {
            used: core_val_fail.used,
            max: core_val_fail.max,
            users: core_val_fail.users.clone(),
        }
    }
}
