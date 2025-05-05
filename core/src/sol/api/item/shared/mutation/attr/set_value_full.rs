use crate::sol::{
    AttrVal,
    api::FullMAttrMut,
    uad::item::{ItemAttrMutationValue, ItemChangeAttrMutation, UadItem},
};

impl<'a> FullMAttrMut<'a> {
    pub fn set_value(&mut self, roll: Option<AttrVal>) {
        let attr_mutations = vec![ItemChangeAttrMutation::new(
            self.a_attr_id,
            roll.map(|v| ItemAttrMutationValue::Absolute(v)),
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
