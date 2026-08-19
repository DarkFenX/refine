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

#[cfg_attr(
    feature = "serde",
    cfg_eval,
    serde_with::serde_as,
    derive(serde_tuple::Serialize_tuple)
)]
#[derive(Clone)]
pub struct ValEffectSecZoneFail {
    /// Solar system security zone.
    pub zone: SecZone,
    /// Map between IDs of items+effects which cannot be used in current security zone, and a list
    /// of security zones they can be used in.
    #[cfg_attr(feature = "serde", serde_as(as = "refine_serde::VecAsMap"))]
    pub items: Vec<ValEffectSecZoneItemInfo>,
}

#[cfg_attr(feature = "serde", derive(refine_serde::VecAsMapEntry))]
#[derive(Clone)]
pub struct ValEffectSecZoneItemInfo {
    /// Item which fails validation.
    #[cfg_attr(feature = "serde", vec_map(key))]
    pub item_id: ItemId,
    /// Effects which fail validation with extra info.
    #[cfg_attr(feature = "serde", vec_map(value, serialize_as = "refine_serde::VecAsMap"))]
    pub effects: Vec<ValEffectSecZoneEffectInfo>,
}

#[cfg_attr(feature = "serde", derive(refine_serde::VecAsMapEntry))]
#[derive(Clone)]
pub struct ValEffectSecZoneEffectInfo {
    /// Effect which cannot be used in current security zone.
    #[cfg_attr(feature = "serde", vec_map(key))]
    pub effect_id: EffectId,
    /// Security zones the effect can be used in.
    #[cfg_attr(feature = "serde", vec_map(value))]
    pub allowed_zones: Vec<SecZone>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_sec_zone_effect_fast(&self, kfs: &RSet<UItemId>, ctx: SvcCtx) -> bool {
        if self.sec_zone_effect.is_empty() {
            return true;
        }
        match ctx.u_data.sec_zone {
            SecZone::HiSec(..) => {
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
            SecZone::LowSec(..) => {
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
            SecZone::HiSec(..) => {
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
            SecZone::LowSec(..) => {
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
