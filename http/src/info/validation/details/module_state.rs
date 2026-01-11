use serde::Serialize;
use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

use crate::shared::HModuleState;

#[serde_as]
#[derive(Serialize)]
#[serde(transparent)]
pub(in crate::info::validation) struct HValModuleStateFail {
    #[serde_as(as = "Map<DisplayFromStr, _>")]
    modules: Vec<(rc::ItemId, HValModuleStateModuleInfo)>,
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

#[serde_as]
#[derive(Serialize_tuple)]
struct HValModuleStateModuleInfo {
    state: HModuleState,
    max_state: HModuleState,
}
impl From<&rc::val::ValModuleStateModuleInfo> for HValModuleStateModuleInfo {
    fn from(core_val_module_info: &rc::val::ValModuleStateModuleInfo) -> Self {
        Self {
            state: HModuleState::from_core(core_val_module_info.state),
            max_state: HModuleState::from_core(core_val_module_info.max_state),
        }
    }
}
