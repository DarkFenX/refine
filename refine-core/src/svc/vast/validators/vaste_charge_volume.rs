use crate::{
    ItemId, PValue,
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
pub struct ValChargeVolumeFail {
    #[cfg_attr(feature = "serde", serde_as(as = "serde_with::KeyValueMap<_>"))]
    pub charges: Vec<ValChargeVolumeChargeInfo>,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
#[derive(Copy, Clone)]
pub struct ValChargeVolumeChargeInfo {
    /// Charge item ID.
    #[cfg_attr(feature = "serde", serde(rename = "$key$"))]
    pub charge_item_id: ItemId,
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
        let mut charges = Vec::new();
        for (&charge_uid, &cont_uid) in self.charge_volume.difference(kfs) {
            charges.push(ValChargeVolumeChargeInfo {
                charge_item_id: ctx.u_data.items.ext_id_by_int_id(charge_uid),
                parent_item_id: ctx.u_data.items.ext_id_by_int_id(cont_uid),
                charge_volume: ctx.u_data.items.get(charge_uid).get_axt().unwrap().volume,
                max_volume: ctx.u_data.items.get(cont_uid).get_axt().unwrap().capacity,
            });
        }
        match charges.is_empty() {
            true => None,
            false => Some(ValChargeVolumeFail { charges }),
        }
    }
}
