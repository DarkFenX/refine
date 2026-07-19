use crate::{AttrId, ItemTypeId, UnitInterval, Value};

#[cfg_attr(feature = "serde", serde_with::serde_as, derive(serde_tuple::Serialize_tuple))]
pub struct ItemMutationInfo {
    pub base_type_id: ItemTypeId,
    pub mutator_id: ItemTypeId,
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::Map<_, _>"))]
    pub attrs: Vec<(AttrId, AttrMutationInfo)>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
pub struct AttrMutationInfo {
    pub roll: Option<UnitInterval>,
    pub value: Value,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemMutationInfo {
    pub(in crate::info::item) fn try_from_core(core_mutation: rc::Mutation) -> Option<Self> {
        let rc::Mutation::Effective(core_mutation) = core_mutation else {
            return None;
        };
        Some(Self {
            base_type_id: core_mutation.get_base_type_id(),
            mutator_id: core_mutation.get_mutator_type_id(),
            attrs: core_mutation
                .iter_full_mattrs()
                .map(|v| (v.get_attr_id(), AttrMutationInfo::from_core(v)))
                .collect(),
        })
    }
}

impl AttrMutationInfo {
    fn from_core(core_full_mutated_attr: rc::FullMAttr) -> Self {
        Self {
            roll: core_full_mutated_attr.get_roll(),
            value: core_full_mutated_attr.get_value(),
        }
    }
}
