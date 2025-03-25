#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValUnusableSlotFail {
    max: Option<rc::Count>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    users: Vec<rc::ItemId>,
}
impl From<&rc::val::ValUnusableSlotFail> for HValUnusableSlotFail {
    fn from(core_val_fail: &rc::val::ValUnusableSlotFail) -> Self {
        Self {
            max: core_val_fail.max,
            users: core_val_fail.users.clone(),
        }
    }
}
