use crate::{
    defs::{AttrVal, EAttrId, SolItemId},
    ec,
    sol::{
        svc::{
            calc::SolCalc,
            vast::{SolStatResource, SolVastFitData},
        },
        uad::{fit::SolFit, SolUad},
    },
};

pub struct SolResValFail {
    pub total_use: AttrVal,
    pub output: AttrVal,
    pub users: Vec<SolResUser>,
}
impl SolResValFail {
    fn new(total_use: AttrVal, output: AttrVal, users: Vec<SolResUser>) -> Self {
        Self {
            total_use,
            output,
            users,
        }
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
    pub(in crate::sol::svc::vast) fn validate_cpu_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolResValFail> {
        let stat = self.get_stats_cpu(uad, calc, fit);
        self.validate_resource_verbose(uad, calc, stat, &ec::attrs::CPU)
    }
    pub(in crate::sol::svc::vast) fn validate_pg_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        fit: &SolFit,
    ) -> Option<SolResValFail> {
        let stat = self.get_stats_pg(uad, calc, fit);
        self.validate_resource_verbose(uad, calc, stat, &ec::attrs::POWER)
    }
    fn validate_resource_verbose(
        &self,
        uad: &SolUad,
        calc: &mut SolCalc,
        stat: SolStatResource,
        use_attr_id: &EAttrId,
    ) -> Option<SolResValFail> {
        if stat.used <= stat.output {
            return None;
        };
        let mut users = Vec::with_capacity(self.mods_online.len());
        for item_id in self.mods_online.iter() {
            match calc.get_item_attr_val(uad, item_id, use_attr_id) {
                Ok(sol_val) => users.push(SolResUser::new(*item_id, sol_val.extra)),
                Err(_) => continue,
            };
        }
        Some(SolResValFail::new(stat.used, stat.output, users))
    }
    pub(in crate::sol::svc::vast) fn validate_cpu_fast(&self, uad: &SolUad, calc: &mut SolCalc, fit: &SolFit) -> bool {
        let stats = self.get_stats_cpu(uad, calc, fit);
        stats.used > stats.output
    }
    pub(in crate::sol::svc::vast) fn validate_pg_fast(&self, uad: &SolUad, calc: &mut SolCalc, fit: &SolFit) -> bool {
        let stats = self.get_stats_pg(uad, calc, fit);
        stats.used > stats.output
    }
}
