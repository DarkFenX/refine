use std::collections::HashMap;

use crate::{
    def::ItemId,
    svc::{SvcCtx, vast::VastFitData},
    uad::{ShipKind, UadFit, UadItemKey},
    util::RSet,
};

/// Some items can only be fit to a ship or a structure. Currently, applies just to modules.
pub struct ValItemVsShipKindFail {
    /// Kind of current ship.
    pub ship_kind: ValShipKind,
    /// Map with items which need other ship kind, and what kind they need (either ship or
    /// structure).
    pub items: HashMap<ItemId, ValShipKind>,
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
    pub(in crate::svc::vast) fn validate_item_vs_ship_kind_fast(&self, kfs: &RSet<UadItemKey>) -> bool {
        if self.mods_rigs_svcs_vs_ship_kind.is_empty() {
            return true;
        }
        self.mods_rigs_svcs_vs_ship_kind.difference(kfs).next().is_none()
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_item_vs_ship_kind_verbose(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        fit: &UadFit,
    ) -> Option<ValItemVsShipKindFail> {
        let items: HashMap<_, _> = self
            .mods_rigs_svcs_vs_ship_kind
            .difference(kfs)
            .map(|(item_key, needed_kind)| (ctx.uad.items.id_by_key(*item_key), *needed_kind))
            .collect();
        match items.is_empty() {
            true => None,
            false => Some(ValItemVsShipKindFail {
                ship_kind: fit.kind.into(),
                items,
            }),
        }
    }
}
