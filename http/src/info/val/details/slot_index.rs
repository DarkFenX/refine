use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::val) struct HValSlotIndexFail {
    #[serde_as(as = "HashMap<_, Vec<serde_with::DisplayFromStr>>")]
    slot_users: HashMap<rc::SlotIndex, Vec<rc::ItemId>>,
}
impl From<&rc::val::ValSlotIndexFail> for HValSlotIndexFail {
    fn from(core_val_fail: &rc::val::ValSlotIndexFail) -> Self {
        Self {
            slot_users: core_val_fail.slot_users.clone(),
        }
    }
}
