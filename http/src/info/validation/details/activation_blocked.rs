use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValActivationBlockedFail {
    #[serde_as(as = "Vec<DisplayFromStr>")]
    module_ids: Vec<rc::ItemId>,
}
impl From<&rc::val::ValActivationBlockedFail> for HValActivationBlockedFail {
    fn from(core_val_fail: &rc::val::ValActivationBlockedFail) -> Self {
        Self {
            module_ids: core_val_fail.module_ids.clone(),
        }
    }
}
