use crate::{AttrId, SideEffect, UnitInterval};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> SideEffect<'s> {
    /// Get side effect chance attribute ID.
    pub fn get_chance_attr_id(&self) -> AttrId {
        AttrId::from_aid(self.chance_attr_aid)
    }
    /// Get side effect chance.
    pub fn get_chance(&mut self) -> UnitInterval {
        let attr_rid = self
            .core
            .u_data
            .r_data
            .get_attr_rid_by_aid(&self.chance_attr_aid)
            .unwrap();
        UnitInterval::from_finite_value_clamped(
            self.core.internal_get_item_attr(self.item_uid, attr_rid).unwrap().extra,
        )
    }
}
