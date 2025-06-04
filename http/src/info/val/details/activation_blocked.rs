#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::val) struct HValActivationBlockedFail {
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    module_ids: Vec<rc::ItemId>,
}
impl From<&rc::val::ValActivationBlockedFail> for HValActivationBlockedFail {
    fn from(core_val_fail: &rc::val::ValActivationBlockedFail) -> Self {
        Self {
            module_ids: core_val_fail.module_ids.clone(),
        }
    }
}
