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
    pub(in crate::ss::ss) fn add_charge_with_id(
        &mut self,
        item_id: Option<ReeId>,
        fit_id: ReeId,
        type_id: Option<ReeInt>,
        cont_id: ReeId,
    ) -> Option<ChargeInfo> {
        match (item_id, type_id) {
            (Some(iid), Some(tid)) => {
                let charge = Charge::new(&self.src, iid, fit_id, tid, cont_id);
                let info = ChargeInfo::from(&charge);
                let item = Item::Charge(charge);
                self.add_item(item);
                Some(info)
            }
            _ => None,
        }
    }
}
