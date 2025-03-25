use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValDroneGroupFail {
    allowed_group_ids: Vec<rc::ItemGrpId>,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    drone_groups: HashMap<rc::ItemId, rc::ItemGrpId>,
}
impl From<&rc::val::ValDroneGroupFail> for HValDroneGroupFail {
    fn from(core_val_fail: &rc::val::ValDroneGroupFail) -> Self {
        Self {
            allowed_group_ids: core_val_fail.allowed_group_ids.clone(),
            drone_groups: core_val_fail.items.iter().map(|v| (v.item_id, v.group_id)).collect(),
        }
    }
}
