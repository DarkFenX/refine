use crate::{
    defs::{AttrVal, EAttrId, SolItemId, OF},
    ec,
    sol::{
        svc::{
            calc::SolCalc,
            vast::{SolStatRes, SolVastFitData},
        },
        uad::{fit::SolFit, SolUad},
    },
};

pub struct SolResValFail {
    pub used: AttrVal,
    pub output: Option<AttrVal>,
    pub users: Vec<SolResUser>,
}
impl SolResValFail {
    fn new(used: AttrVal, output: Option<AttrVal>, users: Vec<SolResUser>) -> Self {
        Self { used, output, users }
    }
}

pub struct SolResUser {
    pub item_id: SolItemId,
    pub used: AttrVal,
}
impl SolResUser {
    fn new(item_id: SolItemId, used: AttrVal) -> Self {
        Self { item_id, used }
    }
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_cpu_fast(&self, uad: &SolUad, calc: &mut SolCalc, fit: &SolFit) -> bool {
        let stats = self.get_stats_cpu(uad, calc, fit);
        stats.used <= stats.output.unwrap_or(OF(0.0))
    }
    pub(in crate::sol::svc::vast) fn validate_pg_fast(&self, uad: &SolUad, calc: &mut SolCalc, fit: &SolFit) -> bool {
        let stats = self.get_stats_pg(uad, calc, fit);
        stats.used <= stats.output.unwrap_or(OF(0.0))
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_fast(&self, uad: &SolUad, fit: &SolFit) -> bool {
        let stats = self.get_stats_calibration(uad, fit);
        stats.used <= stats.output.unwrap_or(OF(0.0))
    }
    pub(in crate::sol::svc::vast) fn validate_dronebay_volume_fast(&self, uad: &SolUad, fit: &SolFit) -> bool {
        let stats = self.get_stats_dronebay_volume(uad, fit);
        stats.used <= stats.output.unwrap_or(OF(0.0))
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_cpu_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolResValFail> {
        let stat = self.get_stats_cpu(uad, calc, fit);
        self.validate_resource_verbose_fitting(uad, calc, stat, self.mods_online.iter(), &ec::attrs::CPU)
    }
    pub(in crate::sol::svc::vast) fn validate_pg_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolResValFail> {
        let stat = self.get_stats_pg(uad, calc, fit);
        self.validate_resource_verbose_fitting(uad, calc, stat, self.mods_online.iter(), &ec::attrs::POWER)
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_verbose(
        &self,
        uad: &SolUad,
        fit: &SolFit,
    ) -> Option<SolResValFail> {
        let stat = self.get_stats_calibration(uad, fit);
        self.validate_resource_verbose_other(stat, self.rigs_rigslot_calibration.iter())
    }
    pub(in crate::sol::svc::vast) fn validate_dronebay_volume_verbose(
        &self,
        uad: &SolUad,
        fit: &SolFit,
    ) -> Option<SolResValFail> {
        let stat = self.get_stats_dronebay_volume(uad, fit);
        self.validate_resource_verbose_other(stat, self.drones_volume.iter())
    }
    fn validate_resource_verbose_fitting<'a>(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        stat: SolStatRes,
        items: impl ExactSizeIterator<Item = &'a SolItemId>,
        use_attr_id: &EAttrId,
    ) -> Option<SolResValFail> {
        if stat.used <= stat.output.unwrap_or(OF(0.0)) {
            return None;
        };
        let mut users = Vec::with_capacity(items.len());
        for item_id in items {
            match calc.get_item_attr_val(uad, item_id, use_attr_id) {
                Ok(sol_val) if sol_val.extra > OF(0.0) => users.push(SolResUser::new(*item_id, sol_val.extra)),
                _ => continue,
            };
        }
        Some(SolResValFail::new(stat.used, stat.output, users))
    }
    fn validate_resource_verbose_other<'a>(
        &self,
        stat: SolStatRes,
        items: impl ExactSizeIterator<Item = (&'a SolItemId, &'a AttrVal)>,
    ) -> Option<SolResValFail> {
        if stat.used <= stat.output.unwrap_or(OF(0.0)) {
            return None;
        };
        let mut users = Vec::with_capacity(items.len());
        for (&item_id, &res_used) in items {
            if res_used > OF(0.0) {
                users.push(SolResUser::new(item_id, res_used))
            }
        }
        Some(SolResValFail::new(stat.used, stat.output, users))
    }
}
