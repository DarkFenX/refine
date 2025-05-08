use crate::sol::{ItemKey, SolarSystem, err::ChargeFoundError};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_module_charge(
        &mut self,
        item_key: ItemKey,
    ) -> Result<(), ChargeFoundError> {
        let uad_module = self.uad.items.get(item_key).get_module().unwrap();
        let charge_key = match uad_module.get_charge_item_key() {
            Some(charge_key) => charge_key,
            None => {
                return Err(ChargeFoundError {
                    cont_item_id: uad_module.get_item_id(),
                });
            }
        };
        self.internal_remove_charge(charge_key);
        Ok(())
    }
}
