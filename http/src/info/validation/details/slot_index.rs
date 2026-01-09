use serde::Serialize;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValSlotIndexFail {
    #[serde_as(as = "Map<_, Vec<DisplayFromStr>>")]
    slot_users: Vec<(i32, Vec<rc::ItemId>)>,
}
impl From<&rc::val::ValSlotIndexFail> for HValSlotIndexFail {
    fn from(core_val_fail: &rc::val::ValSlotIndexFail) -> Self {
        Self {
            slot_users: core_val_fail
                .slot_users
                .iter()
                .map(|(k, v)| (k.into_i32(), v.clone()))
                .collect(),
        }
    }
}
