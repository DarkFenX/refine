use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HCapitalModValFail {
    max_subcap_volume: rc::AttrVal,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    module_volumes: HashMap<rc::SolItemId, rc::AttrVal>,
}
impl From<&rc::SolCapitalModValFail> for HCapitalModValFail {
    fn from(core_val_fail: &rc::SolCapitalModValFail) -> Self {
        Self {
            max_subcap_volume: core_val_fail.max_subcap_volume.clone(),
            module_volumes: core_val_fail.items.iter().map(|v| (v.item_id, v.volume)).collect(),
        }
    }
}
