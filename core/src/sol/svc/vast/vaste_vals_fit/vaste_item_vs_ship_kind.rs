use itertools::Itertools;

use crate::{
    sol::{
        ItemId,
        svc::vast::VastFitData,
        uad::{fit::Fit, item::ShipKind},
    },
    util::HSet,
};

pub struct ValItemVsShipKindFail {
    pub ship_kind: ValShipKind,
    pub items: Vec<ValItemVsShipKindItemInfo>,
}
pub struct ValItemVsShipKindItemInfo {
    pub item_id: ItemId,
    pub needed_kind: ValShipKind,
}
#[derive(Copy, Clone)]
pub enum ValShipKind {
    Ship,
    Structure,
    Unknown,
}
impl From<ShipKind> for ValShipKind {
    fn from(ship_kind: ShipKind) -> Self {
        match ship_kind {
            ShipKind::Ship => Self::Ship,
            ShipKind::Structure => Self::Structure,
            ShipKind::Unknown => Self::Unknown,
        }
    }
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_item_vs_ship_kind_fast(&self, kfs: &HSet<ItemId>) -> bool {
        if self.mods_rigs_svcs_vs_ship_kind.is_empty() {
            return true;
        }
        self.mods_rigs_svcs_vs_ship_kind.difference(kfs).next().is_none()
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_item_vs_ship_kind_verbose(
        &self,
        kfs: &HSet<ItemId>,
        fit: &Fit,
    ) -> Option<ValItemVsShipKindFail> {
        let items = self
            .mods_rigs_svcs_vs_ship_kind
            .difference(kfs)
            .map(|(k, v)| ValItemVsShipKindItemInfo {
                item_id: *k,
                needed_kind: *v,
            })
            .collect_vec();
        if items.is_empty() {
            return None;
        }
        Some(ValItemVsShipKindFail {
            ship_kind: fit.kind.into(),
            items,
        })
    }
}
