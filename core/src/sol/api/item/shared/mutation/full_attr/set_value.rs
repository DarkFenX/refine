use crate::{
    def::AttrVal,
    sol::api::{FullMAttrMut, item::shared::mutation::resolve_absolutes_into_rolls_with_ids},
    ud::UItem,
};

impl<'a> FullMAttrMut<'a> {
    /// Set value for the attribute.
    ///
    /// If value is out of bounds allowed by mutator, it will be clamped. None as value removes
    /// user-defined mutation.
    pub fn set_value(&mut self, absolute_value: Option<AttrVal>) {
        let u_item = self.sol.u_data.items.get(self.item_key);
        let attr_mutation_request = match absolute_value {
            Some(absolute_value) => {
                let (base_a_item_id, a_mutator_id) = match u_item {
                    UItem::Drone(drone) => (
                        drone
                            .get_mutation_data()
                            .unwrap()
                            .get_cache()
                            .unwrap()
                            .get_base_a_item_id(),
                        drone.get_mutation_data().unwrap().get_a_mutator_id(),
                    ),
                    UItem::Module(module) => (
                        module
                            .get_mutation_data()
                            .unwrap()
                            .get_cache()
                            .unwrap()
                            .get_base_a_item_id(),
                        module.get_mutation_data().unwrap().get_a_mutator_id(),
                    ),
                    _ => unreachable!("unmutable item kind is used in mutation"),
                };
                resolve_absolutes_into_rolls_with_ids(
                    &self.sol.u_data.src,
                    &base_a_item_id,
                    &a_mutator_id,
                    &[(self.a_attr_id, absolute_value)],
                )
            }
            None => Vec::new(),
        };
        if attr_mutation_request.is_empty() {
            return;
        }
        self.sol
            .internal_change_item_mutation_attrs(self.item_key, attr_mutation_request)
            .unwrap();
    }
}
