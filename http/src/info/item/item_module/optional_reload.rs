use serde_tuple::Serialize_tuple;

use crate::shared::HOptionalReload;

#[derive(Serialize_tuple)]
pub(in crate::info::item::item_module) struct HItemOptionalReloadInfo {
    value: HOptionalReload,
    overridden: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HItemOptionalReloadInfo {
    pub(in crate::info::item::item_module) fn from_core(core_optional_reload: rc::ItemOptionalReloadInfo) -> Self {
        Self {
            value: HOptionalReload::from_core(core_optional_reload.value),
            overridden: core_optional_reload.overridden,
        }
    }
}
