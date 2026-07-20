use crate::{
    api::ItemGrpId,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UItemId},
    util::RSet,
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
pub struct ValChargeParentGroupFail {
    /// Charges and info about failed validation.
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::Map<_, _>"))]
    pub charges: Vec<(ItemId, ValChargeParentGroupInfo)>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
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
        let mut charges = Vec::new();
        for (&charge_uid, &cont_uid) in self.charge_cont_group.difference(kfs) {
            charges.push((
                ctx.u_data.items.ext_id_by_int_id(charge_uid),
                ValChargeParentGroupInfo {
                    parent_item_id: ctx.u_data.items.ext_id_by_int_id(cont_uid),
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
            ));
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeParentGroupFail { charges }),
        }
    }
}
