use crate::{
    DetectedItemKind, ItemTypeId, ModuleState,
    src::SrcMutation,
    util::{RMap, RSet},
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
#[derive(Clone)]
pub struct SrcMutatedItemInfos {
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub base_types: Vec<SrcMutatedItemBaseInfo>,
}
impl SrcMutatedItemInfos {
    pub fn get_item(&self, base_type_id: &ItemTypeId, mutator_id: &ItemTypeId) -> Option<&SrcMutatedItemInfo> {
        let base_type = self
            .base_types
            .iter()
            .find(|base_type| base_type.base_type_id == *base_type_id)?;
        base_type.items.iter().find(|item| item.mutator_id == *mutator_id)
    }
}

/// Mutated items of a base item type.
#[cfg_attr(feature = "serde", derive(refine_serde::VecAsMapEntry))]
#[derive(Clone)]
pub struct SrcMutatedItemBaseInfo {
    #[cfg_attr(feature = "serde", vec_map(key))]
    pub base_type_id: ItemTypeId,
    #[cfg_attr(feature = "serde", vec_map(value, serialize_as = "serde_with::KeyValueMap<_>"))]
    pub items: Vec<SrcMutatedItemInfo>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Copy, Clone)]
pub struct SrcMutatedItemInfo {
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
impl SrcMutatedItemInfos {
    pub(super) fn from_core(core_src: &rc::Src, mutations: Option<&[SrcMutation]>) -> Self {
        let mut base_types = Vec::new();
        let mut base_type_indices = RMap::default();
        match mutations {
            Some(mutations) => {
                let mut seen_mutations = RSet::default();
                for &mutation in mutations {
                    if !seen_mutations.insert(mutation) {
                        continue;
                    }
                    let SrcMutation {
                        base_type_id,
                        mutator_id,
                    } = mutation;
                    let Some(core_src_item) = core_src.get_mutated_item(base_type_id, mutator_id) else {
                        continue;
                    };
                    let item = SrcMutatedItemInfo::from_core(mutator_id, core_src_item);
                    push_item(&mut base_types, &mut base_type_indices, base_type_id, item);
                }
            }
            None => {
                for (base_type_id, item) in SrcMutatedItemInfo::iter_from_core(core_src) {
                    push_item(&mut base_types, &mut base_type_indices, base_type_id, item);
                }
            }
        }
        Self { base_types }
    }
}

impl SrcMutatedItemInfo {
    pub(crate) fn iter_from_core(core_src: &rc::Src) -> impl ExactSizeIterator<Item = (ItemTypeId, Self)> {
        core_src
            .iter_mutated_items()
            .map(|(base_type_id, mutator_id, core_src_item)| (base_type_id, Self::from_core(mutator_id, core_src_item)))
    }
    fn from_core(mutator_id: ItemTypeId, core_src_item: rc::src::SrcItem) -> Self {
        Self {
            mutator_id,
            type_id: core_src_item.get_type_id(),
            detected_kind: core_src_item.get_detected_kind(),
            module_max_state: core_src_item.get_module_max_state(),
        }
    }
}

fn push_item(
    base_types: &mut Vec<SrcMutatedItemBaseInfo>,
    base_type_indices: &mut RMap<ItemTypeId, usize>,
    base_type_id: ItemTypeId,
    item: SrcMutatedItemInfo,
) {
    let base_type_index = *base_type_indices.entry(base_type_id).or_insert_with(|| {
        base_types.push(SrcMutatedItemBaseInfo {
            base_type_id,
            items: Vec::new(),
        });
        base_types.len() - 1
    });
    base_types[base_type_index].items.push(item);
}
