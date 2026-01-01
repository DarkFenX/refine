use crate::{
    cmd::{
        shared::get_primary_fleet,
        stats::options::{
            HStatOption, HStatOptionFitDps, HStatOptionFitMining, HStatOptionFitOutCps, HStatOptionFitOutNps,
            HStatOptionFitOutRps, HStatOptionFitVolley, HStatResolvedOption,
        },
    },
    info::{
        HFleetStats,
        stats::{HStatDmg, HStatMining, HStatTank},
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
    mps: Option<HStatOption<HStatOptionFitMining>>,
    outgoing_nps: Option<HStatOption<HStatOptionFitOutNps>>,
    outgoing_rps: Option<HStatOption<HStatOptionFitOutRps>>,
    outgoing_cps: Option<HStatOption<HStatOptionFitOutCps>>,
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
        let mps_opt = HStatResolvedOption::new(&self.mps, self.default);
        if mps_opt.enabled {
            stats.mps = Some(get_mps_stats(&mut core_fleet, mps_opt.options));
        }
        let out_nps_opt = HStatResolvedOption::new(&self.outgoing_nps, self.default);
        if out_nps_opt.enabled {
            stats.outgoing_nps = Some(get_outgoing_nps_stats(&mut core_fleet, out_nps_opt.options));
        }
        let out_rps_opt = HStatResolvedOption::new(&self.outgoing_rps, self.default);
        if out_rps_opt.enabled {
            stats.outgoing_rps = Some(get_outgoing_rps_stats(&mut core_fleet, out_rps_opt.options));
        }
        let out_cps_opt = HStatResolvedOption::new(&self.outgoing_cps, self.default);
        if out_cps_opt.enabled {
            stats.outgoing_cps = Some(get_outgoing_cps_stats(&mut core_fleet, out_cps_opt.options));
        }
        Ok(stats)
    }
}

fn get_dps_stats(core_fleet: &mut rc::FleetMut, options: Vec<HStatOptionFitDps>) -> Vec<Option<HStatDmg>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_item_kinds = (&option.item_kinds).into();
        let core_spool = option.spool.map(Into::into);
        match &option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fleet.get_stat_dps_applied(core_item_kinds, option.reload, core_spool, projectee_item_id) {
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
        match &option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fleet.get_stat_volley_applied(core_item_kinds, core_spool, projectee_item_id) {
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
fn get_mps_stats(core_fleet: &mut rc::FleetMut, options: Vec<HStatOptionFitMining>) -> Vec<HStatMining> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_result = core_fleet.get_stat_mps((&option.item_kinds).into(), option.reload);
        results.push(core_result.into());
    }
    results
}
fn get_outgoing_nps_stats(
    core_fleet: &mut rc::FleetMut,
    options: Vec<HStatOptionFitOutNps>,
) -> Vec<Option<rc::AttrVal>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_item_kinds = (&option.item_kinds).into();
        let core_time_options = option.time_options.into();
        match &option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fleet.get_stat_outgoing_nps_applied(core_item_kinds, core_time_options, projectee_item_id) {
                    Ok(result) => results.push(Some(result)),
                    Err(_) => results.push(None),
                }
            }
            None => {
                let result = core_fleet.get_stat_outgoing_nps(core_item_kinds, core_time_options);
                results.push(Some(result));
            }
        }
    }
    results
}
fn get_outgoing_rps_stats(
    core_fleet: &mut rc::FleetMut,
    options: Vec<HStatOptionFitOutRps>,
) -> Vec<Option<HStatTank<rc::AttrVal>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_item_kinds = (&option.item_kinds).into();
        let core_time_options = option.time_options.into();
        match &option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fleet.get_stat_outgoing_rps_applied(core_item_kinds, core_time_options, projectee_item_id) {
                    Ok(result) => results.push(Some(result.into())),
                    Err(_) => results.push(None),
                }
            }
            None => {
                let result = core_fleet.get_stat_outgoing_rps(core_item_kinds, core_time_options);
                results.push(Some(result.into()));
            }
        }
    }
    results
}
fn get_outgoing_cps_stats(
    core_fleet: &mut rc::FleetMut,
    options: Vec<HStatOptionFitOutCps>,
) -> Vec<Option<rc::AttrVal>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_time_options = option.time_options.into();
        match &option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_fleet.get_stat_outgoing_cps_applied(core_time_options, projectee_item_id) {
                    Ok(result) => results.push(Some(result)),
                    Err(_) => results.push(None),
                }
            }
            None => {
                let result = core_fleet.get_stat_outgoing_cps(core_time_options);
                results.push(Some(result));
            }
        }
    }
    results
}
