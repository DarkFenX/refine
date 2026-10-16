use crate::{
    ItemGrpId, ItemId, ItemTypeId,
    ad::{AItemGrpId, AItemId},
    rd::RItemShipLimit,
    svc::{SvcCtx, vast::VastFitData},
    ud::{UData, UFit, UFitId, UItemId, UShip},
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
pub struct ValShipLimitFail {
    /// Type ID of current ship.
    pub ship_type_id: Option<ItemTypeId>,
    /// Group ID of current ship.
    pub ship_group_id: Option<ItemGrpId>,
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::KeyValueMap<_>"))]
    pub items: Vec<ValShipLimitItemInfo>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Clone)]
pub struct ValShipLimitItemInfo {
    /// Item which cannot be fit to current ship.
    pub item_id: ItemId,
    /// Ship type IDs item can be fit to.
    pub allowed_type_ids: Vec<ItemTypeId>,
    /// Ship group IDs item can be fit to.
    pub allowed_group_ids: Vec<ItemGrpId>,
}
impl ValShipLimitItemInfo {
    fn from_id_and_r_limit(item_id: ItemId, item_ship_limit: &RItemShipLimit) -> Self {
        Self {
            item_id,
            allowed_type_ids: item_ship_limit
                .type_aids
                .iter()
                .map(|type_aid| ItemTypeId::from_aid(*type_aid))
                .collect(),
            allowed_group_ids: item_ship_limit
                .group_aids
                .iter()
                .map(|item_grp_aid| ItemGrpId::from_aid(*item_grp_aid))
                .collect(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Validators
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_ship_limit_fast(&self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.ship_limit_mismatches.is_empty(),
            false => self.ship_limit_mismatches.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast::val) fn validate_ship_limit_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        ship: Option<&UShip>,
    ) -> Option<ValShipLimitFail> {
        if self.ship_limit_mismatches.is_empty() {
            return None;
        }
        let (ship_type_aid, ship_group_aid) = match ship {
            Some(ship) => (Some(ship.get_type_aid()), ship.get_r_item_base().map(|v| v.grp_id)),
            None => (None, None),
        };
        let mut mismatches = Vec::new();
        for limited_item_uid in self.ship_limit_mismatches.iter() {
            if kfs.contains(limited_item_uid) {
                continue;
            }
            let limited_item_rib = ctx.u_data.items.get(*limited_item_uid).get_r_item_base().unwrap();
            mismatches.push(ValShipLimitItemInfo::from_id_and_r_limit(
                ctx.u_data.items.ext_id_by_int_id(*limited_item_uid),
                limited_item_rib.ship_limit.as_ref().unwrap(),
            ));
        }
        match mismatches.is_empty() {
            true => None,
            false => Some(ValShipLimitFail {
                ship_type_id: ship_type_aid.map(ItemTypeId::from_aid),
                ship_group_id: ship_group_aid.map(ItemGrpId::from_aid),
                items: mismatches,
            }),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    pub(in crate::svc::vast) fn ship_limit_add(
        &mut self,
        u_data: &UData,
        fit_uid: UFitId,
        item_uid: UItemId,
        ship_limit: &RItemShipLimit,
    ) {
        self.ship_limited_items.insert(item_uid);
        let (ship_type_aid, ship_grp_aid) = get_fit_ship_type_group(u_data, u_data.fits.get(fit_uid));
        if !is_ship_limit_met(ship_limit, ship_type_aid, ship_grp_aid) {
            self.ship_limit_mismatches.insert(item_uid);
        }
    }
    pub(in crate::svc::vast) fn refresh_ship_limit_mismatches(
        &mut self,
        u_data: &UData,
        ship_type_aid: Option<AItemId>,
        ship_grp_aid: Option<AItemGrpId>,
    ) {
        self.ship_limit_mismatches.clear();
        for &item_uid in self.ship_limited_items.iter() {
            let item_rib = u_data.items.get(item_uid).get_r_item_base().unwrap();
            if !is_ship_limit_met(item_rib.ship_limit.as_ref().unwrap(), ship_type_aid, ship_grp_aid) {
                self.ship_limit_mismatches.insert(item_uid);
            }
        }
    }
}

// Ship type and group which ship limits of items are checked against. Group is known only for
// loaded ships, and without ship no item with a ship limit can be fit
pub(in crate::svc::vast) fn get_fit_ship_type_group(
    u_data: &UData,
    fit: &UFit,
) -> (Option<AItemId>, Option<AItemGrpId>) {
    match fit.ship {
        Some(ship_uid) => {
            let ship = u_data.items.get(ship_uid);
            (Some(ship.get_type_aid()), ship.get_r_item_base().map(|v| v.grp_id))
        }
        None => (None, None),
    }
}

pub(in crate::svc::vast) fn is_ship_limit_met(
    ship_limit: &RItemShipLimit,
    ship_type_aid: Option<AItemId>,
    ship_grp_aid: Option<AItemGrpId>,
) -> bool {
    ship_type_aid.is_some_and(|v| ship_limit.type_aids.contains(&v))
        || ship_grp_aid.is_some_and(|v| ship_limit.group_aids.contains(&v))
}
