use crate::{
    ItemId,
    ad::AItemId,
    svc::{SvcCtx, vast::VastFitData},
    ud::{UFit, UItemId, UShip},
    util::RSet,
    val::ValResult,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Result
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Fails when a ship which can't have a stance but has one.
#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ValShipStanceFail {
    /// Stance item ID.
    pub stance_item_id: ItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Validators
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_ship_stance_fast(
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
    pub(in crate::svc::vast::val) fn validate_ship_stance_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        fit: &UFit,
        ship: Option<&UShip>,
    ) -> ValResult<ValShipStanceFail> {
        let Some(stance_uid) = fit.stance else {
            return ValResult::Passed;
        };
        // Stance is the only item which can fail, so it has no status - validation result tells if
        // it is a known failure. Without ship, known failures are not considered
        let is_kf = match ship {
            Some(ship) => match stanceable_matcher(ship) {
                true => return ValResult::Passed,
                false => kfs.contains(&stance_uid),
            },
            None => false,
        };
        ValResult::from_fail(
            is_kf,
            ValShipStanceFail {
                stance_item_id: ctx.u_data.items.ext_id_by_int_id(stance_uid),
            },
        )
    }
}

fn stanceable_matcher(ship: &UShip) -> bool {
    matches!(
        ship.get_type_aid(),
        AItemId::CONFESSOR | AItemId::HECATE | AItemId::JACKDAW | AItemId::SVIPUL | AItemId::SKUA | AItemId::ANHINGA
    )
}
