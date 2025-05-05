use crate::sol::{
    UnitInterval,
    api::FullMAttrMut,
    uad::item::{ItemAttrMutationValue, ItemChangeAttrMutation, UadItem},
};

impl<'a> FullMAttrMut<'a> {
    /// Set roll for the attribute.
    ///
    /// None as value removes user-defined mutation.
    pub fn set_roll(&mut self, roll: Option<UnitInterval>) {
        let attr_mutations = vec![ItemChangeAttrMutation::new(
            self.a_attr_id,
            roll.map(|v| ItemAttrMutationValue::Roll(v)),
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
