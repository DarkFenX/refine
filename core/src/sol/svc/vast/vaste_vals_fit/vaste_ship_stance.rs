use crate::{
    ac,
    sol::{
        ItemId,
        svc::vast::VastFitData,
        uad::{fit::Fit, item::Ship},
    },
    util::RSet,
};

/// Fails when a ship which can't have a stance has one.
pub struct ValShipStanceFail {
    /// Stance item ID.
    pub stance_item_id: ItemId,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_ship_stance_fast(
        &self,
        kfs: &RSet<ItemId>,
        fit: &Fit,
        ship: Option<&Ship>,
    ) -> bool {
        let stance_id = match fit.stance {
            Some(stance_id) => stance_id,
            None => return true,
        };
        let ship = match ship {
            Some(ship) => ship,
            None => return false,
        };
        matches!(
            ship.get_a_item_id(),
            ac::items::CONFESSOR | ac::items::HECATE | ac::items::JACKDAW | ac::items::SVIPUL
        ) || kfs.contains(&stance_id)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_ship_stance_verbose(
        &self,
        kfs: &RSet<ItemId>,
        fit: &Fit,
        ship: Option<&Ship>,
    ) -> Option<ValShipStanceFail> {
        let stance_id = fit.stance?;
        let ship = match ship {
            Some(ship) => ship,
            None => {
                return Some(ValShipStanceFail {
                    stance_item_id: stance_id,
                });
            }
        };
        if matches!(
            ship.get_a_item_id(),
            ac::items::CONFESSOR | ac::items::HECATE | ac::items::JACKDAW | ac::items::SVIPUL
        ) {
            return None;
        }
        if kfs.contains(&stance_id) {
            return None;
        }
        Some(ValShipStanceFail {
            stance_item_id: stance_id,
        })
    }
}
