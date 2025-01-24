#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HSlotValFail {
    used: rc::Count,
    total: Option<rc::Count>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    users: Vec<rc::SolItemId>,
}
impl From<&rc::SolSlotValFail> for HSlotValFail {
    fn from(core_val_fail: &rc::SolSlotValFail) -> Self {
        Self {
            used: core_val_fail.used,
            total: core_val_fail.total,
            users: core_val_fail.users.clone(),
        }
    }
}
