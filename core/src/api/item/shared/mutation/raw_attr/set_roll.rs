use crate::{api::RawMAttrMut, ud::UAttrMutationRequest, util::UnitInterval};

impl<'a> RawMAttrMut<'a> {
    /// Set roll for the attribute.
    pub fn set_roll(&mut self, roll: UnitInterval) {
        let attr_mutations = vec![UAttrMutationRequest {
            attr_id: self.attr_aid,
            value: Some(roll),
        }];
        self.sol
            .internal_change_item_mutation_attrs(self.item_key, attr_mutations)
            .unwrap();
    }
}
