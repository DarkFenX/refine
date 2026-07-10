use crate::{api::RawMAttrMut, num::UnitInterval, ud::UAttrMutationRequest};

impl<'a> RawMAttrMut<'a> {
    /// Set roll for the attribute.
    pub fn set_roll(&mut self, roll: UnitInterval) {
        let attr_mutations = vec![UAttrMutationRequest {
            attr_aid: self.attr_aid,
            roll: Some(roll),
        }];
        self.sol
            .internal_change_item_mutation_attrs(self.item_uid, attr_mutations)
            .unwrap();
    }
}
