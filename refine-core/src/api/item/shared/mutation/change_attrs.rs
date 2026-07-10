use crate::{
    sol::SolarSystem,
    ud::{UAttrMutationRequest, UItem, UItemId, err::ItemMutatedError},
};

impl SolarSystem {
    pub(in crate::api) fn internal_change_item_mutation_attrs(
        &mut self,
        item_uid: UItemId,
        attr_mutations: Vec<UAttrMutationRequest>,
    ) -> Result<(), ItemMutatedError> {
        let u_item = self.u_data.items.get_mut(item_uid);
        match u_item {
            UItem::Drone(_) => self.internal_change_drone_mutation_attrs(item_uid, attr_mutations),
            UItem::Module(_) => self.internal_change_module_mutation_attrs(item_uid, attr_mutations),
            _ => unreachable!("unmutable item kind is used in mutation"),
        }
    }
}
