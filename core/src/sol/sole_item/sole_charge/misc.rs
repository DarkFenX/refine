use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolCharge, SolItem, SolItemState},
        item_info::SolChargeInfo,
        SolarSystem,
    },
};

impl SolarSystem {
    pub(in crate::sol::sole_item) fn add_charge_with_id(
        &mut self,
        item_id: SolItemId,
        fit_id: SolFitId,
        type_id: EItemId,
        cont_id: SolItemId,
        cont_state: SolItemState,
    ) -> SolChargeInfo {
        let charge = SolCharge::new(&self.src, item_id, fit_id, type_id, cont_id, cont_state, false);
        let info = SolChargeInfo::from(&charge);
        let item = SolItem::Charge(charge);
        self.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        info
    }
}
