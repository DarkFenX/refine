use rc::Lender;
use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::validation) struct HValDroneGroupFail {
    allowed_group_ids: Vec<i32>,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    drone_groups: Vec<(rc::ItemId, i32)>,
}
impl From<&rc::val::ValDroneGroupFail> for HValDroneGroupFail {
    fn from(core_val_fail: &rc::val::ValDroneGroupFail) -> Self {
        Self {
            allowed_group_ids: core_val_fail.allowed_group_ids.iter().map(|v| v.into_i32()).collect(),
            drone_groups: core_val_fail
                .drone_groups
                .iter()
                .map((|(k, v)| (*k, v.into_i32())))
                .collect(),
        }
    }
}
