use crate::sol::{AttrMutationRequest, api::RawMAttrMut, uad::item::UadItem};

impl<'a> RawMAttrMut<'a> {
    /// Remove user-defined mutation for the attribute.
    pub fn remove(self) {
        let attr_mutations = vec![AttrMutationRequest {
            a_attr_id: self.a_attr_id,
            value: None,
        }];
        match self.sol.uad.items.get(self.item_key) {
            UadItem::Drone(_) => self
                .sol
                .internal_change_drone_mutation(self.item_key, attr_mutations)
                .unwrap(),
            UadItem::Module(_) => self
                .sol
                .internal_change_module_mutation(self.item_key, attr_mutations)
                .unwrap(),
            _ => unreachable!(),
        }
    }
}
