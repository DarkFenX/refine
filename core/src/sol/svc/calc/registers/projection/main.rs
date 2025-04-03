use crate::{
    ad,
    sol::{AttrVal, ItemId},
    util::HMap,
};

// Holds info about effect projections
#[derive(Clone)]
pub(in crate::sol::svc::calc) struct ProjectionRegister {
    pub(super) ranges: HMap<(ItemId, ad::AEffectId, ItemId), AttrVal>,
}
impl ProjectionRegister {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self { ranges: HMap::new() }
    }
    // Query methods
    pub(in crate::sol::svc::calc) fn get_range(
        &self,
        affector_item_id: ItemId,
        a_effect_id: ad::AEffectId,
        affectee_item_id: ItemId,
    ) -> Option<AttrVal> {
        self.ranges
            .get(&(affector_item_id, a_effect_id, affectee_item_id))
            .copied()
    }
    // Modification methods
    pub(in crate::sol::svc::calc) fn add_range(
        &mut self,
        affector_item_id: ItemId,
        a_effect_id: ad::AEffectId,
        affectee_item_id: ItemId,
        range: Option<AttrVal>,
    ) {
        if let Some(range) = range {
            self.ranges
                .insert((affector_item_id, a_effect_id, affectee_item_id), range);
        }
    }
    pub(in crate::sol::svc::calc) fn change_range(
        &mut self,
        affector_item_id: ItemId,
        a_effect_id: ad::AEffectId,
        affectee_item_id: ItemId,
        range: Option<AttrVal>,
    ) {
        match range {
            Some(range) => self
                .ranges
                .insert((affector_item_id, a_effect_id, affectee_item_id), range),
            None => self.ranges.remove(&(affector_item_id, a_effect_id, affectee_item_id)),
        };
    }
    pub(in crate::sol::svc::calc) fn remove_range(
        &mut self,
        affector_item_id: ItemId,
        a_effect_id: ad::AEffectId,
        affectee_item_id: ItemId,
    ) {
        self.ranges.remove(&(affector_item_id, a_effect_id, affectee_item_id));
    }
}
