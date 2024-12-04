use crate::defs::{AttrVal, EAttrId, EItemId, MutaRoll};

pub struct SolItemMutationInfo {
    pub base_type_id: EItemId,
    pub mutator_id: EItemId,
    pub attrs: Vec<SolAttrMutationInfo>,
}
impl SolItemMutationInfo {
    pub(in crate::sol) fn new(base_type_id: EAttrId, mutator_id: EItemId, attrs: Vec<SolAttrMutationInfo>) -> Self {
        Self {
            base_type_id,
            mutator_id,
            attrs,
        }
    }
}

pub struct SolAttrMutationInfo {
    pub attr_id: EAttrId,
    pub roll: MutaRoll,
    pub value: AttrVal,
}
impl SolAttrMutationInfo {
    pub(in crate::sol) fn new(attr_id: EAttrId, roll: MutaRoll, value: AttrVal) -> Self {
        Self { attr_id, roll, value }
    }
}
