use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HSlotIndexValFail {
    #[serde(flatten)]
    #[serde_as(as = "HashMap<_, Vec<serde_with::DisplayFromStr>>")]
    data: HashMap<rc::SlotNumber, Vec<rc::SolItemId>>,
}
impl HSlotIndexValFail {
    pub(in crate::info::valid) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
impl From<&Vec<rc::SolSlotIndexValFail>> for HSlotIndexValFail {
    fn from(core_val_fails: &Vec<rc::SolSlotIndexValFail>) -> Self {
        Self {
            data: core_val_fails.iter().map(|v| (v.slot, v.users.clone())).collect(),
        }
    }
}
