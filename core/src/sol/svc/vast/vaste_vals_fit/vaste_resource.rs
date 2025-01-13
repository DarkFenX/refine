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
    pub output: AttrVal,
    pub users: Vec<SolResUser>,
}
impl SolResValFail {
    fn new(used: AttrVal, output: AttrVal, users: Vec<SolResUser>) -> Self {
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
        stats.used <= stats.output
    }
    pub(in crate::sol::svc::vast) fn validate_pg_fast(&self, uad: &SolUad, calc: &mut SolCalc, fit: &SolFit) -> bool {
        let stats = self.get_stats_pg(uad, calc, fit);
        stats.used <= stats.output
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_fast(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> bool {
        let stats = self.get_stats_calibration(uad, calc, fit);
        stats.used <= stats.output
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_cpu_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolResValFail> {
        let stat = self.get_stats_cpu(uad, calc, fit);
        self.validate_resource_verbose(uad, calc, stat, self.mods_online.iter(), &ec::attrs::CPU)
    }
    pub(in crate::sol::svc::vast) fn validate_pg_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolResValFail> {
        let stat = self.get_stats_pg(uad, calc, fit);
        self.validate_resource_verbose(uad, calc, stat, self.mods_online.iter(), &ec::attrs::POWER)
    }
    pub(in crate::sol::svc::vast) fn validate_calibration_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolResValFail> {
        let stat = self.get_stats_calibration(uad, calc, fit);
        self.validate_resource_verbose(uad, calc, stat, self.rigs_rigslot.iter(), &ec::attrs::UPGRADE_COST)
    }
    fn validate_resource_verbose<'a>(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        stat: SolStatRes,
        items: impl ExactSizeIterator<Item = &'a SolItemId>,
        use_attr_id: &EAttrId,
    ) -> Option<SolResValFail> {
        if stat.used <= stat.output {
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
}
