use crate::{
    AttrId, FullMAttrMut, ItemId, Value,
    api::item::shared::mutation::resolve_absolute_into_roll,
    ud::{UAttrMutationRequest, UItem},
};

impl<'s> FullMAttrMut<'s> {
    /// Set value for the attribute.
    ///
    /// If value is out of bounds allowed by mutator, it will be clamped. None as value removes
    /// user-defined mutation.
    pub fn set_value(&mut self, absolute_value: Option<Value>) -> Result<(), FullMAttrValueSetError> {
        let roll = match absolute_value {
            Some(absolute_value) => {
                let u_item = self.sol.u_data.items.get(self.item_uid);
                let (base_type_aid, mutator_type_aid) = match u_item {
                    UItem::Drone(drone) => (drone.get_base_type_aid(), drone.get_mutator_type_aid().unwrap()),
                    UItem::Module(module) => (module.get_base_type_aid(), module.get_mutator_type_aid().unwrap()),
                    _ => unreachable!("unmutable item kind is used in mutation"),
                };
                match resolve_absolute_into_roll(
                    &self.sol.u_data.r_data,
                    &base_type_aid,
                    &mutator_type_aid,
                    &self.attr_aid,
                    absolute_value,
                ) {
                    Some(roll) => Some(roll),
                    None => {
                        return Err(FullMAttrValueSetError::ZeroWidthRange(
                            self.sol.u_data.items.ext_id_by_int_id(self.item_uid),
                            AttrId::from_aid(self.attr_aid),
                        ));
                    }
                }
            }
            None => None,
        };
        let attr_mutation_request = vec![UAttrMutationRequest {
            attr_aid: self.attr_aid,
            roll,
        }];
        self.sol
            .internal_change_item_mutation_attrs(self.item_uid, attr_mutation_request)
            .unwrap();
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FullMAttrValueSetError {
    #[error("attribute {1} on item {0} has zero-width mutation range, value can't be set")]
    ZeroWidthRange(ItemId, AttrId),
}
