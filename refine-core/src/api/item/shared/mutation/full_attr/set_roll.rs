use crate::{api::FullMAttr, num::UnitInterval, ud::UAttrMutationRequest};

impl<'s> FullMAttr<'s> {
    /// Set roll for the attribute.
    ///
    /// None as value removes user-defined mutation.
    pub fn set_roll(&mut self, roll: Option<UnitInterval>) {
        let attr_mutation = UAttrMutationRequest {
            attr_aid: self.attr_aid,
            roll,
        };
        self.sol
            .internal_change_item_mutation_attr(self.item_uid, attr_mutation)
            .unwrap();
    }
}
