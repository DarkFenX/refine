use crate::{
    sol::{ItemId, svc::vast::VastFitData},
    util::StSet,
};

pub struct ValItemVsShipKindFail {
    pub item_id: ItemId,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_item_vs_ship_kind_fast(&self, kfs: &StSet<ItemId>) -> bool {
        if self.mods_rigs_svcs_vs_ship_kind.is_empty() {
            return true;
        }
        self.mods_rigs_svcs_vs_ship_kind.difference(kfs).next().is_none()
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_item_vs_ship_kind_verbose(
        &self,
        kfs: &StSet<ItemId>,
    ) -> Vec<ValItemVsShipKindFail> {
        self.mods_rigs_svcs_vs_ship_kind
            .difference(kfs)
            .map(|v| ValItemVsShipKindFail { item_id: *v })
            .collect()
    }
}
