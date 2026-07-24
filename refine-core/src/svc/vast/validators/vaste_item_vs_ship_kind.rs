use itertools::Itertools;

use crate::{
    ItemId,
    svc::{SvcCtx, vast::VastFitData},
    ud::{UFit, UItemId, UShipKind},
    util::RSet,
};

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Clone)]
pub struct ValItemVsShipKindFail {
    /// Kind of current ship.
    pub ship_kind: ValShipKind,
    /// Items which need other ship kind, and what kind they need (either ship or structure).
    #[cfg_attr(feature = "serde", serde(serialize_with = "custom_serde::as_map"))]
    pub items: Vec<ValItemVsShipKindItemInfo>,
}

#[derive(Copy, Clone)]
pub struct ValItemVsShipKindItemInfo {
    /// Items which need other ship kind.
    pub item_id: ItemId,
    /// Ship kind item needs.
    pub needed_ship_kind: ValShipKind,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "snake_case"))]
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
        let items = self
            .mods_rigs_svcs_vs_ship_kind
            .difference(kfs)
            .map(|(item_uid, needed_kind)| ValItemVsShipKindItemInfo {
                item_id: ctx.u_data.items.ext_id_by_int_id(*item_uid),
                needed_ship_kind: *needed_kind,
            })
            .collect_vec();
        match items.is_empty() {
            true => None,
            false => Some(ValItemVsShipKindFail {
                ship_kind: ValShipKind::from_u_ship_kind(fit.ship_kind),
                items,
            }),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{SerializeMap, Serializer};

    use super::*;

    pub(super) fn as_map<S>(items: &[ValItemVsShipKindItemInfo], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(items.len()))?;
        for item in items {
            map.serialize_entry(&item.item_id, &item.needed_ship_kind)?;
        }
        map.end()
    }
}
