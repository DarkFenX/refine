use crate::{
    defs::{ReeId, ReeInt},
    ss::{
        info::ChargeInfo,
        item::{Charge, Item},
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
        let item = self.get_item(item_id)?;
        match item {
            Item::Charge(charge) => Ok(charge),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                Charge::get_name(),
            ))),
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
