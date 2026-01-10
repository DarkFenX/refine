use crate::{api::FullMAttrMut, num::UnitInterval, ud::UAttrMutationRequest};

impl<'a> FullMAttrMut<'a> {
    /// Set roll for the attribute.
    ///
    /// None as value removes user-defined mutation.
    pub fn set_roll(&mut self, roll: Option<UnitInterval>) {
        let attr_mutation_request = vec![UAttrMutationRequest {
            attr_aid: self.attr_aid,
            roll: roll,
        }];
        self.sol
            .internal_change_item_mutation_attrs(self.item_uid, attr_mutation_request)
            .unwrap();
    }
}
