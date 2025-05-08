use crate::sol::{AttrMutationRequest, api::RawMAttrMut};

impl<'a> RawMAttrMut<'a> {
    /// Remove user-defined mutation for the attribute.
    pub fn remove(self) {
        let attr_mutations = vec![AttrMutationRequest {
            a_attr_id: self.a_attr_id,
            value: None,
        }];
        self.sol
            .internal_change_item_mutation_attrs(self.item_key, attr_mutations)
            .unwrap();
    }
}
