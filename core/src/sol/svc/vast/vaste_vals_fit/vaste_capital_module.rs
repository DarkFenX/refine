use std::collections::HashMap;

use crate::{
    ac, ad,
    sol::{AttrVal, ItemId, svc::vast::VastFitData, uad::item::Ship},
    util::RSet,
};

pub struct ValCapitalModFail {
    /// Modules up to and including this volume are not considered capital.
    pub max_subcap_volume: AttrVal,
    /// List of modules breaking validation, and their volumes.
    pub module_volumes: HashMap<ItemId, AttrVal>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_capital_module_fast(
        &self,
        kfs: &RSet<ItemId>,
        ship: Option<&Ship>,
    ) -> bool {
        if !is_ship_subcap(ship) {
            return true;
        }
        match kfs.is_empty() {
            true => self.mods_capital.is_empty(),
            false => self.mods_capital.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_capital_module_verbose(
        &self,
        kfs: &RSet<ItemId>,
        ship: Option<&Ship>,
    ) -> Option<ValCapitalModFail> {
        if !is_ship_subcap(ship) {
            return None;
        }
        let items: HashMap<_, _> = self
            .mods_capital
            .iter()
            .filter(|(k, _)| !kfs.contains(k))
            .map(|(k, v)| (*k, *v))
            .collect();
        if items.is_empty() {
            return None;
        }
        Some(ValCapitalModFail {
            max_subcap_volume: ac::extras::MAX_SUBCAP_MODULE_VOLUME,
            module_volumes: items,
        })
    }
}

fn is_ship_subcap(ship: Option<&Ship>) -> bool {
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
