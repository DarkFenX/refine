use std::collections::HashMap;

use crate::{
    def::{ItemGrpId, ItemId},
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemId,
    util::RSet,
};

pub struct ValChargeGroupFail {
    /// Map between charge IDs and info about failed validation.
    pub charges: HashMap<ItemId, ValChargeGroupChargeInfo>,
}

pub struct ValChargeGroupChargeInfo {
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Group ID of current charge.
    pub charge_group_id: ItemGrpId,
    /// Group IDs allowed by containing module.
    pub allowed_group_ids: Vec<ItemGrpId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_charge_group_fast(&mut self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.charge_group.is_empty(),
            false => self.charge_group.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_charge_group_verbose(
        &mut self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValChargeGroupFail> {
        let mut charges = HashMap::new();
        for (&charge_key, &cont_key) in self.charge_group.difference(kfs) {
            charges.insert(
                ctx.u_data.items.ext_id_by_int_id(charge_key),
                ValChargeGroupChargeInfo {
                    parent_item_id: ctx.u_data.items.ext_id_by_int_id(cont_key),
                    charge_group_id: ctx.u_data.items.get(charge_key).get_group_id().unwrap(),
                    allowed_group_ids: ctx
                        .u_data
                        .items
                        .get(cont_key)
                        .get_axt()
                        .unwrap()
                        .charge_limit
                        .as_ref()
                        .unwrap()
                        .group_ids
                        .clone(),
                },
            );
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeGroupFail { charges }),
        }
    }
}
