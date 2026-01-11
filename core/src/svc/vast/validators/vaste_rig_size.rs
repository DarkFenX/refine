use std::collections::HashMap;

use crate::{
    num::Value,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId, UShip},
    util::RSet,
};

pub struct ValRigSizeFail {
    /// Rig size compatible with the ship.
    pub allowed_size: Value,
    /// Sizes of incompatible rigs.
    pub rig_sizes: HashMap<ItemId, Option<Value>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_rig_size_fast(&self, kfs: &RSet<UItemId>, ship: Option<&UShip>) -> bool {
        let allowed_size = match get_allowed_size(ship) {
            Some(allowed_size) => allowed_size,
            None => return true,
        };
        for (rig_uidd, &rig_size) in self.rigs_rig_size.iter() {
            if rig_size != Some(allowed_size) && !kfs.contains(rig_uidd) {
                return false;
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_rig_size_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        ship: Option<&UShip>,
    ) -> Option<ValRigSizeFail> {
        let allowed_size = get_allowed_size(ship)?;
        let mut rig_sizes = HashMap::new();
        for (rig_uid, &rig_size) in self.rigs_rig_size.iter() {
            if rig_size != Some(allowed_size) && !kfs.contains(rig_uid) {
                rig_sizes.insert(ctx.u_data.items.xid_by_iid(*rig_uid), rig_size);
            }
        }
        match rig_sizes.is_empty() {
            true => None,
            false => Some(ValRigSizeFail {
                allowed_size,
                rig_sizes,
            }),
        }
    }
}

fn get_allowed_size(ship: Option<&UShip>) -> Option<Value> {
    ship?.get_axt()?.rig_size
}
