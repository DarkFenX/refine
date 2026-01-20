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

#[serde_as]
#[derive(Serialize_tuple)]
struct HValModuleStateModuleInfo {
    state: HModuleState,
    max_state: HModuleState,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HValModuleStateFail {
    pub(in crate::info::validation) fn from_core(core_val_fail: rc::val::ValModuleStateFail) -> Self {
        Self {
            modules: core_val_fail
                .modules
                .into_iter()
                .map(|(module_item_id, module_info)| {
                    (module_item_id, HValModuleStateModuleInfo::from_core(module_info))
                })
                .collect(),
        }
    }
}

impl HValModuleStateModuleInfo {
    fn from_core(core_val_module_info: rc::val::ValModuleStateModuleInfo) -> Self {
        Self {
            state: HModuleState::from_core(core_val_module_info.state),
            max_state: HModuleState::from_core(core_val_module_info.max_state),
        }
    }
}
