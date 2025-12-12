use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::validation) struct HValCapUseFail {
    max_cap: rc::AttrVal,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    items: HashMap<rc::ItemId, rc::AttrVal>,
}
impl From<&rc::val::ValCapUseFail> for HValCapUseFail {
    fn from(core_val_fail: &rc::val::ValCapUseFail) -> Self {
        Self {
            max_cap: core_val_fail.max_cap,
            items: core_val_fail.items.clone(),
        }
    }
}
