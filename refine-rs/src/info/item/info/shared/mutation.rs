use crate::{AttrId, ItemTypeId, UnitInterval, Value};

#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(tag = "type", rename_all = "snake_case")
)]
#[derive(Clone)]
pub enum ItemMutationInfo {
    /// This mutation has full effect upon item: item's type is mutated, its attributes are mutable.
    Effective(ItemMutationEffectiveInfo),
    /// This mutation lacks some info in the data source to be applied to the item, and has no
    /// effect on it.
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
    /// All the attributes which are mutable for the item in the context of data source used by sol.
    ///
    /// Exposes absolute value of every attribute, and roll value. If roll value was not previously
    /// set by user, it is calculated using base attribute value. There are rare cases when it can't
    /// be calculated, and it is the only case roll can be None.
    #[cfg_attr(
        feature = "serde",
        serde_as(as = "serde_with::Map<_, _>"),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub attrs: Vec<(AttrId, AttrMutationInfo)>,
    /// Attribute roll values as they are stored internally.
    ///
    /// Contains only data for attributes which were set by the user. This means that not all
    /// mutable attributes for the item can be exposed, and some of exposed attributes might be not
    /// mutable on the current data source.
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
