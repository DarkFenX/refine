use crate::{
    defs::SolItemId,
    sol::{
        item::{SolAutoCharge, SolItem},
        item_info::SolAutoChargeInfo,
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_auto_charge_info(&self, item_id: &SolItemId) -> Result<SolAutoChargeInfo> {
        Ok(self.get_auto_charge(item_id)?.into())
    }
    // Non-public
    fn get_auto_charge(&self, item_id: &SolItemId) -> Result<&SolAutoCharge> {
        let item = self.items.get_item(item_id)?;
        match item {
            SolItem::AutoCharge(auto_charge) => Ok(auto_charge),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolAutoCharge::get_name(),
            ))),
        }
    }
}
