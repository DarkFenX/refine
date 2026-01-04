use crate::{
    api::{AttrId, FullSideEffect, FullSideEffectMut},
    def::AttrVal,
};

impl<'a> FullSideEffect<'a> {
    /// Get side effect chance attribute ID.
    pub fn get_chance_attr_id(&self) -> AttrId {
        self.chance_attr_id.into()
    }
}

impl<'a> FullSideEffectMut<'a> {
    /// Get side effect chance attribute ID.
    pub fn get_chance_attr_id(&self) -> AttrId {
        self.chance_attr_id.into()
    }
    /// Get side effect chance.
    ///
    /// Since operation mutates solar system state, side effect has to be mutable as well.
    pub fn get_chance(&mut self) -> AttrVal {
        let attr_key = self.sol.u_data.src.get_attr_rid_by_aid(&self.chance_attr_id).unwrap();
        self.sol.internal_get_item_attr(self.key, attr_key).unwrap().extra
    }
}
