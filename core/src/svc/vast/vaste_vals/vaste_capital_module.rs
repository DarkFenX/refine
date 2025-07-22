use std::collections::HashMap;

use crate::{
    ac, ad,
    def::{AttrVal, ItemId},
    svc::{SvcCtx, vast::VastFitData},
    uad::{UadItemKey, UadShip},
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
    pub(in crate::svc::vast) fn validate_capital_module_fast(
        &self,
        kfs: &RSet<UadItemKey>,
        ship: Option<&UadShip>,
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
    pub(in crate::svc::vast) fn validate_capital_module_verbose(
        &self,
        kfs: &RSet<UadItemKey>,
        ctx: SvcCtx,
        ship: Option<&UadShip>,
    ) -> Option<ValCapitalModFail> {
        if !is_ship_subcap(ship) {
            return None;
        }
        let module_volumes: HashMap<_, _> = self
            .mods_capital
            .iter()
            .filter(|(module_key, _)| !kfs.contains(module_key))
            .map(|(module_key, module_volume)| (ctx.uad.items.id_by_key(*module_key), *module_volume))
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

fn is_ship_subcap(ship: Option<&UadShip>) -> bool {
    let ship = match ship {
        Some(ship) => ship,
        None => return false,
    };
    let a_item_xt = match ship.get_a_xt() {
        Some(extras) => extras,
        None => return false,
    };
    matches!(a_item_xt.ship_kind, Some(ad::AShipKind::Ship))
}
