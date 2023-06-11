use crate::{
    defs::{ReeId, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_charge_info(&self, item_id: &ReeId) -> Result<ssn::SsChargeInfo> {
        Ok(self.get_charge(item_id)?.into())
    }
    // Non-public
    fn get_charge(&self, item_id: &ReeId) -> Result<&ssi::SsCharge> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Charge(charge) => Ok(charge),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsCharge::get_name(),
            ))),
        }
    }
    pub(in crate::ss::ss) fn add_charge_with_id_opt(
        &mut self,
        item_id: Option<ReeId>,
        fit_id: ReeId,
        a_item_id: Option<ReeInt>,
        cont_id: ReeId,
    ) -> Option<ssn::SsChargeInfo> {
        match (item_id, a_item_id) {
            (Some(item_id), Some(a_item_id)) => Some(self.add_charge_with_id(item_id, fit_id, a_item_id, cont_id)),
            _ => None,
        }
    }
    pub(in crate::ss::ss) fn add_charge_with_id(
        &mut self,
        item_id: ReeId,
        fit_id: ReeId,
        a_item_id: ReeInt,
        cont_id: ReeId,
    ) -> ssn::SsChargeInfo {
        let charge = ssi::SsCharge::new(&self.src, item_id, fit_id, a_item_id, cont_id);
        let info = ssn::SsChargeInfo::from(&charge);
        let item = ssi::SsItem::Charge(charge);
        self.add_item(item);
        info
    }
}
