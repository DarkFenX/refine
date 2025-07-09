use std::collections::HashMap;

use crate::{
    def::{AttrVal, ItemId, ItemKey},
    svc::{SvcCtx, vast::VastFitData},
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
    pub charge_size: Option<AttrVal>,
    /// Size value allowed by module.
    pub allowed_size: AttrVal,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_charge_size_fast(&mut self, kfs: &RSet<ItemKey>) -> bool {
        match kfs.is_empty() {
            true => self.charge_size.is_empty(),
            false => self.charge_size.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_charge_size_verbose(
        &mut self,
        kfs: &RSet<ItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValChargeSizeFail> {
        let mut charges = HashMap::new();
        for (&charge_key, &cont_key) in self.charge_size.difference(kfs) {
            charges.insert(
                ctx.uad.items.id_by_key(charge_key),
                ValChargeSizeChargeInfo {
                    parent_item_id: ctx.uad.items.id_by_key(cont_key),
                    charge_size: ctx.uad.items.get(charge_key).get_a_xt().unwrap().charge_size,
                    allowed_size: ctx.uad.items.get(cont_key).get_a_xt().unwrap().charge_size.unwrap(),
                },
            );
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeSizeFail { charges }),
        }
    }
}
