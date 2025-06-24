use crate::{
    sol::{AttrVal, ItemKey, svc::EffectSpec},
    util::RMap,
};

// Holds info about effect projections
#[derive(Clone)]
pub(crate) struct EProjs {
    pub(super) ranges: RMap<(EffectSpec, ItemKey), AttrVal>,
}
impl EProjs {
    pub(in crate::sol::svc) fn new() -> Self {
        Self { ranges: RMap::new() }
    }
    // Query methods
    pub(in crate::sol::svc) fn get_range(
        &self,
        affector_espec: EffectSpec,
        affectee_item_key: ItemKey,
    ) -> Option<AttrVal> {
        self.ranges.get(&(affector_espec, affectee_item_key)).copied()
    }
    // Modification methods
    pub(in crate::sol::svc) fn add_range(
        &mut self,
        affector_espec: EffectSpec,
        affectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) {
        if let Some(range) = range {
            self.ranges.insert((affector_espec, affectee_item_key), range);
        }
    }
    pub(in crate::sol::svc) fn change_range(
        &mut self,
        affector_espec: EffectSpec,
        affectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) {
        match range {
            Some(range) => self.ranges.insert((affector_espec, affectee_item_key), range),
            None => self.ranges.remove(&(affector_espec, affectee_item_key)),
        };
    }
    pub(in crate::sol::svc) fn remove_range(&mut self, affector_espec: EffectSpec, affectee_item_key: ItemKey) {
        self.ranges.remove(&(affector_espec, affectee_item_key));
    }
}
