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
    pub(in crate::sol::svc::vast) fn validate_cpu_fast(&self, uad: &SolUad, calc: &mut SolCalc, fit: &SolFit) -> bool {
        let stats = self.get_stats_cpu(uad, calc, fit);
        stats.used <= stats.output.unwrap_or(OF(0.0))
    }
    pub(in crate::sol::svc::vast) fn validate_powergrid_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_powergrid(uad, calc, fit);
        stats.used <= stats.output.unwrap_or(OF(0.0))
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_calibration(uad, calc, fit);
        stats.used <= stats.output.unwrap_or(OF(0.0))
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bay_volume_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_drone_bay_volume(uad, calc, fit);
        stats.used <= stats.output.unwrap_or(OF(0.0))
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bandwidth_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_drone_bandwidth(uad, calc, fit);
        stats.used <= stats.output.unwrap_or(OF(0.0))
    }
    pub(in crate::sol::svc::vast) fn validate_fighter_bay_volume_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_fighter_bay_volume(uad, calc, fit);
        stats.used <= stats.output.unwrap_or(OF(0.0))
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_cpu_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValResFail> {
        let stats = self.get_stats_cpu(uad, calc, fit);
        validate_verbose_fitting(uad, calc, stats, self.mods_online.iter(), &ec::attrs::CPU)
    }
    pub(in crate::sol::svc::vast) fn validate_powergrid_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValResFail> {
        let stats = self.get_stats_powergrid(uad, calc, fit);
        validate_verbose_fitting(uad, calc, stats, self.mods_online.iter(), &ec::attrs::POWER)
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValResFail> {
        let stats = self.get_stats_calibration(uad, calc, fit);
        validate_verbose_other(stats, self.rigs_rigslot_calibration.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bay_volume_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValResFail> {
        let stats = self.get_stats_drone_bay_volume(uad, calc, fit);
        validate_verbose_other(stats, self.drones_volume.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_drone_bandwidth_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValResFail> {
        let stats = self.get_stats_drone_bandwidth(uad, calc, fit);
        validate_verbose_other(stats, self.drones_online_bandwidth.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_fighter_bay_volume_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolValResFail> {
        let stats = self.get_stats_fighter_bay_volume(uad, calc, fit);
        validate_verbose_other(stats, self.fighters_volume.iter())
    }
}

fn validate_verbose_fitting<'a>(
    uad: &SolUad,
    calc: &mut SolCalc,
    stats: SolStatRes,
    items: impl ExactSizeIterator<Item = &'a SolItemId>,
    use_attr_id: &EAttrId,
) -> Option<SolValResFail> {
    if stats.used <= stats.output.unwrap_or(OF(0.0)) {
        return None;
    };
    let mut users = Vec::with_capacity(items.len());
    for item_id in items {
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
    stats: SolStatRes,
    items: impl ExactSizeIterator<Item = (&'a SolItemId, &'a AttrVal)>,
) -> Option<SolValResFail> {
    if stats.used <= stats.output.unwrap_or(OF(0.0)) {
        return None;
    };
    let mut users = Vec::with_capacity(items.len());
    for (&item_id, &res_used) in items {
        if res_used > OF(0.0) {
            users.push(SolValResItemInfo {
                item_id,
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
