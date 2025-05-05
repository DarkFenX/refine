use std::collections::HashMap;

use crate::shared::HMutaRoll;

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HItemMutationInfo {
    pub(crate) base_type_id: rc::ItemTypeId,
    pub(crate) mutator_id: rc::ItemTypeId,
    pub(crate) attrs: HashMap<rc::AttrId, HAttrMutationInfo>,
}
impl From<rc::EffectiveMutation<'_>> for HItemMutationInfo {
    fn from(core_effective_mutation: rc::EffectiveMutation) -> Self {
        Self {
            base_type_id: core_effective_mutation.get_base_type_id(),
            mutator_id: core_effective_mutation.get_mutator_id(),
            attrs: core_effective_mutation
                .iter_full_mattrs()
                .map(|v| (v.get_attr_id(), v.into()))
                .collect(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HAttrMutationInfo {
    pub(crate) roll: Option<HMutaRoll>,
    pub(crate) value: rc::AttrVal,
}
impl From<rc::FullMAttr<'_>> for HAttrMutationInfo {
    fn from(core_full_mutated_attr: rc::FullMAttr) -> Self {
        Self {
            roll: core_full_mutated_attr.get_roll().map(|v| v.get_inner().into_inner()),
            value: core_full_mutated_attr.get_value(),
        }
    }
}
