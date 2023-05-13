use crate::{
    ss::item::{Charge, ChargeInfo, Item},
    util::Named,
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    // Public
    pub fn get_charge_info(&self, item_id: &ReeId) -> Result<ChargeInfo> {
        Ok(self.get_charge(item_id)?.into())
    }
    // Non-public
    fn get_charge(&self, item_id: &ReeId) -> Result<&Charge> {
        match self.get_item(item_id)? {
            Item::Charge(c) => Ok(c),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Charge::get_name(), item_id),
            )),
        }
    }
}
