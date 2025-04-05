use std::collections::HashMap;

use crate::shared::HModuleState;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(in crate::info::valid) struct HValModuleStateFail {
    #[serde_as(as = "HashMap<serde_with::DisplayFromStr, _>")]
    modules: HashMap<rc::ItemId, HValModuleStateModuleInfo>,
}
impl From<&rc::val::ValModuleStateFail> for HValModuleStateFail {
    fn from(core_val_fail: &rc::val::ValModuleStateFail) -> Self {
        Self {
            modules: core_val_fail
                .modules
                .iter()
                .map(|(module_item_id, module_info)| (*module_item_id, module_info.into()))
                .collect(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(in crate::info::valid) struct HValModuleStateModuleInfo {
    state: HModuleState,
    max_state: HModuleState,
}
impl From<&rc::val::ValModuleStateModuleInfo> for HValModuleStateModuleInfo {
    fn from(core_val_module_info: &rc::val::ValModuleStateModuleInfo) -> Self {
        Self {
            state: (&core_val_module_info.state).into(),
            max_state: (&core_val_module_info.max_state).into(),
        }
    }
}
