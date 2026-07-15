pub struct ItemMutationInfo {
    pub base_type_id: rc::ItemTypeId,
    pub mutator_id: rc::ItemTypeId,
    pub attrs: Vec<(rc::AttrId, AttrMutationInfo)>,
}

pub struct AttrMutationInfo {
    pub roll: Option<rc::UnitInterval>,
    pub value: rc::Value,
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
