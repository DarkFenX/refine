use crate::{
    api::{AttrId, SideEffect, SideEffectMut},
    num::UnitInterval,
};

impl<'s> SideEffect<'s> {
    /// Get side effect chance attribute ID.
    pub fn get_chance_attr_id(&self) -> AttrId {
        AttrId::from_aid(self.chance_attr_aid)
    }
}

impl<'s> SideEffectMut<'s> {
    /// Get side effect chance attribute ID.
    pub fn get_chance_attr_id(&self) -> AttrId {
        AttrId::from_aid(self.chance_attr_aid)
    }
    /// Get side effect chance.
    ///
    /// Since operation mutates solar system state, side effect has to be mutable as well.
    pub fn get_chance(&mut self) -> UnitInterval {
        let attr_rid = self
            .sol
            .u_data
            .r_data
            .get_attr_rid_by_aid(&self.chance_attr_aid)
            .unwrap();
        UnitInterval::from_value_clamped(self.sol.internal_get_item_attr(self.item_uid, attr_rid).unwrap().extra)
    }
}
