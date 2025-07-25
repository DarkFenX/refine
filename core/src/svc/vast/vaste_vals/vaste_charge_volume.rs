use std::collections::HashMap;

use crate::{
    def::{AttrVal, ItemId},
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemKey,
    util::RSet,
};

pub struct ValChargeVolumeFail {
    /// Map between charge IDs and info about failed validation.
    pub charges: HashMap<ItemId, ValChargeVolumeChargeInfo>,
}

pub struct ValChargeVolumeChargeInfo {
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Volume of current charge.
    pub charge_volume: AttrVal,
    /// Maximum charge volume allowed by its parent module.
    pub max_volume: AttrVal,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_charge_volume_fast(&mut self, kfs: &RSet<UItemKey>) -> bool {
        match kfs.is_empty() {
            true => self.charge_volume.is_empty(),
            false => self.charge_volume.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_charge_volume_verbose(
        &mut self,
        kfs: &RSet<UItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValChargeVolumeFail> {
        let mut charges = HashMap::new();
        for (&charge_key, &cont_key) in self.charge_volume.difference(kfs) {
            charges.insert(
                ctx.u_data.items.id_by_key(charge_key),
                ValChargeVolumeChargeInfo {
                    parent_item_id: ctx.u_data.items.id_by_key(cont_key),
                    charge_volume: ctx.u_data.items.get(charge_key).get_r_axt().unwrap().volume,
                    max_volume: ctx.u_data.items.get(cont_key).get_r_axt().unwrap().capacity,
                },
            );
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeVolumeFail { charges }),
        }
    }
}
