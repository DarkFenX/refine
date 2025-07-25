use crate::{
    misc::AttrMutationRequest,
    sol::SolarSystem,
    ud::{UItem, UItemKey, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::sol) fn internal_change_item_mutation_attrs(
        &mut self,
        item_key: UItemKey,
        attr_mutations: Vec<AttrMutationRequest>,
    ) -> Result<(), ItemMutatedError> {
        let u_item = self.u_data.items.get_mut(item_key);
        match u_item {
            UItem::Drone(_) => self.internal_change_drone_mutation_attrs(item_key, attr_mutations),
            UItem::Module(_) => self.internal_change_module_mutation_attrs(item_key, attr_mutations),
            _ => unreachable!("unmutable item kind is used in mutation"),
        }
    }
}
