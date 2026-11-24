use crate::{AttrId, UnitInterval, Value, src::SrcMutation, util::RSet};

/// Rolls which give attributes of a mutated item requested absolute values.
#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
#[derive(Clone)]
pub struct SrcMutatedAttrRolls {
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::Map<_, _>"))]
    pub rolls: Vec<(AttrId, UnitInterval)>,
}
impl SrcMutatedAttrRolls {
    pub fn get_roll(&self, attr_id: &AttrId) -> Option<UnitInterval> {
        self.rolls.iter().find_map(|&(id, roll)| match id == *attr_id {
            true => Some(roll),
            false => None,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SrcMutatedAttrRolls {
    pub(super) fn from_core(core_src: &rc::Src, mutation: SrcMutation, values: &[(AttrId, Value)]) -> Self {
        let SrcMutation {
            base_type_id,
            mutator_id,
        } = mutation;
        let mut seen_attr_ids = RSet::default();
        let rolls = values
            .iter()
            .filter(|&&(attr_id, _)| seen_attr_ids.insert(attr_id))
            .filter_map(|&(attr_id, value)| {
                core_src
                    .get_mutated_attr_roll(base_type_id, mutator_id, attr_id, value)
                    .map(|roll| (attr_id, roll))
            })
            .collect();
        Self { rolls }
    }
}
