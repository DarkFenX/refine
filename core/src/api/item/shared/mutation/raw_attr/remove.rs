use crate::{api::RawMAttrMut, ud::UAttrMutationRequest};

impl<'a> RawMAttrMut<'a> {
    /// Remove user-defined mutation for the attribute.
    pub fn remove(self) {
        let attr_mutations = vec![UAttrMutationRequest {
            attr_id: self.attr_aid,
            value: None,
        }];
        self.sol
            .internal_change_item_mutation_attrs(self.item_key, attr_mutations)
            .unwrap();
    }
}
