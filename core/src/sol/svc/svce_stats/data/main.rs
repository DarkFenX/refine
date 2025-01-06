use crate::sol::svc::svce_stats::stat_cpu::SolStatRegCpu;

#[derive(Clone)]
pub(in crate::sol::svc) struct SolSvcStatsData {
    pub(in crate::sol::svc::svce_stats) cpu: SolStatRegCpu,
}
impl SolSvcStatsData {
    pub(in crate::sol::svc) fn new() -> Self {
        Self {
            cpu: SolStatRegCpu::new(),
        }
    }
}
