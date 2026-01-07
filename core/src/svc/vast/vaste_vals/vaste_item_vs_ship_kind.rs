use std::collections::HashMap;

use crate::{
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UFit, UItemId, UShipKind},
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
impl ValShipKind {
    fn from_u_ship_kind(u_ship_kind: UShipKind) -> Self {
        match u_ship_kind {
            UShipKind::Ship => Self::Ship,
            UShipKind::Structure => Self::Structure,
            UShipKind::Unknown => Self::Unknown,
        }
    }
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_item_vs_ship_kind_fast(&self, kfs: &RSet<UItemId>) -> bool {
        if self.mods_rigs_svcs_vs_ship_kind.is_empty() {
            return true;
        }
        self.mods_rigs_svcs_vs_ship_kind.difference(kfs).next().is_none()
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_item_vs_ship_kind_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        fit: &UFit,
    ) -> Option<ValItemVsShipKindFail> {
        let items: HashMap<_, _> = self
            .mods_rigs_svcs_vs_ship_kind
            .difference(kfs)
            .map(|(item_key, needed_kind)| (ctx.u_data.items.xid_by_iid(*item_key), *needed_kind))
            .collect();
        match items.is_empty() {
            true => None,
            false => Some(ValItemVsShipKindFail {
                ship_kind: fit.ship_kind.into(),
                items,
            }),
        }
    }
}
