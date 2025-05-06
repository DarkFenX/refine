use crate::sol::{ItemAttrMutationValue, ItemChangeAttrMutation, UnitInterval, api::FullMAttrMut, uad::item::UadItem};

impl<'a> FullMAttrMut<'a> {
    /// Set roll for the attribute.
    ///
    /// None as value removes user-defined mutation.
    pub fn set_roll(&mut self, roll: Option<UnitInterval>) {
        let attr_mutations = vec![ItemChangeAttrMutation::new(
            self.a_attr_id,
            roll.map(ItemAttrMutationValue::Roll),
        )];
        match self.sol.uad.items.get(self.item_key) {
            UadItem::Drone(_) => self
                .sol
                .internal_change_drone_mutation(self.item_key, attr_mutations)
                .unwrap(),
            UadItem::Module(_) => self
                .sol
                .internal_change_module_mutation(self.item_key, attr_mutations)
                .unwrap(),
            _ => panic!(),
        }
    }
}
