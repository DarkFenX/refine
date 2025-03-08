use crate::{
    defs::{AttrVal, EAttrId, OF, SolItemId},
    ec,
    sol::{
        svc::{
            calc::SolCalc,
            vast::{SolStatRes, SolVastFitData},
        },
        uad::{SolUad, fit::SolFit},
    },
    util::{StSet, round},
};

pub struct SolValResFail {
    pub used: AttrVal,
    pub output: Option<AttrVal>,
    pub users: Vec<SolValResItemInfo>,
}

pub struct SolValResItemInfo {
    pub item_id: SolItemId,
    pub used: AttrVal,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_cpu_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> bool {
        let output = match fit.ship {
            Some(ship_id) => match calc.get_item_attr_val(uad, &ship_id, &ec::attrs::CPU_OUTPUT) {
                Ok(attr_val) => attr_val.extra,
                Err(_) => OF(0.0),
            },
            None => OF(0.0),
        };
        let mut total_use = OF(0.0);
        let mut force_pass = true;
        for item_id in self.mods_online.iter() {
            let item_use = match calc.get_item_attr_val(uad, item_id, &ec::attrs::CPU) {
                Ok(attr_val) => attr_val.extra,
                Err(_) => continue,
            };
            if force_pass && item_use > OF(0.0) && !kfs.contains(item_id) {
                force_pass = false;
            }
            total_use += item_use;
        }
        let total_use = round(total_use, 2);
        force_pass || total_use <= output
    }
    pub(in crate::sol::svc::vast) fn validate_powergrid_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_powergrid(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_calibration(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bay_volume_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_drone_bay_volume(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bandwidth_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_drone_bandwidth(uad, calc, fit);
        validate_fast(stats)
    }
    pub(in crate::sol::svc::vast) fn validate_fighter_bay_volume_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_fighter_bay_volume(uad, calc, fit);
        validate_fast(stats)
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_cpu_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValResFail> {
        let output = match fit.ship {
            Some(ship_id) => match calc.get_item_attr_val(uad, &ship_id, &ec::attrs::CPU_OUTPUT) {
                Ok(attr_val) => Some(attr_val.extra),
                Err(_) => None,
            },
            None => None,
        };
        let mut total_use = OF(0.0);
        let mut users = Vec::with_capacity(self.mods_online.len());
        for item_id in self.mods_online.iter() {
            let item_use = match calc.get_item_attr_val(uad, item_id, &ec::attrs::CPU) {
                Ok(attr_val) => attr_val.extra,
                Err(_) => continue,
            };
            total_use += item_use;
            if item_use > OF(0.0) && !kfs.contains(item_id) {
                users.push(SolValResItemInfo {
                    item_id: *item_id,
                    used: item_use,
                });
            }
        }
        let total_use = round(total_use, 2);
        if users.is_empty() || total_use <= output.unwrap_or(OF(0.0)) {
            return None;
        }
        Some(SolValResFail {
            used: total_use,
            output,
            users,
        })
    }
    pub(in crate::sol::svc::vast) fn validate_powergrid_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValResFail> {
        let stats = self.get_stats_powergrid(uad, calc, fit);
        validate_verbose_fitting(uad, calc, kfs, stats, self.mods_online.iter(), &ec::attrs::POWER)
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValResFail> {
        let stats = self.get_stats_calibration(uad, calc, fit);
        validate_verbose_other(kfs, stats, self.rigs_rigslot_calibration.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bay_volume_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValResFail> {
        let stats = self.get_stats_drone_bay_volume(uad, calc, fit);
        validate_verbose_other(kfs, stats, self.drones_volume.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bandwidth_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValResFail> {
        let stats = self.get_stats_drone_bandwidth(uad, calc, fit);
        validate_verbose_other(kfs, stats, self.drones_online_bandwidth.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_fighter_bay_volume_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
        kfs: &StSet<SolItemId>,
    ) -> Option<SolValResFail> {
        let stats = self.get_stats_fighter_bay_volume(uad, calc, fit);
        validate_verbose_other(kfs, stats, self.fighters_volume.iter())
    }
}

fn validate_fast(stats: SolStatRes) -> bool {
    stats.used <= stats.output.unwrap_or(OF(0.0))
}
fn validate_verbose_fitting<'a>(
    uad: &SolUad,
    calc: &mut SolCalc,
    kfs: &StSet<SolItemId>,
    stats: SolStatRes,
    items: impl ExactSizeIterator<Item = &'a SolItemId>,
    use_attr_id: &EAttrId,
) -> Option<SolValResFail> {
    if stats.used <= stats.output.unwrap_or(OF(0.0)) {
        return None;
    };
    let mut users = Vec::with_capacity(items.len());
    for item_id in items {
        if kfs.contains(&item_id) {
            continue;
        }
        match calc.get_item_attr_val(uad, item_id, use_attr_id) {
            Ok(sol_val) if sol_val.extra > OF(0.0) => users.push(SolValResItemInfo {
                item_id: *item_id,
                used: sol_val.extra,
            }),
            _ => continue,
        };
    }
    Some(SolValResFail {
        used: stats.used,
        output: stats.output,
        users,
    })
}
fn validate_verbose_other<'a>(
    kfs: &StSet<SolItemId>,
    stats: SolStatRes,
    items: impl ExactSizeIterator<Item = (&'a SolItemId, &'a AttrVal)>,
) -> Option<SolValResFail> {
    if stats.used <= stats.output.unwrap_or(OF(0.0)) {
        return None;
    };
    let mut users = Vec::with_capacity(items.len());
    for (item_id, &res_used) in items {
        if kfs.contains(item_id) {
            continue;
        }
        if res_used > OF(0.0) {
            users.push(SolValResItemInfo {
                item_id: *item_id,
                used: res_used,
            })
        }
    }
    Some(SolValResFail {
        used: stats.used,
        output: stats.output,
        users,
    })
}
