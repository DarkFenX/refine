use crate::{
    consts,
    defs::SolItemId,
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
            None => return false,
        };
        matches!(
            ship.get_type_id(),
            consts::items::CONFESSOR | consts::items::HECATE | consts::items::JACKDAW | consts::items::SVIPUL
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
            None => return Some(SolValShipStanceFail { item_id: stance_id }),
        };
        if matches!(
            ship.get_type_id(),
            consts::items::CONFESSOR | consts::items::HECATE | consts::items::JACKDAW | consts::items::SVIPUL
        ) {
            return None;
        }
        if kfs.contains(&stance_id) {
            return None;
        }
        Some(SolValShipStanceFail { item_id: stance_id })
    }
}
