use std::collections::HashMap;

use crate::{
    ac, ad,
    sol::{
        AttrVal, ItemId, ItemKey,
        svc::vast::VastFitData,
        uad::{Uad, item::Ship},
    },
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
        kfs: &RSet<ItemKey>,
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
        kfs: &RSet<ItemKey>,
        uad: &Uad,
        ship: Option<&Ship>,
    ) -> Option<ValCapitalModFail> {
        if !is_ship_subcap(ship) {
            return None;
        }
        let module_volumes: HashMap<_, _> = self
            .mods_capital
            .iter()
            .filter(|(module_key, _)| !kfs.contains(module_key))
            .map(|(module_key, module_volume)| (uad.items.id_by_key(*module_key), *module_volume))
            .collect();
        match module_volumes.is_empty() {
            true => None,
            false => Some(ValCapitalModFail {
                max_subcap_volume: ac::extras::MAX_SUBCAP_MODULE_VOLUME,
                module_volumes,
            }),
        }
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
