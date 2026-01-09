use std::collections::HashMap;

use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValProjFilterFail {
    #[serde_as(as = "HashMap<DisplayFromStr, Vec<DisplayFromStr>>")]
    items: HashMap<rc::ItemId, Vec<rc::ItemId>>,
}
impl From<&rc::val::ValProjFilterFail> for HValProjFilterFail {
    fn from(core_val_fail: &rc::val::ValProjFilterFail) -> Self {
        Self {
            items: core_val_fail.items.clone(),
        }
    }
}
