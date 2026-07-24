use crate::{
    EffectId, ItemId, SecZone, SecZoneCorruption,
    rd::REffectId,
    svc::{SvcCtx, vast::VastFitData},
    ud::{UData, UItemId},
    util::RSet,
};

#[derive(Copy, Clone)]
pub(in crate::svc::vast) struct EffectSecZoneInfo {
    pub(in crate::svc::vast) banned_in_hisec: bool,
    pub(in crate::svc::vast) banned_in_lowsec: bool,
}

#[cfg_attr(feature = "serde", derive(serde_tuple::Serialize_tuple))]
pub struct ValEffectSecZoneFail {
    /// Solar system security zone.
    pub zone: SecZone,
    /// Map between IDs of items+effects which cannot be used in current security zone, and a list
    /// of security zones they can be used in.
    #[cfg_attr(feature = "serde", serde(serialize_with = "custom_serde::as_nested_map"))]
    pub items: Vec<ValEffectSecZoneItemInfo>,
}

pub struct ValEffectSecZoneItemInfo {
    /// Item which fails validation.
    pub item_id: ItemId,
    /// Effects which fail validation with extra info.
    pub effects: Vec<ValEffectSecZoneEffectInfo>,
}

pub struct ValEffectSecZoneEffectInfo {
    /// Effect which cannot be used in current security zone.
    pub effect_id: EffectId,
    /// Security zones the effect can be used in.
    pub allowed_zones: Vec<SecZone>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_sec_zone_effect_fast(&self, kfs: &RSet<UItemId>, ctx: SvcCtx) -> bool {
        if self.sec_zone_effect.is_empty() {
            return true;
        }
        match ctx.u_data.sec_zone {
            SecZone::HiSec(_) => {
                'items: for (item_uid, item_data) in self.sec_zone_effect.iter() {
                    for sec_zone_info in item_data.values() {
                        if sec_zone_info.banned_in_hisec {
                            match kfs.contains(item_uid) {
                                true => continue 'items,
                                false => return false,
                            }
                        }
                    }
                }
                true
            }
            SecZone::LowSec(_) => {
                'items: for (item_uid, item_data) in self.sec_zone_effect.iter() {
                    for sec_zone_info in item_data.values() {
                        if sec_zone_info.banned_in_lowsec {
                            match kfs.contains(item_uid) {
                                true => continue 'items,
                                false => return false,
                            }
                        }
                    }
                }
                true
            }
            SecZone::NullSec | SecZone::WSpace | SecZone::Hazard => true,
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_sec_zone_effect_verbose(
        &self,
        kfs: &RSet<UItemId>,
        ctx: SvcCtx,
    ) -> Option<ValEffectSecZoneFail> {
        if self.sec_zone_effect.is_empty() {
            return None;
        }
        let mut items = Vec::new();
        match ctx.u_data.sec_zone {
            SecZone::HiSec(_) => {
                'items: for (&item_uid, item_data) in self.sec_zone_effect.iter() {
                    let mut effects = Vec::new();
                    for (&effect_rid, sec_zone_info) in item_data.iter() {
                        if sec_zone_info.banned_in_hisec {
                            match kfs.contains(&item_uid) {
                                true => continue 'items,
                                false => add_failed_effect(ctx.u_data, &mut effects, effect_rid, sec_zone_info),
                            }
                        }
                    }
                    if !effects.is_empty() {
                        let item_id = ctx.u_data.items.ext_id_by_int_id(item_uid);
                        items.push(ValEffectSecZoneItemInfo { item_id, effects });
                    }
                }
            }
            SecZone::LowSec(_) => {
                'items: for (&item_uid, item_data) in self.sec_zone_effect.iter() {
                    let mut effects = Vec::new();
                    for (&effect_rid, sec_zone_info) in item_data.iter() {
                        if sec_zone_info.banned_in_lowsec {
                            match kfs.contains(&item_uid) {
                                true => continue 'items,
                                false => add_failed_effect(ctx.u_data, &mut effects, effect_rid, sec_zone_info),
                            }
                        }
                    }
                    if !effects.is_empty() {
                        let item_id = ctx.u_data.items.ext_id_by_int_id(item_uid);
                        items.push(ValEffectSecZoneItemInfo { item_id, effects });
                    }
                }
            }
            SecZone::NullSec | SecZone::WSpace | SecZone::Hazard => (),
        }
        match items.is_empty() {
            true => None,
            false => Some(ValEffectSecZoneFail {
                zone: ctx.u_data.sec_zone,
                items,
            }),
        }
    }
}

fn add_failed_effect(
    u_data: &UData,
    item_fails: &mut Vec<ValEffectSecZoneEffectInfo>,
    effect_rid: REffectId,
    sec_zone_info: &EffectSecZoneInfo,
) {
    let effect_id = EffectId::from_aid(u_data.r_data.get_effect_by_rid(effect_rid).aid);
    let mut allowed_zones = Vec::new();
    if !sec_zone_info.banned_in_hisec {
        allowed_zones.push(SecZone::HiSec(SecZoneCorruption::None));
    }
    if !sec_zone_info.banned_in_lowsec {
        allowed_zones.push(SecZone::LowSec(SecZoneCorruption::None));
    }
    allowed_zones.extend([SecZone::NullSec, SecZone::WSpace, SecZone::Hazard]);
    item_fails.push(ValEffectSecZoneEffectInfo {
        effect_id,
        allowed_zones,
    });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
mod custom_serde {
    use serde::ser::{Serialize, SerializeMap, Serializer};

    use super::*;

    pub(super) fn as_nested_map<S>(items: &[ValEffectSecZoneItemInfo], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(items.len()))?;
        for item in items {
            map.serialize_entry(&item.item_id, &EffectInfo(&item.effects))?;
        }
        map.end()
    }

    struct EffectInfo<'a>(&'a [ValEffectSecZoneEffectInfo]);
    impl Serialize for EffectInfo<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(self.0.len()))?;
            for effect in self.0 {
                map.serialize_entry(&effect.effect_id, &effect.allowed_zones)?;
            }
            map.end()
        }
    }
}
