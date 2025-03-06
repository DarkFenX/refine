use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValRigSizeFail {
    allowed_size: rc::AttrVal,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    mismatches: HashMap<rc::SolItemId, Option<rc::AttrVal>>,
}
impl From<&rc::SolValRigSizeFail> for HValRigSizeFail {
    fn from(core_val_fail: &rc::SolValRigSizeFail) -> Self {
        Self {
            allowed_size: core_val_fail.allowed_size,
            mismatches: core_val_fail
                .mismatches
                .iter()
                .map(|v| (v.item_id, v.rig_size))
                .collect(),
        }
    }
}
