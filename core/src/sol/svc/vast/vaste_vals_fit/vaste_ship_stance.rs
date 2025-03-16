use crate::{
    defs::{EItemId, SolItemId},
    ec,
    sol::{
        svc::vast::SolVastFitData,
        uad::{SolUad, fit::SolFit, item::SolShip},
    },
    util::StSet,
};

pub struct SolValShipStanceFail {
    pub ship_id: SolItemId,
    pub stance: Option<SolValShipStanceItemInfo>,
    pub stance_type_ids: Vec<EItemId>,
}

pub struct SolValShipStanceItemInfo {
    pub item_id: SolItemId,
    pub type_id: EItemId,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_ship_stance_fast(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        fit: &SolFit,
        ship: Option<&SolShip>,
    ) -> bool {
        let ship = match ship {
            Some(ship) => ship,
            None => return true,
        };
        match fit.stance {
            Some(stance_id) => match ship.get_type_id() {
                ec::items::CONFESSOR => {
                    matches!(
                        uad.items.get_item(&stance_id).unwrap().get_type_id(),
                        ec::items::CONFESSOR_DEFENSE_MODE
                            | ec::items::CONFESSOR_PROPULSION_MODE
                            | ec::items::CONFESSOR_SHARPSHOOTER_MODE
                    ) || kfs.contains(&ship.get_id())
                }
                ec::items::HECATE => {
                    matches!(
                        uad.items.get_item(&stance_id).unwrap().get_type_id(),
                        ec::items::HECATE_DEFENSE_MODE
                            | ec::items::HECATE_PROPULSION_MODE
                            | ec::items::HECATE_SHARPSHOOTER_MODE
                    ) || kfs.contains(&ship.get_id())
                }
                ec::items::JACKDAW => {
                    matches!(
                        uad.items.get_item(&stance_id).unwrap().get_type_id(),
                        ec::items::JACKDAW_DEFENSE_MODE
                            | ec::items::JACKDAW_PROPULSION_MODE
                            | ec::items::JACKDAW_SHARPSHOOTER_MODE
                    ) || kfs.contains(&ship.get_id())
                }
                ec::items::SVIPUL => {
                    matches!(
                        uad.items.get_item(&stance_id).unwrap().get_type_id(),
                        ec::items::SVIPUL_DEFENSE_MODE
                            | ec::items::SVIPUL_PROPULSION_MODE
                            | ec::items::SVIPUL_SHARPSHOOTER_MODE
                    ) || kfs.contains(&ship.get_id())
                }
                _ => kfs.contains(&ship.get_id()),
            },
            None => match ship.get_type_id() {
                ec::items::CONFESSOR | ec::items::HECATE | ec::items::JACKDAW | ec::items::SVIPUL => {
                    kfs.contains(&ship.get_id())
                }
                _ => true,
            },
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_ship_stance_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        uad: &SolUad,
        fit: &SolFit,
        ship: Option<&SolShip>,
    ) -> Option<SolValShipStanceFail> {
        let ship = match ship {
            Some(ship) => ship,
            None => return None,
        };
        let stance_info = fit.stance.map(|v| SolValShipStanceItemInfo {
            item_id: v,
            type_id: uad.items.get_item(&v).unwrap().get_type_id(),
        });
        let needed_type_ids = match ship.get_type_id() {
            ec::items::CONFESSOR => vec![
                ec::items::CONFESSOR_DEFENSE_MODE,
                ec::items::CONFESSOR_PROPULSION_MODE,
                ec::items::CONFESSOR_SHARPSHOOTER_MODE,
            ],
            ec::items::HECATE => vec![
                ec::items::HECATE_DEFENSE_MODE,
                ec::items::HECATE_PROPULSION_MODE,
                ec::items::HECATE_SHARPSHOOTER_MODE,
            ],
            ec::items::JACKDAW => vec![
                ec::items::JACKDAW_DEFENSE_MODE,
                ec::items::JACKDAW_PROPULSION_MODE,
                ec::items::JACKDAW_SHARPSHOOTER_MODE,
            ],
            ec::items::SVIPUL => vec![
                ec::items::SVIPUL_DEFENSE_MODE,
                ec::items::SVIPUL_PROPULSION_MODE,
                ec::items::SVIPUL_SHARPSHOOTER_MODE,
            ],
            _ => Vec::new(),
        };
        match stance_info {
            Some(stance_info) => {
                if needed_type_ids.contains(&stance_info.type_id) || kfs.contains(&ship.get_id()) {
                    return None;
                }
            }
            None => {
                if needed_type_ids.is_empty() || kfs.contains(&ship.get_id()) {
                    return None;
                }
            }
        }
        Some(SolValShipStanceFail {
            ship_id: ship.get_id(),
            stance: None,
            stance_type_ids: needed_type_ids,
        })
    }
}
