use crate::{
    defines::{ReeId, ReeInt},
    ss::{
        item::{Charge, ChargeInfo, Item},
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
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
    pub(in crate::ss::ss) fn add_charge_with_id_opt(
        &mut self,
        item_id: Option<ReeId>,
        fit_id: ReeId,
        type_id: Option<ReeInt>,
        cont_id: ReeId,
    ) -> Option<ChargeInfo> {
        match (item_id, type_id) {
            (Some(iid), Some(tid)) => Some(self.add_charge_with_id(iid, fit_id, tid, cont_id)),
            _ => None,
        }
    }
    pub(in crate::ss::ss) fn add_charge_with_id(
        &mut self,
        item_id: ReeId,
        fit_id: ReeId,
        type_id: ReeInt,
        cont_id: ReeId,
    ) -> ChargeInfo {
        let charge = Charge::new(&self.src, item_id, fit_id, type_id, cont_id);
        let info = ChargeInfo::from(&charge);
        let item = Item::Charge(charge);
        self.add_item(item);
        info
    }
}
