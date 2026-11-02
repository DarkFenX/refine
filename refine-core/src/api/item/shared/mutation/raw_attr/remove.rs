use crate::{api::RawMAttrMut, ud::UAttrMutationRequest};

impl<'s> RawMAttrMut<'s> {
    /// Remove user-defined mutation for the attribute.
    pub fn remove(self) {
        let attr_mutation = UAttrMutationRequest {
            attr_aid: self.attr_aid,
            roll: None,
        };
        self.sol
            .internal_change_item_mutation_attr(self.item_uid, attr_mutation)
            .unwrap();
    }
}
