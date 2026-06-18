use crate::{
    ad::AItemId,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UFit, UItemId, UShip},
    util::RSet,
};

/// Fails when a ship which can't have a stance but has one.
pub struct ValShipStanceFail {
    /// Stance item ID.
    pub stance_item_id: ItemId,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_ship_stance_fast(
        &self,
        kfs: &RSet<UItemId>,
        fit: &UFit,
        ship: Option<&UShip>,
    ) -> bool {
        let Some(stance_uid) = fit.stance else {
            return true;
        };
        let Some(ship) = ship else {
            return false;
        };
        stanceable_matcher(ship) || kfs.contains(&stance_uid)
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_ship_stance_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        fit: &UFit,
        ship: Option<&UShip>,
    ) -> Option<ValShipStanceFail> {
        let stance_uid = fit.stance?;
        let Some(ship) = ship else {
            return Some(ValShipStanceFail {
                stance_item_id: ctx.u_data.items.ext_id_by_int_id(stance_uid),
            });
        };
        if stanceable_matcher(ship) {
            return None;
        }
        if kfs.contains(&stance_uid) {
            return None;
        }
        Some(ValShipStanceFail {
            stance_item_id: ctx.u_data.items.ext_id_by_int_id(stance_uid),
        })
    }
}

fn stanceable_matcher(ship: &UShip) -> bool {
    matches!(
        ship.get_type_aid(),
        AItemId::CONFESSOR | AItemId::HECATE | AItemId::JACKDAW | AItemId::SVIPUL | AItemId::SKUA | AItemId::ANHINGA
    )
}
