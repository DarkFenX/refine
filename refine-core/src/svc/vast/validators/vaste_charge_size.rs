use crate::{
    num::Value,
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
pub struct ValChargeSizeFail {
    /// Charges and info about failed validation.
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::Map<_, _>"))]
    pub charges: Vec<(ItemId, ValChargeSizeChargeInfo)>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
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
        let mut charges = Vec::new();
        for (&charge_uid, &cont_uid) in self.charge_size.difference(kfs) {
            charges.push((
                ctx.u_data.items.ext_id_by_int_id(charge_uid),
                ValChargeSizeChargeInfo {
                    parent_item_id: ctx.u_data.items.ext_id_by_int_id(cont_uid),
                    charge_size: ctx.u_data.items.get(charge_uid).get_axt().unwrap().charge_size,
                    allowed_size: ctx.u_data.items.get(cont_uid).get_axt().unwrap().charge_size.unwrap(),
                },
            ));
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeSizeFail { charges }),
        }
    }
}
