use crate::{
    ItemId, Value,
    svc::{SvcCtx, vast::VastFitData},
    ud::{UData, UFit, UItemId, UShip},
    util::RSet,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Result
////////////////////////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Validators
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast::val) fn validate_rig_size_fast(&self, kfs: &RSet<UItemId>) -> bool {
        match kfs.is_empty() {
            true => self.rig_size_mismatches.is_empty(),
            false => self.rig_size_mismatches.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast::val) fn validate_rig_size_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: &SvcCtx,
        ship: Option<&UShip>,
    ) -> Option<ValRigSizeFail> {
        let allowed_size = get_allowed_size(ship)?;
        let mut rig_sizes = Vec::new();
        for rig_uid in self.rig_size_mismatches.iter() {
            if !kfs.contains(rig_uid) {
                rig_sizes.push(ValRigSizeItemInfo {
                    rig_id: ctx.u_data.items.ext_id_by_int_id(*rig_uid),
                    size: *self.rigs_rig_size.get(rig_uid).unwrap(),
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
    ship?.get_r_item_base()?.rig_size
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
impl VastFitData {
    pub(in crate::svc::vast) fn refresh_rig_size_mismatches(&mut self, allowed_size: Option<Value>) {
        self.rig_size_mismatches.clear();
        for (&rig_uid, &rig_size) in self.rigs_rig_size.iter() {
            if is_rig_size_mismatch(rig_size, allowed_size) {
                self.rig_size_mismatches.insert(rig_uid);
            }
        }
    }
}

// Rig size is limited only by loaded ships which have it defined
pub(in crate::svc::vast) fn get_fit_ship_rig_size(u_data: &UData, fit: &UFit) -> Option<Value> {
    u_data.items.get(fit.ship?).get_r_item_base()?.rig_size
}

pub(in crate::svc::vast) fn is_rig_size_mismatch(rig_size: Option<Value>, allowed_size: Option<Value>) -> bool {
    allowed_size.is_some() && rig_size != allowed_size
}
