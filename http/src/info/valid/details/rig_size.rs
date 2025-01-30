use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HRigSizeValFail {
    allowed_size: rc::AttrVal,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    mismatches: HashMap<rc::SolItemId, Option<rc::AttrVal>>,
}
impl From<&rc::SolRigSizeValFail> for HRigSizeValFail {
    fn from(core_val_fail: &rc::SolRigSizeValFail) -> Self {
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
