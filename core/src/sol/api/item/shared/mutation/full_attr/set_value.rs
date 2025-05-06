use crate::sol::{AttrVal, ItemAttrMutationValue, ItemChangeAttrMutation, api::FullMAttrMut, uad::item::UadItem};

impl<'a> FullMAttrMut<'a> {
    /// Set value for the attribute.
    ///
    /// If value is out of bounds allowed by mutator, it will be clamped. None as value removes
    /// user-defined mutation.
    pub fn set_value(&mut self, roll: Option<AttrVal>) {
        let attr_mutations = vec![ItemChangeAttrMutation::new(
            self.a_attr_id,
            roll.map(ItemAttrMutationValue::Absolute),
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
