use crate::{
    ad,
    sol::{
        ItemKey, SolarSystem, UnitInterval,
        api::{FullMAttrMut, RawMAttrMut},
        uad::item::{ItemAttrMutationValue, ItemChangeAttrMutation, UadItem},
    },
};

impl<'a> RawMAttrMut<'a> {
    pub fn set_roll(&mut self, roll: UnitInterval) {
        set_roll(self.sol, self.item_key, self.a_attr_id, Some(roll))
    }
}

impl<'a> FullMAttrMut<'a> {
    pub fn set_roll(&mut self, roll: Option<UnitInterval>) {
        set_roll(self.sol, self.item_key, self.a_attr_id, roll)
    }
}

fn set_roll(sol: &mut SolarSystem, item_key: ItemKey, a_attr_id: ad::AAttrId, roll: Option<UnitInterval>) {
    let attr_mutations = vec![ItemChangeAttrMutation::new(
        a_attr_id,
        roll.map(|v| ItemAttrMutationValue::Roll(v)),
    )];
    match sol.uad.items.get(item_key) {
        UadItem::Drone(_) => sol.internal_change_drone_mutation(item_key, attr_mutations).unwrap(),
        UadItem::Module(_) => sol.internal_change_module_mutation(item_key, attr_mutations).unwrap(),
        _ => panic!(),
    }
}
