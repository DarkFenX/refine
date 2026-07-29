use crate::{
    ItemId, Value,
    svc::{SvcCtx, vast::VastFitData},
    ud::UItemId,
    util::RSet,
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde::Serialize),
    serde(transparent)
)]
#[derive(Clone)]
pub struct ValChargeSizeFail {
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::KeyValueMap<_>"))]
    pub charges: Vec<ValChargeSizeChargeInfo>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ValChargeSizeChargeInfo {
    /// Charge item ID.
    #[cfg_attr(feature = "serde", serde(rename = "$key$"))]
    pub charge_item_id: ItemId,
    /// Parent module item ID.
    pub parent_item_id: ItemId,
    /// Size attribute value of current charge.
    pub charge_size: Option<Value>,
    /// Size value allowed by module.
    pub allowed_size: Value,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_charge_size_fast(&self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.charge_size.is_empty(),
            false => self.charge_size.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_charge_size_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValChargeSizeFail> {
        let mut charges = Vec::new();
        for (&charge_uid, &cont_uid) in self.charge_size.difference(kfs) {
            charges.push(ValChargeSizeChargeInfo {
                charge_item_id: ctx.u_data.items.ext_id_by_int_id(charge_uid),
                parent_item_id: ctx.u_data.items.ext_id_by_int_id(cont_uid),
                charge_size: ctx
                    .u_data
                    .items
                    .get(charge_uid)
                    .get_r_item_attr_data()
                    .unwrap()
                    .charge_size,
                allowed_size: ctx
                    .u_data
                    .items
                    .get(cont_uid)
                    .get_r_item_attr_data()
                    .unwrap()
                    .charge_size
                    .unwrap(),
            });
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeSizeFail { charges }),
        }
    }
}
