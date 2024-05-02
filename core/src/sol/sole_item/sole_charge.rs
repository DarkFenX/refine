use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolCharge, SolItem},
        item_info::SolChargeInfo,
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_charge_info(&self, item_id: &SolItemId) -> Result<SolChargeInfo> {
        Ok(self.get_charge(item_id)?.into())
    }
    // Non-public
    fn get_charge(&self, item_id: &SolItemId) -> Result<&SolCharge> {
        let item = self.items.get_item(item_id)?;
        match item {
            SolItem::Charge(charge) => Ok(charge),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolCharge::get_name(),
            ))),
        }
    }
    pub(in crate::sol::sole_item) fn add_charge_with_id_opt(
        &mut self,
        item_id: Option<SolItemId>,
        fit_id: SolFitId,
        a_item_id: Option<EItemId>,
        cont_id: SolItemId,
    ) -> Option<SolChargeInfo> {
        match (item_id, a_item_id) {
            (Some(item_id), Some(a_item_id)) => Some(self.add_charge_with_id(item_id, fit_id, a_item_id, cont_id)),
            _ => None,
        }
    }
    pub(in crate::sol::sole_item) fn add_charge_with_id(
        &mut self,
        item_id: SolItemId,
        fit_id: SolFitId,
        a_item_id: EItemId,
        cont_id: SolItemId,
    ) -> SolChargeInfo {
        let charge = SolCharge::new(&self.src, item_id, fit_id, a_item_id, cont_id);
        let info = SolChargeInfo::from(&charge);
        let item = SolItem::Charge(charge);
        self.add_item(item);
        info
    }
}
