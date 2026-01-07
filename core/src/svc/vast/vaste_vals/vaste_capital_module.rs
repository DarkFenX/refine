use std::collections::HashMap;

use crate::{
    def::MAX_SUBCAP_MODULE_VOLUME,
    misc::PValue,
    rd::RShipKind,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId, UShip},
    util::RSet,
};

pub struct ValCapitalModFail {
    /// Modules up to and including this volume are not considered capital.
    pub max_subcap_volume: PValue,
    /// List of modules breaking validation, and their volumes.
    pub module_volumes: HashMap<ItemId, PValue>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_capital_module_fast(&self, kfs: &RSet<UItemId>, ship: Option<&UShip>) -> bool {
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
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        ship: Option<&UShip>,
    ) -> Option<ValCapitalModFail> {
        if !is_ship_subcap(ship) {
            return None;
        }
        let module_volumes: HashMap<_, _> = self
            .mods_capital
            .iter()
            .filter(|(module_uid, _)| !kfs.contains(module_uid))
            .map(|(module_uid, module_volume)| (ctx.u_data.items.xid_by_iid(*module_uid), *module_volume))
            .collect();
        match module_volumes.is_empty() {
            true => None,
            false => Some(ValCapitalModFail {
                max_subcap_volume: PValue::from_f64_clamped(MAX_SUBCAP_MODULE_VOLUME),
                module_volumes,
            }),
        }
    }
}

fn is_ship_subcap(ship: Option<&UShip>) -> bool {
    let ship = match ship {
        Some(ship) => ship,
        None => return false,
    };
    matches!(ship.get_r_kind(), Some(RShipKind::Ship))
}
