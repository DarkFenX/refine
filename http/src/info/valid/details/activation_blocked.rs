#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::valid) struct HValActivationBlockedFail {
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    item_ids: Vec<rc::ItemId>,
}
impl HValActivationBlockedFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.item_ids.is_empty()
    }
}
impl From<&rc::val::ValActivationBlockedFail> for HValActivationBlockedFail {
    fn from(core_val_fails: &rc::val::ValActivationBlockedFail) -> Self {
        Self {
            item_ids: core_val_fails.item_ids.iter().copied().collect(),
        }
    }
}
