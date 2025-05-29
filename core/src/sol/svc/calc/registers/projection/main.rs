use crate::{
    sol::{AttrVal, ItemKey, svc::EffectSpec},
    util::RMap,
};

// Holds info about effect projections
#[derive(Clone)]
pub(in crate::sol::svc::calc) struct ProjectionRegister {
    pub(super) ranges: RMap<(EffectSpec, ItemKey), AttrVal>,
}
impl ProjectionRegister {
    pub(in crate::sol::svc::calc) fn new() -> Self {
        Self { ranges: RMap::new() }
    }
    // Query methods
    pub(in crate::sol::svc::calc) fn get_range(
        &self,
        affector_espec: EffectSpec,
        affectee_item_key: ItemKey,
    ) -> Option<AttrVal> {
        self.ranges.get(&(affector_espec, affectee_item_key)).copied()
    }
    // Modification methods
    pub(in crate::sol::svc::calc) fn add_range(
        &mut self,
        affector_espec: EffectSpec,
        affectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) {
        if let Some(range) = range {
            self.ranges.insert((affector_espec, affectee_item_key), range);
        }
    }
    pub(in crate::sol::svc::calc) fn change_range(
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
    pub(in crate::sol::svc::calc) fn remove_range(&mut self, affector_espec: EffectSpec, affectee_item_key: ItemKey) {
        self.ranges.remove(&(affector_espec, affectee_item_key));
    }
}
