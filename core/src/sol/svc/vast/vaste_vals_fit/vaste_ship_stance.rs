use crate::{
    defs::SolItemId,
    ec,
    sol::{
        svc::vast::SolVastFitData,
        uad::{fit::SolFit, item::SolShip},
    },
    util::StSet,
};

pub struct SolValShipStanceFail {
    pub item_id: SolItemId,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_ship_stance_fast(
        &self,
        kfs: &StSet<SolItemId>,
        fit: &SolFit,
        ship: Option<&SolShip>,
    ) -> bool {
        let stance_id = match fit.stance {
            Some(stance_id) => stance_id,
            None => return true,
        };
        let ship = match ship {
            Some(ship) => ship,
            None => return true,
        };
        matches!(
            ship.get_type_id(),
            ec::items::CONFESSOR | ec::items::HECATE | ec::items::JACKDAW | ec::items::SVIPUL
        ) || kfs.contains(&stance_id)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_ship_stance_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        fit: &SolFit,
        ship: Option<&SolShip>,
    ) -> Option<SolValShipStanceFail> {
        let stance_id = match fit.stance {
            Some(stance_id) => stance_id,
            None => return None,
        };
        let ship = match ship {
            Some(ship) => ship,
            None => return None,
        };
        if matches!(
            ship.get_type_id(),
            ec::items::CONFESSOR | ec::items::HECATE | ec::items::JACKDAW | ec::items::SVIPUL
        ) {
            return None;
        }
        if kfs.contains(&stance_id) {
            return None;
        }
        Some(SolValShipStanceFail { item_id: stance_id })
    }
}
