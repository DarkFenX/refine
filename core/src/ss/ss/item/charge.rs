use crate::{
    defs::{ReeId, ReeInt},
    ss::{
        info::SsChargeInfo,
        item::{SsCharge, SsItem},
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_charge_info(&self, item_id: &ReeId) -> Result<SsChargeInfo> {
        Ok(self.get_charge(item_id)?.into())
    }
    // Non-public
    fn get_charge(&self, item_id: &ReeId) -> Result<&SsCharge> {
        let item = self.items.get_item(item_id)?;
        match item {
            SsItem::Charge(charge) => Ok(charge),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsCharge::get_name(),
            ))),
        }
    }
    pub(in crate::ss::ss) fn add_charge_with_id_opt(
        &mut self,
        item_id: Option<ReeId>,
        fit_id: ReeId,
        a_item_id: Option<ReeInt>,
        cont_id: ReeId,
    ) -> Option<SsChargeInfo> {
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
    ) -> SsChargeInfo {
        let charge = SsCharge::new(&self.src, item_id, fit_id, a_item_id, cont_id);
        let info = SsChargeInfo::from(&charge);
        let item = SsItem::Charge(charge);
        self.add_item(item);
        info
    }
}
