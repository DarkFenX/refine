use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::validation) struct HValUnusableSlotFail {
    max: Option<u32>,
    #[serde_as(as = "Vec<DisplayFromStr>")]
    users: Vec<rc::ItemId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HValUnusableSlotFail {
    pub(in crate::info::validation) fn from_core(core_val_fail: rc::val::ValUnusableSlotFail) -> Self {
        Self {
            max: core_val_fail.max.map(|v| v.into_u32()),
            users: core_val_fail.users,
        }
    }
}
