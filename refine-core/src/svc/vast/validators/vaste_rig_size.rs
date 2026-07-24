use crate::{
    ItemId, Value,
    svc::{SvcCtx, vast::VastFitData},
    ud::{UItemId, UShip},
    util::RSet,
};

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
pub struct ValRigSizeFail {
    /// Rig size compatible with the ship.
    pub allowed_size: Value,
    #[cfg_attr(feature = "serde", serde(serialize_with = "custom_serde::as_map"))]
    pub rig_sizes: Vec<ValRigSizeItemInfo>,
}

pub struct ValRigSizeItemInfo {
    /// Rig which failed the validation.
    pub rig_id: ItemId,
    /// Size of the rig.
    pub size: Option<Value>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_rig_size_fast(&self, kfs: &RSet<UItemId>, ship: Option<&UShip>) -> bool {
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
    pub(in crate::svc::vast) fn validate_rig_size_verbose(
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
    ship?.get_axt()?.rig_size
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{SerializeMap, Serializer};

    use super::*;

    pub(super) fn as_map<S>(items: &[ValRigSizeItemInfo], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(items.len()))?;
        for item in items {
            map.serialize_entry(&item.rig_id, &item.size)?;
        }
        map.end()
    }
}
