use crate::{
    def::{AttrId, AttrVal},
    sol::api::{FullSideEffect, FullSideEffectMut},
};

impl<'a> FullSideEffect<'a> {
    /// Get side effect chance attribute ID.
    pub fn get_chance_attr_id(&self) -> AttrId {
        self.chance_a_attr_id
    }
}

impl<'a> FullSideEffectMut<'a> {
    /// Get side effect chance attribute ID.
    pub fn get_chance_attr_id(&self) -> AttrId {
        self.chance_a_attr_id
    }
    /// Get side effect chance.
    ///
    /// Since operation mutates solar system state, side effect has to be mutable as well.
    pub fn get_chance(&mut self) -> AttrVal {
        self.sol
            .internal_get_item_attr(self.key, &self.chance_a_attr_id)
            .unwrap()
            .extra
    }
}
