use std::collections::HashMap;

use crate::{
    misc::Value,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::RSet,
};

pub struct ValChargeSizeFail {
    /// Map between charge IDs and info about failed validation.
    pub charges: HashMap<ItemId, ValChargeSizeChargeInfo>,
}

pub struct ValChargeSizeChargeInfo {
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Size attribute value of current charge.
    pub charge_size: Option<Value>,
    /// Size value allowed by module.
    pub allowed_size: Value,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_charge_size_fast(&mut self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.charge_size.is_empty(),
            false => self.charge_size.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_charge_size_verbose(
        &mut self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValChargeSizeFail> {
        let mut charges = HashMap::new();
        for (&charge_uid, &cont_uid) in self.charge_size.difference(kfs) {
            charges.insert(
                ctx.u_data.items.xid_by_iid(charge_uid),
                ValChargeSizeChargeInfo {
                    parent_item_id: ctx.u_data.items.xid_by_iid(cont_uid),
                    charge_size: ctx.u_data.items.get(charge_uid).get_axt().unwrap().charge_size,
                    allowed_size: ctx.u_data.items.get(cont_uid).get_axt().unwrap().charge_size.unwrap(),
                },
            );
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeSizeFail { charges }),
        }
    }
}
