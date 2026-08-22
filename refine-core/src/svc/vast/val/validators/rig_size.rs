use crate::{
    ItemId, Value,
    svc::{SvcCtx, vast::VastFitData},
    ud::{UItemId, UShip},
    util::RSet,
};

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde_tuple::Serialize_tuple)
)]
#[derive(Clone)]
pub struct ValRigSizeFail {
    /// Rig size compatible with the ship.
    pub allowed_size: Value,
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub rig_sizes: Vec<ValRigSizeItemInfo>,
}

#[cfg_attr(feature = "serde", derive(refine_serde::VecAsMapEntry))]
#[derive(Copy, Clone)]
pub struct ValRigSizeItemInfo {
    /// Rig which failed the validation.
    #[cfg_attr(feature = "serde", vec_map(key))]
    pub rig_id: ItemId,
    /// Size of the rig.
    #[cfg_attr(feature = "serde", vec_map(value))]
    pub size: Option<Value>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_rig_size_fast(&self, kfs: &RSet<UItemId>, ship: Option<&UShip>) -> bool {
        let Some(allowed_size) = get_allowed_size(ship) else {
            return true;
        };
        for (rig_uid, &rig_size) in self.rigs_rig_size.iter() {
            if rig_size != Some(allowed_size) && !kfs.contains(rig_uid) {
                return false;
            }
        }
        true
    }
    // Verbose validations
    pub(in crate::svc::vast::val) fn validate_rig_size_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
        ship: Option<&UShip>,
    ) -> Option<ValRigSizeFail> {
        let allowed_size = get_allowed_size(ship)?;
        let mut rig_sizes = Vec::new();
        for (rig_uid, &rig_size) in self.rigs_rig_size.iter() {
            if rig_size != Some(allowed_size) && !kfs.contains(rig_uid) {
                rig_sizes.push(ValRigSizeItemInfo {
                    rig_id: ctx.u_data.items.ext_id_by_int_id(*rig_uid),
                    size: rig_size,
                });
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
    ship?.get_r_item_attr_data()?.rig_size
}
