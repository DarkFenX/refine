use crate::{RawMAttr, UnitInterval, ud::UAttrMutationRequest};

impl<'s> RawMAttr<'s> {
    /// Set roll for the attribute.
    pub fn set_roll(&mut self, roll: UnitInterval) {
        let attr_mutation = UAttrMutationRequest {
            attr_aid: self.attr_aid,
            roll: Some(roll),
        };
        self.sol
            .internal_change_item_mutation_attr(self.item_uid, attr_mutation)
            .unwrap();
    }
}
