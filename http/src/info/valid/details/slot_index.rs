use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HValSlotIndexFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<_, Vec<serde_with::DisplayFromStr>>")]
    data: HashMap<rc::SlotIndex, Vec<rc::ItemId>>,
}
impl HValSlotIndexFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::val::ValSlotIndexFail>> for HValSlotIndexFail {
    fn from(core_val_fails: &Vec<rc::val::ValSlotIndexFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.slot, v.users.clone())).collect(),
        }
    }
}
