use crate::sol::{AttrMutationRequest, UnitInterval, api::RawMAttrMut, uad::item::UadItem};

impl<'a> RawMAttrMut<'a> {
    /// Set roll for the attribute.
    pub fn set_roll(&mut self, roll: UnitInterval) {
        let attr_mutations = vec![AttrMutationRequest {
            a_attr_id: self.a_attr_id,
            value: Some(roll),
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
