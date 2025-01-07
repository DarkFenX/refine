use crate::{
    defs::{AttrVal, EEffectId, SolItemId},
    util::StMap,
};

// Holds info about effect projections
#[derive(Clone)]
pub(in crate::sol::svc::calc) struct SolProjectionRegister {
    pub(super) ranges: StMap<(SolItemId, EEffectId, SolItemId), AttrVal>,
}
impl SolProjectionRegister {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self { ranges: StMap::new() }
    }
    // Query methods
    pub(in crate::sol::svc::calc) fn get_range(
        &self,
        affector_item_id: SolItemId,
        effect_id: EEffectId,
        affectee_item_id: SolItemId,
    ) -> Option<AttrVal> {
        self.ranges
            .get(&(affector_item_id, effect_id, affectee_item_id))
            .map(|v| *v)
    }
    // Modification methods
    pub(in crate::sol::svc::calc) fn add_range(
        &mut self,
        affector_item_id: SolItemId,
        effect_id: EEffectId,
        affectee_item_id: SolItemId,
        range: Option<AttrVal>,
    ) {
        if let Some(range) = range {
            self.ranges
                .insert((affector_item_id, effect_id, affectee_item_id), range);
        }
    }
    pub(in crate::sol::svc::calc) fn change_range(
        &mut self,
        affector_item_id: SolItemId,
        effect_id: EEffectId,
        affectee_item_id: SolItemId,
        range: Option<AttrVal>,
    ) {
        match range {
            Some(range) => self
                .ranges
                .insert((affector_item_id, effect_id, affectee_item_id), range),
            None => self.ranges.remove(&(affector_item_id, effect_id, affectee_item_id)),
        };
    }
    pub(in crate::sol::svc::calc) fn remove_range(
        &mut self,
        affector_item_id: SolItemId,
        effect_id: EEffectId,
        affectee_item_id: SolItemId,
    ) {
        self.ranges.remove(&(affector_item_id, effect_id, affectee_item_id));
    }
}
