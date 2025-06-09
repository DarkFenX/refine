use std::collections::HashMap;

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::validation) struct HValCapitalModFail {
    max_subcap_volume: rc::AttrVal,
    #[serde_as(as = "&HashMap<serde_with::DisplayFromStr, _>")]
    module_volumes: HashMap<rc::ItemId, rc::AttrVal>,
}
impl From<&rc::val::ValCapitalModFail> for HValCapitalModFail {
    fn from(core_val_fail: &rc::val::ValCapitalModFail) -> Self {
        Self {
            max_subcap_volume: core_val_fail.max_subcap_volume,
            module_volumes: core_val_fail.module_volumes.clone(),
        }
    }
}
