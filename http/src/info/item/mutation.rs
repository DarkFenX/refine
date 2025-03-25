use std::collections::HashMap;

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HItemMutationInfo {
    pub(crate) base_type_id: rc::ItemTypeId,
    pub(crate) mutator_id: rc::ItemTypeId,
    pub(crate) attrs: HashMap<rc::AttrId, HAttrMutationInfo>,
}
impl From<&rc::ItemMutationInfo> for HItemMutationInfo {
    fn from(core_item_mutation_info: &rc::ItemMutationInfo) -> Self {
        Self {
            base_type_id: core_item_mutation_info.base_type_id,
            mutator_id: core_item_mutation_info.mutator_id,
            attrs: core_item_mutation_info
                .attrs
                .iter()
                .map(|v| (v.attr_id, v.into()))
                .collect(),
        }
    }
}

#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HAttrMutationInfo {
    pub(crate) roll: Option<rc::MutaRoll>,
    pub(crate) value: rc::AttrVal,
}
impl From<&rc::AttrMutationInfo> for HAttrMutationInfo {
    fn from(core_attr_mutation_info: &rc::AttrMutationInfo) -> Self {
        Self {
            roll: core_attr_mutation_info.roll,
            value: core_attr_mutation_info.value,
        }
    }
}
