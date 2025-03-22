use itertools::Itertools;

use crate::{
    ad, consts,
    defs::{AttrVal, SolItemId},
    sol::{svc::vast::SolVastFitData, uad::item::SolShip},
    util::StSet,
};

pub struct SolValCapitalModFail {
    pub max_subcap_volume: AttrVal,
    pub items: Vec<SolValCapitalModItemInfo>,
}

pub struct SolValCapitalModItemInfo {
    pub item_id: SolItemId,
    pub volume: AttrVal,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_capital_module_fast(
        &self,
        kfs: &StSet<SolItemId>,
        ship: Option<&SolShip>,
    ) -> bool {
        if !is_ship_subcap(ship) {
            return true;
        }
        match kfs.is_empty() {
            true => self.mods_capital.is_empty(),
            false => self.mods_capital.difference(kfs).nth(0).is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_capital_module_verbose(
        &self,
        kfs: &StSet<SolItemId>,
        ship: Option<&SolShip>,
    ) -> Option<SolValCapitalModFail> {
        if !is_ship_subcap(ship) {
            return None;
        }
        let items = self
            .mods_capital
            .iter()
            .filter(|(k, _)| !kfs.contains(k))
            .map(|(k, v)| SolValCapitalModItemInfo {
                item_id: *k,
                volume: *v,
            })
            .collect_vec();
        if items.is_empty() {
            return None;
        }
        Some(SolValCapitalModFail {
            max_subcap_volume: consts::extras::MAX_SUBCAP_MODULE_VOLUME,
            items,
        })
    }
}

fn is_ship_subcap(ship: Option<&SolShip>) -> bool {
    let ship = match ship {
        Some(ship) => ship,
        None => return false,
    };
    let extras = match ship.get_a_extras() {
        Some(extras) => extras,
        None => return false,
    };
    matches!(extras.ship_kind, Some(ad::AShipKind::Ship))
}
