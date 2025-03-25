use crate::{
    ac,
    sol::{
        ItemId,
        svc::vast::VastFitData,
        uad::{fit::Fit, item::Ship},
    },
    util::StSet,
};

pub struct ValShipStanceFail {
    pub item_id: ItemId,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_ship_stance_fast(
        &self,
        kfs: &StSet<ItemId>,
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
        kfs: &StSet<ItemId>,
        fit: &Fit,
        ship: Option<&Ship>,
    ) -> Option<ValShipStanceFail> {
        let stance_id = match fit.stance {
            Some(stance_id) => stance_id,
            None => return None,
        };
        let ship = match ship {
            Some(ship) => ship,
            None => return Some(ValShipStanceFail { item_id: stance_id }),
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
        Some(ValShipStanceFail { item_id: stance_id })
    }
}
