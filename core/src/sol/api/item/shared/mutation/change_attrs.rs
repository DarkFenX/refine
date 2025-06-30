use crate::{
    def::ItemKey,
    misc::AttrMutationRequest,
    sol::SolarSystem,
    uad::{UadItem, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_change_item_mutation_attrs(
        &mut self,
        item_key: ItemKey,
        attr_mutations: Vec<AttrMutationRequest>,
    ) -> Result<(), ItemMutatedError> {
        let uad_item = self.uad.items.get_mut(item_key);
        match uad_item {
            UadItem::Drone(_) => self.internal_change_drone_mutation_attrs(item_key, attr_mutations),
            UadItem::Module(_) => self.internal_change_module_mutation_attrs(item_key, attr_mutations),
            _ => unreachable!("unmutable item kind is used in mutation"),
        }
    }
}
