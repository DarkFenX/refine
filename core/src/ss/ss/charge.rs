use crate::{
    defs::{ReeId, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_charge_info(&self, item_id: &ReeId) -> Result<ssn::ChargeInfo> {
        Ok(self.get_charge(item_id)?.into())
    }
    // Non-public
    fn get_charge(&self, item_id: &ReeId) -> Result<&ssi::Charge> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::Item::Charge(charge) => Ok(charge),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::Charge::get_name(),
            ))),
        }
    }
    pub(in crate::ss::ss) fn add_charge_with_id_opt(
        &mut self,
        item_id: Option<ReeId>,
        fit_id: ReeId,
        type_id: Option<ReeInt>,
        cont_id: ReeId,
    ) -> Option<ssn::ChargeInfo> {
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
    ) -> ssn::ChargeInfo {
        let charge = ssi::Charge::new(&self.src, item_id, fit_id, type_id, cont_id);
        let info = ssn::ChargeInfo::from(&charge);
        let item = ssi::Item::Charge(charge);
        self.add_item(item);
        info
    }
}
