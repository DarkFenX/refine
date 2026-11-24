use crate::{ItemTypeId, ModuleState, val::DetectedItemKind};

/// Serialized as an entry of a map keyed by mutator ID, to be put into a map keyed by base type ID.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Copy, Clone)]
pub struct SrcMutatedItemInfo {
    #[cfg_attr(feature = "serde", serde(skip))]
    pub base_type_id: ItemTypeId,
    #[cfg_attr(feature = "serde", serde(rename = "$key$"))]
    pub mutator_id: ItemTypeId,
    pub type_id: ItemTypeId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub detected_kind: Option<DetectedItemKind>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub module_max_state: Option<ModuleState>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcMutatedItemInfo {
    pub(crate) fn from_core(base_type_id: ItemTypeId, mutator_id: ItemTypeId, core_src_item: rc::src::SrcItem) -> Self {
        Self {
            base_type_id,
            mutator_id,
            type_id: core_src_item.get_type_id(),
            detected_kind: core_src_item.get_detected_kind(),
            module_max_state: core_src_item.get_module_max_state(),
        }
    }
}
