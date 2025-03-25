#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValSlotCountFail {
    used: rc::Count,
    max: Option<rc::Count>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    users: Vec<rc::ItemId>,
}
impl From<&rc::val::ValSlotCountFail> for HValSlotCountFail {
    fn from(core_val_fail: &rc::val::ValSlotCountFail) -> Self {
        Self {
            used: core_val_fail.used,
            max: core_val_fail.max,
            users: core_val_fail.users.clone(),
        }
    }
}
