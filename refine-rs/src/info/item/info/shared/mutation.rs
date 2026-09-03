use crate::{AttrId, ItemTypeId, UnitInterval, Value};

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(tag = "type", rename_all = "snake_case")
)]
#[derive(Clone)]
pub enum ItemMutationInfo {
    Effective(ItemMutationEffectiveInfo),
    Dormant(ItemMutationDormantInfo),
}
impl ItemMutationInfo {
    pub fn get_mutator_id(&self) -> ItemTypeId {
        match self {
            Self::Effective(inner) => inner.mutator_id,
            Self::Dormant(inner) => inner.mutator_id,
        }
    }
    pub fn get_rolls(&self) -> &Vec<(AttrId, UnitInterval)> {
        match self {
            Self::Effective(inner) => &inner.rolls,
            Self::Dormant(inner) => &inner.rolls,
        }
    }
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct ItemMutationEffectiveInfo {
    pub base_type_id: ItemTypeId,
    pub mutator_id: ItemTypeId,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub attrs: Vec<(AttrId, AttrMutationInfo)>,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub rolls: Vec<(AttrId, UnitInterval)>,
}

#[cfg_attr(feature = "serde", cfg_eval, serde_with::serde_as, derive(serde::Serialize))]
#[derive(Clone)]
pub struct ItemMutationDormantInfo {
    pub mutator_id: ItemTypeId,
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub rolls: Vec<(AttrId, UnitInterval)>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct AttrMutationInfo {
    pub roll: Option<UnitInterval>,
    pub value: Value,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ItemMutationInfo {
    pub(in crate::info::item) fn try_from_core(core_mutation: rc::Mutation) -> Self {
        match core_mutation {
            rc::Mutation::Effective(core_mutation) => Self::Effective(ItemMutationEffectiveInfo {
                base_type_id: core_mutation.get_base_type_id(),
                mutator_id: core_mutation.get_mutator_type_id(),
                attrs: core_mutation
                    .iter_full_mattrs()
                    .map(|v| (v.get_attr_id(), AttrMutationInfo::from_core(v)))
                    .collect(),
                rolls: core_mutation
                    .iter_raw_mattrs()
                    .map(|v| (v.get_attr_id(), v.get_roll()))
                    .collect(),
            }),
            rc::Mutation::Dormant(core_mutation) => Self::Dormant(ItemMutationDormantInfo {
                mutator_id: core_mutation.get_mutator_type_id(),
                rolls: core_mutation
                    .iter_raw_mattrs()
                    .map(|v| (v.get_attr_id(), v.get_roll()))
                    .collect(),
            }),
        }
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
