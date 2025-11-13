use crate::{
    cmd::{
        shared::get_primary_fleet,
        stats::options::{
            HStatOption, HStatOptionFitDps, HStatOptionFitRemoteNps, HStatOptionFitRemoteRps, HStatOptionFitVolley,
            HStatResolvedOption,
        },
    },
    info::{
        HFleetStats,
        stats::{HStatDmg, HStatTank},
    },
    util::{HExecError, default_true},
};

#[derive(educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(crate) struct HGetFleetStatsCmd {
    #[serde(default = "default_true")]
    #[educe(Default = true)]
    default: bool,
    dps: Option<HStatOption<HStatOptionFitDps>>,
    volley: Option<HStatOption<HStatOptionFitVolley>>,
    remote_rps: Option<HStatOption<HStatOptionFitRemoteRps>>,
    remote_cps: Option<bool>,
    remote_nps: Option<HStatOption<HStatOptionFitRemoteNps>>,
}
impl HGetFleetStatsCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        fleet_id: &rc::FleetId,
    ) -> Result<HFleetStats, HExecError> {
        let mut core_fleet = get_primary_fleet(core_sol, fleet_id)?;
        let mut stats = HFleetStats::new();
        let dps_opt = HStatResolvedOption::new(&self.dps, self.default);
        if dps_opt.enabled {
            stats.dps = Some(get_dps_stats(&mut core_fleet, dps_opt.options));
        }
        let volley_opt = HStatResolvedOption::new(&self.volley, self.default);
        if volley_opt.enabled {
            stats.volley = Some(get_volley_stats(&mut core_fleet, volley_opt.options));
        }
        let rrps_opt = HStatResolvedOption::new(&self.remote_rps, self.default);
        if rrps_opt.enabled {
            stats.remote_rps = Some(get_remote_rps_stats(&mut core_fleet, rrps_opt.options));
        }
        if self.remote_cps.unwrap_or(self.default) {
            stats.remote_cps = Some(core_fleet.get_stat_remote_cps());
        }
        let rnps_opt = HStatResolvedOption::new(&self.remote_nps, self.default);
        if rnps_opt.enabled {
            stats.remote_nps = Some(get_remote_nps_stats(&mut core_fleet, rnps_opt.options));
        }
        Ok(stats)
    }
}

fn get_dps_stats(core_fleet: &mut rc::FleetMut, options: Vec<HStatOptionFitDps>) -> Vec<Option<HStatDmg>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_item_kinds = (&option.item_kinds).into();
        let core_spool = option.spool.map(Into::into);
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fleet.get_stat_dps_applied(core_item_kinds, option.reload, core_spool, &projectee_item_id) {
                    Ok(core_stat) => results.push(Some(core_stat.into())),
                    Err(_) => results.push(None),
                };
            }
            None => {
                let core_stat = core_fleet.get_stat_dps(core_item_kinds, option.reload, core_spool);
                results.push(Some(core_stat.into()));
            }
        }
    }
    results
}

fn get_volley_stats(core_fleet: &mut rc::FleetMut, options: Vec<HStatOptionFitVolley>) -> Vec<Option<HStatDmg>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_item_kinds = (&option.item_kinds).into();
        let core_spool = option.spool.map(Into::into);
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fleet.get_stat_volley_applied(core_item_kinds, core_spool, &projectee_item_id) {
                    Ok(core_stat) => results.push(Some(core_stat.into())),
                    Err(_) => results.push(None),
                };
            }
            None => {
                let core_stat = core_fleet.get_stat_volley(core_item_kinds, core_spool);
                results.push(Some(core_stat.into()));
            }
        }
    }
    results
}

fn get_remote_rps_stats(
    core_fleet: &mut rc::FleetMut,
    options: Vec<HStatOptionFitRemoteRps>,
) -> Vec<HStatTank<rc::AttrVal>> {
    options
        .iter()
        .map(|option| {
            let core_item_kinds = (&option.item_kinds).into();
            let core_spool = option.spool.map(Into::into);
            core_fleet.get_stat_remote_rps(core_item_kinds, core_spool).into()
        })
        .collect()
}

fn get_remote_nps_stats(
    core_fleet: &mut rc::FleetMut,
    options: Vec<HStatOptionFitRemoteNps>,
) -> Vec<Option<rc::AttrVal>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_item_kinds = (&option.item_kinds).into();
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fleet.get_stat_remote_nps_applied(core_item_kinds, &projectee_item_id) {
                    Ok(result) => results.push(Some(result)),
                    Err(_) => results.push(None),
                }
            }
            None => {
                let result = core_fleet.get_stat_remote_nps(core_item_kinds);
                results.push(Some(result));
            }
        }
    }
    results
}
