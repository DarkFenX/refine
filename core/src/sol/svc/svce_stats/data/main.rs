use crate::sol::svc::svce_stats::stat_mods_online::SolStatRegModsOnline;

#[derive(Clone)]
pub(in crate::sol::svc) struct SolSvcStatsData {
    pub(in crate::sol::svc::svce_stats) mods_online: SolStatRegModsOnline,
}
impl SolSvcStatsData {
    pub(in crate::sol::svc) fn new() -> Self {
        Self {
            mods_online: SolStatRegModsOnline::new(),
        }
    }
}
