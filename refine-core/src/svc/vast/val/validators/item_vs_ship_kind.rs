use itertools::Itertools;

use crate::{
    ItemId,
    ad::AItemCatId,
    svc::{SvcCtx, vast::VastFitData},
    ud::{UData, UFit, UFitId, UItemId, UShipKind},
    util::RSet,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Result
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde_tuple::Serialize_tuple)
)]
#[derive(Clone)]
pub struct ValItemVsShipKindFail {
    /// Kind of current ship.
    pub ship_kind: ValShipKind,
    /// Items which need other ship kind, and what kind they need (either ship or structure).
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub items: Vec<ValItemVsShipKindItemInfo>,
}

#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(refine_serde::VecAsMapEntry))]
pub struct ValItemVsShipKindItemInfo {
    /// Items which need other ship kind.
    #[cfg_attr(feature = "serde", vec_map(key))]
    pub item_id: ItemId,
    /// Ship kind item needs.
    #[cfg_attr(feature = "serde", vec_map(value))]
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Validators
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_item_vs_ship_kind_fast(&self, kfs: &RSet<UItemId>) -> bool {
        if self.mods_rigs_svcs_vs_ship_kind.is_empty() {
            return true;
        }
        self.mods_rigs_svcs_vs_ship_kind.difference(kfs).next().is_none()
    }
    // Verbose validations
    pub(in crate::svc::vast::val) fn validate_item_vs_ship_kind_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
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
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    pub(in crate::svc::vast) fn item_vs_ship_kind_add(
        &mut self,
        u_data: &UData,
        item_uid: UItemId,
        item_cat: AItemCatId,
        fit_uid: UFitId,
    ) {
        if let Some(ship_kind) = get_item_vs_ship_kind_mismatch(u_data, u_data.fits.get(fit_uid), item_cat) {
            self.mods_rigs_svcs_vs_ship_kind.insert(item_uid, ship_kind);
        }
    }
}

// Ship kind an item of given category needs, when the fit ship is of another kind. Not enforced
// without ship, and for non-loaded ships
pub(in crate::svc::vast) fn get_item_vs_ship_kind_mismatch(
    u_data: &UData,
    fit: &UFit,
    item_cat: AItemCatId,
) -> Option<ValShipKind> {
    let ship_uid = fit.ship?;
    match item_cat {
        AItemCatId::MODULE => match fit.ship_kind {
            UShipKind::Ship => None,
            UShipKind::Structure => Some(ValShipKind::Ship),
            UShipKind::Unknown => u_data.items.get(ship_uid).is_loaded().then_some(ValShipKind::Ship),
        },
        AItemCatId::STRUCTURE_MODULE => match fit.ship_kind {
            UShipKind::Ship => Some(ValShipKind::Structure),
            UShipKind::Structure => None,
            UShipKind::Unknown => u_data.items.get(ship_uid).is_loaded().then_some(ValShipKind::Structure),
        },
        _ => None,
    }
}
