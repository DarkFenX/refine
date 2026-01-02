use std::collections::HashMap;

use crate::{
    api::EffectId,
    def::ItemId,
    misc::{SecZone, SecZoneCorruption},
    rd::REffectKey,
    svc::{SvcCtx, vast::VastFitData},
    ud::{UData, UItemId},
    util::RSet,
};

#[derive(Copy, Clone)]
pub(in crate::svc::vast) struct EffectSecZoneInfo {
    pub(in crate::svc::vast) banned_in_hisec: bool,
    pub(in crate::svc::vast) banned_in_lowsec: bool,
}

pub struct ValEffectSecZoneFail {
    /// Solar system security zone.
    pub zone: SecZone,
    /// Map between IDs of items+effects which cannot be used in current security zone, and a list
    /// of security zones they can be used in.
    pub items: HashMap<ItemId, HashMap<EffectId, Vec<SecZone>>>,
}

impl VastFitData {
    // Fast validations
    pub(in crate::svc::vast) fn validate_sec_zone_effect_fast(&self, kfs: &RSet<UItemId>, ctx: SvcCtx) -> bool {
        if self.sec_zone_effect.is_empty() {
            return true;
        }
        match ctx.u_data.sec_zone {
            SecZone::HiSec(_) => {
                'items: for (item_key, item_data) in self.sec_zone_effect.iter() {
                    for sec_zone_info in item_data.values() {
                        if sec_zone_info.banned_in_hisec {
                            match kfs.contains(item_key) {
                                true => continue 'items,
                                false => return false,
                            }
                        }
                    }
                }
                true
            }
            SecZone::LowSec(_) => {
                'items: for (item_key, item_data) in self.sec_zone_effect.iter() {
                    for sec_zone_info in item_data.values() {
                        if sec_zone_info.banned_in_lowsec {
                            match kfs.contains(item_key) {
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
        let mut items: HashMap<_, HashMap<_, _>> = HashMap::new();
        match ctx.u_data.sec_zone {
            SecZone::HiSec(_) => {
                'items: for (&item_key, item_data) in self.sec_zone_effect.iter() {
                    for (&effect_key, sec_zone_info) in item_data.iter() {
                        if sec_zone_info.banned_in_hisec {
                            match kfs.contains(&item_key) {
                                true => continue 'items,
                                false => add_fail_entry(ctx.u_data, &mut items, item_key, effect_key, sec_zone_info),
                            }
                        }
                    }
                }
            }
            SecZone::LowSec(_) => {
                'items: for (&item_key, item_data) in self.sec_zone_effect.iter() {
                    for (&effect_key, sec_zone_info) in item_data.iter() {
                        if sec_zone_info.banned_in_lowsec {
                            match kfs.contains(&item_key) {
                                true => continue 'items,
                                false => add_fail_entry(ctx.u_data, &mut items, item_key, effect_key, sec_zone_info),
                            }
                        }
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

fn add_fail_entry(
    u_data: &UData,
    items: &mut HashMap<ItemId, HashMap<EffectId, Vec<SecZone>>>,
    item_key: UItemId,
    effect_key: REffectKey,
    sec_zone_info: &EffectSecZoneInfo,
) {
    let item_id = u_data.items.ext_id_by_int_id(item_key);
    let effect_id = u_data.src.get_effect(effect_key).id.into();
    let mut allowed_zones = Vec::new();
    if !sec_zone_info.banned_in_hisec {
        allowed_zones.push(SecZone::HiSec(SecZoneCorruption::None));
    }
    if !sec_zone_info.banned_in_lowsec {
        allowed_zones.push(SecZone::LowSec(SecZoneCorruption::None));
    }
    allowed_zones.extend([SecZone::NullSec, SecZone::WSpace, SecZone::Hazard]);
    items
        .entry(item_id)
        .or_default()
        .entry(effect_id)
        .insert_entry(allowed_zones);
}
