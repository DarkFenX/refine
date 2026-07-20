use crate::{
    api::EffectId,
    misc::{SecZone, SecZoneCorruption},
    rd::REffectId,
    svc::{SvcCtx, vast::VastFitData},
    ud::{ItemId, UData, UItemId},
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
pub struct ValEffectSecZoneFail {
    /// Solar system security zone.
    pub zone: SecZone,
    /// Map between IDs of items+effects which cannot be used in current security zone, and a list
    /// of security zones they can be used in.
    #[cfg_attr(feature = "serde", serde_as(as = "&serde_with::Map<_, serde_with::Map<_, _>>"))]
    pub items: Vec<(ItemId, Vec<(EffectId, Vec<SecZone>)>)>,
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
                    let mut item_fails = Vec::new();
                    for (&effect_rid, sec_zone_info) in item_data.iter() {
                        if sec_zone_info.banned_in_hisec {
                            match kfs.contains(&item_uid) {
                                true => continue 'items,
                                false => add_fail_entry(ctx.u_data, &mut item_fails, effect_rid, sec_zone_info),
                            }
                        }
                    }
                    if !item_fails.is_empty() {
                        let item_id = ctx.u_data.items.ext_id_by_int_id(item_uid);
                        items.push((item_id, item_fails));
                    }
                }
            }
            SecZone::LowSec(_) => {
                'items: for (&item_uid, item_data) in self.sec_zone_effect.iter() {
                    let mut item_fails = Vec::new();
                    for (&effect_rid, sec_zone_info) in item_data.iter() {
                        if sec_zone_info.banned_in_lowsec {
                            match kfs.contains(&item_uid) {
                                true => continue 'items,
                                false => add_fail_entry(ctx.u_data, &mut item_fails, effect_rid, sec_zone_info),
                            }
                        }
                    }
                    if !item_fails.is_empty() {
                        let item_id = ctx.u_data.items.ext_id_by_int_id(item_uid);
                        items.push((item_id, item_fails));
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
    item_fails: &mut Vec<(EffectId, Vec<SecZone>)>,
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
    item_fails.push((effect_id, allowed_zones));
}
