use itertools::Itertools;

use crate::{
    ItemId, PValue,
    def::MAX_SUBCAP_MODULE_VOLUME,
    rd::RShipKind,
    svc::{SvcCtx, vast::VastFitData},
    ud::{UItemId, UShip},
    util::RSet,
};

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Clone)]
pub struct ValCapitalModFail {
    /// Modules up to and including this volume are not considered capital.
    pub max_subcap_volume: PValue,
    #[cfg_attr(feature = "serde", serde(serialize_with = "custom_serde::as_map"))]
    pub module_volumes: Vec<ValCapitalModInfo>,
}

/// Module which breaks the validation and its volume.
#[derive(Copy, Clone)]
pub struct ValCapitalModInfo {
    pub module_id: ItemId,
    pub volume: PValue,
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
        let module_volumes = self
            .mods_capital
            .iter()
            .filter_map(|(module_uid, module_volume)| match kfs.contains(module_uid) {
                true => None,
                false => Some(ValCapitalModInfo {
                    module_id: ctx.u_data.items.ext_id_by_int_id(*module_uid),
                    volume: *module_volume,
                }),
            })
            .collect_vec();
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
    let Some(ship) = ship else {
        return false;
    };
    matches!(ship.get_r_kind(), Some(RShipKind::Ship))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{SerializeMap, Serializer};

    use super::*;

    pub(super) fn as_map<S>(items: &[ValCapitalModInfo], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(items.len()))?;
        for item in items {
            map.serialize_entry(&item.module_id, &item.volume)?;
        }
        map.end()
    }
}
