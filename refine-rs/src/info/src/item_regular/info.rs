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
#[derive(Clone)]
pub struct SrcItemInfo {
    #[cfg_attr(feature = "serde", serde(rename = "$key$"))]
    pub type_id: ItemTypeId,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub detected_kind: Option<DetectedItemKind>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub module_max_state: Option<ModuleState>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub ship_stance_type_ids: Vec<ItemTypeId>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcItemInfos {
    pub(super) fn from_core(core_src: &rc::Src, type_ids: Option<&[ItemTypeId]>) -> Self {
        let items = match type_ids {
            Some(type_ids) => {
                let mut seen_type_ids = RSet::default();
                type_ids
                    .iter()
                    .filter(|&&type_id| seen_type_ids.insert(type_id))
                    .filter_map(|&type_id| core_src.get_item(type_id).map(SrcItemInfo::from_core))
                    .collect()
            }
            None => SrcItemInfo::iter_from_core(core_src).collect(),
        };
        Self { items }
    }
}

impl SrcItemInfo {
    pub(crate) fn iter_from_core(core_src: &rc::Src) -> impl ExactSizeIterator<Item = Self> {
        core_src.iter_items().map(Self::from_core)
    }
    fn from_core(core_src_item: rc::src::SrcItem) -> Self {
        Self {
            type_id: core_src_item.get_type_id(),
            detected_kind: core_src_item.get_detected_kind(),
            module_max_state: core_src_item.get_module_max_state(),
            ship_stance_type_ids: core_src_item.iter_ship_stance_type_ids().collect(),
        }
    }
}
