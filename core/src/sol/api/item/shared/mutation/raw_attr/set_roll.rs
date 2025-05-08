use crate::sol::{AttrMutationRequest, UnitInterval, api::RawMAttrMut, uad::item::UadItem};

impl<'a> RawMAttrMut<'a> {
    /// Set roll for the attribute.
    pub fn set_roll(&mut self, roll: UnitInterval) {
        let attr_mutations = vec![AttrMutationRequest {
            a_attr_id: self.a_attr_id,
            value: Some(roll),
        }];
        self.sol
            .internal_change_item_mutation_attrs(self.item_key, attr_mutations)
            .unwrap();
    }
}
