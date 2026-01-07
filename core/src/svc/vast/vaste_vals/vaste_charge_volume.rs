use std::collections::HashMap;

use crate::{
    misc::PValue,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
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
    pub charge_volume: PValue,
    /// Maximum charge volume allowed by its parent module.
    pub max_volume: PValue,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_charge_volume_fast(&mut self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.charge_volume.is_empty(),
            false => self.charge_volume.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_charge_volume_verbose(
        &mut self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValChargeVolumeFail> {
        let mut charges = HashMap::new();
        for (&charge_uid, &cont_uid) in self.charge_volume.difference(kfs) {
            charges.insert(
                ctx.u_data.items.xid_by_iid(charge_uid),
                ValChargeVolumeChargeInfo {
                    parent_item_id: ctx.u_data.items.xid_by_iid(cont_uid),
                    charge_volume: ctx.u_data.items.get(charge_uid).get_axt().unwrap().volume,
                    max_volume: ctx.u_data.items.get(cont_uid).get_axt().unwrap().capacity,
                },
            );
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeVolumeFail { charges }),
        }
    }
}
