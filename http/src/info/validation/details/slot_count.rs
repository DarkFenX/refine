use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::validation) struct HValSlotCountFail {
    used: u32,
    max: Option<u32>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    users: Vec<rc::ItemId>,
}
impl From<&rc::val::ValSlotCountFail> for HValSlotCountFail {
    fn from(core_val_fail: &rc::val::ValSlotCountFail) -> Self {
        Self {
            used: core_val_fail.used.into_u32(),
            max: core_val_fail.max.map(|v| v.into_u32()),
            users: core_val_fail.users.clone(),
        }
    }
}
