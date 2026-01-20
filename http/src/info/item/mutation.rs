use serde_tuple::Serialize_tuple;
use serde_with::{DisplayFromStr, Map, serde_as};

#[serde_as]
#[derive(Serialize_tuple)]
pub(in crate::info::item) struct HItemMutationInfo {
    base_type_id: i32,
    mutator_id: i32,
    #[serde_as(as = "&Map<DisplayFromStr, _>")]
    attrs: Vec<(rc::AttrId, HAttrMutationInfo)>,
}

#[derive(Serialize_tuple)]
struct HAttrMutationInfo {
    roll: Option<f64>,
    value: f64,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HItemMutationInfo {
    pub(in crate::info::item) fn from_core(core_effective_mutation: rc::EffectiveMutation) -> Self {
        Self {
            base_type_id: core_effective_mutation.get_base_type_id().into_i32(),
            mutator_id: core_effective_mutation.get_mutator_type_id().into_i32(),
            attrs: core_effective_mutation
                .iter_full_mattrs()
                .map(|v| (v.get_attr_id(), HAttrMutationInfo::from_core(v)))
                .collect(),
        }
    }
}

impl HAttrMutationInfo {
    fn from_core(core_full_mutated_attr: rc::FullMAttr) -> Self {
        Self {
            roll: core_full_mutated_attr.get_roll().map(|v| v.into_f64()),
            value: core_full_mutated_attr.get_value().into_f64(),
        }
    }
}
