use serde::Serialize;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValProjImmunityFail {
    #[serde_as(as = "Map<DisplayFromStr, Vec<DisplayFromStr>>")]
    items: Vec<(rc::ItemId, Vec<rc::ItemId>)>,
}
impl From<&rc::val::ValProjImmunityFail> for HValProjImmunityFail {
    fn from(core_val_fail: &rc::val::ValProjImmunityFail) -> Self {
        Self {
            items: core_val_fail.items.iter().map(|(k, v)| (*k, v.clone())).collect(),
        }
    }
}
