use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::validation) struct HValRigSizeFail {
    allowed_size: rc::AttrVal,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    rig_sizes: HashMap<rc::ItemId, Option<rc::AttrVal>>,
}
impl From<&rc::val::ValRigSizeFail> for HValRigSizeFail {
    fn from(core_val_fail: &rc::val::ValRigSizeFail) -> Self {
        Self {
            allowed_size: core_val_fail.allowed_size,
            rig_sizes: core_val_fail.rig_sizes.clone(),
        }
    }
}
