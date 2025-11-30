use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValProjFilterFail {
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, Vec<serde_with::DisplayFromStr>>")]
    items: HashMap<rc::ItemId, Vec<rc::ItemId>>,
}
impl From<&rc::val::ValProjFilterFail> for HValProjFilterFail {
    fn from(core_val_fail: &rc::val::ValProjFilterFail) -> Self {
        Self {
            items: core_val_fail.items.clone(),
        }
    }
}
