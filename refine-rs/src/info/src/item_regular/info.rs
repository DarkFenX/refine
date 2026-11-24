use crate::{DetectedItemKind, ItemTypeId, ModuleState, util::RSet};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
#[derive(Clone)]
pub struct SrcItemInfos {
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::KeyValueMap<_>"))]
    pub items: Vec<SrcItemInfo>,
}
impl SrcItemInfos {
    pub fn get_item(&self, type_id: &ItemTypeId) -> Option<&SrcItemInfo> {
        self.items.iter().find(|item| item.type_id == *type_id)
    }
}

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
impl SrcItemInfos {
    pub(super) fn from_core(core_src: &rc::Src, type_ids: &[ItemTypeId]) -> Self {
        let mut seen_type_ids = RSet::default();
        let items = type_ids
            .iter()
            .filter(|&&type_id| seen_type_ids.insert(type_id))
            .filter_map(|&type_id| core_src.get_item(type_id).map(SrcItemInfo::from_core))
            .collect();
        Self { items }
    }
}

impl SrcItemInfo {
    fn from_core(core_src_item: rc::src::SrcItem) -> Self {
        Self {
            type_id: core_src_item.get_type_id(),
            detected_kind: core_src_item.get_detected_kind(),
            module_max_state: core_src_item.get_module_max_state(),
        }
    }
}
