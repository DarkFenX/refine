use std::collections::HashMap;

use crate::{
    api::ItemGrpId,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::RSet,
};

pub struct ValChargeParentGroupFail {
    /// Map between charge IDs and info about failed validation.
    pub charges: HashMap<ItemId, ValChargeParentGroupInfo>,
}

pub struct ValChargeParentGroupInfo {
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Parent module group ID.
    pub parent_group_id: ItemGrpId,
    /// Group IDs allowed by charge.
    pub allowed_group_ids: Vec<ItemGrpId>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_charge_cont_group_fast(&mut self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.charge_cont_group.is_empty(),
            false => self.charge_cont_group.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_charge_cont_group_verbose(
        &mut self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValChargeParentGroupFail> {
        let mut charges = HashMap::new();
        for (&charge_uid, &cont_uid) in self.charge_cont_group.difference(kfs) {
            charges.insert(
                ctx.u_data.items.xid_by_iid(charge_uid),
                ValChargeParentGroupInfo {
                    parent_item_id: ctx.u_data.items.xid_by_iid(cont_uid),
                    parent_group_id: ItemGrpId::from_aid(ctx.u_data.items.get(cont_uid).get_group_id().unwrap()),
                    allowed_group_ids: ctx
                        .u_data
                        .items
                        .get(charge_uid)
                        .get_axt()
                        .unwrap()
                        .cont_limit
                        .as_ref()
                        .unwrap()
                        .group_ids
                        .iter()
                        .map(|&grp_aid| ItemGrpId::from_aid(grp_aid))
                        .collect(),
                },
            );
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeParentGroupFail { charges }),
        }
    }
}
