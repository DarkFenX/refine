#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(in crate::info::valid) struct HSlotIndexValFail {
    slot: rc::SlotNumber,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    users: Vec<rc::SolItemId>,
}
impl From<&rc::SolSlotIndexValFail> for HSlotIndexValFail {
    fn from(core_val_fail: &rc::SolSlotIndexValFail) -> Self {
        Self {
            slot: core_val_fail.slot,
            users: core_val_fail.users.clone(),
        }
    }
}
