use crate::{ItemTypeId, ModuleState, val::DetectedItemKind};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Copy, Clone)]
pub struct SrcItemInfo {
    #[cfg_attr(feature = "serde", serde(rename = "$key$"))]
    pub type_id: ItemTypeId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub detected_kind: Option<DetectedItemKind>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub module_max_state: Option<ModuleState>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcItemInfo {
    pub(crate) fn from_core(core_src_item: rc::src::SrcItem) -> Self {
        Self {
            type_id: core_src_item.get_type_id(),
            detected_kind: core_src_item.get_detected_kind(),
            module_max_state: core_src_item.get_module_max_state(),
        }
    }
}
