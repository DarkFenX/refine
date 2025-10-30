use rc::ItemMutCommon;

use crate::{
    cmd::{
        shared::get_primary_item,
        stats::options::{
            HStatOption, HStatOptionEhp, HStatOptionErps, HStatOptionItemDps, HStatOptionItemRemoteCps,
            HStatOptionItemRemoteNps, HStatOptionItemRemoteRps, HStatOptionItemVolley, HStatOptionRps,
            HStatResolvedOption,
        },
    },
    info::{
        HItemStats,
        stats::{HStatDmg, HStatLayerEhp, HStatLayerErps, HStatLayerRps, HStatTank},
    },
    util::HExecError,
};

#[derive(educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(crate) struct HGetItemStatsCmd {
    #[serde(default)]
    #[educe(Default = true)]
    default: bool,
    speed: Option<bool>,
    agility: Option<bool>,
    align_time: Option<bool>,
    sig_radius: Option<bool>,
    mass: Option<bool>,
    warp_speed: Option<bool>,
    max_warp_range: Option<bool>,
    locks: Option<bool>,
    lock_range: Option<bool>,
    scan_res: Option<bool>,
    sensor: Option<bool>,
    probing_size: Option<bool>,
    jam_chance: Option<bool>,
    drone_control_range: Option<bool>,
    dps: Option<HStatOption<HStatOptionItemDps>>,
    volley: Option<HStatOption<HStatOptionItemVolley>>,
    hp: Option<bool>,
    ehp: Option<HStatOption<HStatOptionEhp>>,
    wc_ehp: Option<bool>,
    rps: Option<HStatOption<HStatOptionRps>>,
    erps: Option<HStatOption<HStatOptionErps>>,
    resists: Option<bool>,
    remote_rps: Option<HStatOption<HStatOptionItemRemoteRps>>,
    remote_cps: Option<HStatOption<HStatOptionItemRemoteCps>>,
    remote_nps: Option<HStatOption<HStatOptionItemRemoteNps>>,
    cap: Option<bool>,
    neut_resist: Option<bool>,
}
impl HGetItemStatsCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemStats, HExecError> {
        let mut core_item = get_primary_item(core_sol, item_id)?;
        let mut stats = HItemStats::new();
        if self.speed.unwrap_or(self.default) {
            stats.speed = core_item.get_stat_speed().into();
        }
        if self.agility.unwrap_or(self.default) {
            stats.agility = core_item.get_stat_agility().unwrap_or_default().into();
        }
        if self.align_time.unwrap_or(self.default) {
            stats.align_time = core_item.get_stat_align_time().unwrap_or_default().into();
        }
        if self.sig_radius.unwrap_or(self.default) {
            stats.sig_radius = core_item.get_stat_sig_radius().into();
        }
        if self.mass.unwrap_or(self.default) {
            stats.mass = core_item.get_stat_mass().into();
        }
        if self.warp_speed.unwrap_or(self.default) {
            stats.warp_speed = core_item.get_stat_warp_speed().unwrap_or_default().into();
        }
        if self.max_warp_range.unwrap_or(self.default) {
            stats.max_warp_range = core_item.get_stat_max_warp_range().unwrap_or_default().into();
        }
        if self.locks.unwrap_or(self.default) {
            stats.locks = core_item.get_stat_locks().into();
        }
        if self.lock_range.unwrap_or(self.default) {
            stats.lock_range = core_item.get_stat_lock_range().into();
        }
        if self.scan_res.unwrap_or(self.default) {
            stats.scan_res = core_item.get_stat_scan_res().into();
        }
        if self.sensor.unwrap_or(self.default) {
            stats.sensor = core_item.get_stat_sensor().into();
        }
        if self.probing_size.unwrap_or(self.default) {
            stats.probing_size = core_item.get_stat_probing_size().unwrap_or_default().into();
        }
        if self.jam_chance.unwrap_or(self.default) {
            stats.jam_chance = core_item.get_stat_jam_chance().into();
        }
        if self.drone_control_range.unwrap_or(self.default) {
            stats.drone_control_range = core_item.get_stat_drone_control_range().into();
        }
        let dps_opt = HStatResolvedOption::new(&self.dps, self.default);
        if dps_opt.enabled {
            stats.dps = get_dps_stats(&mut core_item, dps_opt.options).into()
        }
        let volley_opt = HStatResolvedOption::new(&self.volley, self.default);
        if volley_opt.enabled {
            stats.volley = get_volley_stats(&mut core_item, volley_opt.options).into()
        }
        if self.hp.unwrap_or(self.default) {
            stats.hp = core_item.get_stat_hp().into();
        }
        let ehp_opt = HStatResolvedOption::new(&self.ehp, self.default);
        if ehp_opt.enabled {
            stats.ehp = get_ehp_stats(&mut core_item, ehp_opt.options).into()
        }
        if self.wc_ehp.unwrap_or(self.default) {
            stats.wc_ehp = core_item.get_stat_wc_ehp().ok().map(HStatTank::from_opt).into();
        }
        let rps_opt = HStatResolvedOption::new(&self.rps, self.default);
        if rps_opt.enabled {
            stats.rps = get_rps_stats(&mut core_item, rps_opt.options).into();
        }
        let erps_opt = HStatResolvedOption::new(&self.erps, self.default);
        if erps_opt.enabled {
            stats.erps = get_erps_stats(&mut core_item, erps_opt.options).into();
        }
        if self.resists.unwrap_or(self.default) {
            stats.resists = core_item.get_stat_resists().into();
        }
        let rrps_opt = HStatResolvedOption::new(&self.remote_rps, self.default);
        if rrps_opt.enabled {
            stats.remote_rps = get_remote_rps_stats(&mut core_item, rrps_opt.options).into();
        }
        let rcps_opt = HStatResolvedOption::new(&self.remote_cps, self.default);
        if rcps_opt.enabled {
            stats.remote_cps = get_remote_cps_stats(&mut core_item, rcps_opt.options).into();
        }
        let rnps_opt = HStatResolvedOption::new(&self.remote_nps, self.default);
        if rnps_opt.enabled {
            stats.remote_nps = get_remote_nps_stats(&mut core_item, rnps_opt.options).into();
        }
        if self.cap.unwrap_or(self.default) {
            stats.cap = core_item.get_stat_cap().into();
        }
        if self.neut_resist.unwrap_or(self.default) {
            stats.neut_resist = core_item.get_stat_neut_resist().into();
        }
        Ok(stats)
    }
}

fn get_dps_stats(core_item: &mut rc::ItemMut, options: Vec<HStatOptionItemDps>) -> Option<Vec<Option<HStatDmg>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_spool = option.spool.map(|h_spool| h_spool.into());
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_item.get_stat_dps_applied(
                    option.reload,
                    core_spool,
                    option.include_charges,
                    option.ignore_state,
                    &projectee_item_id,
                ) {
                    Ok(core_stat) => results.push(Some(core_stat.into())),
                    Err(core_err) => {
                        match core_err {
                            rc::err::ItemStatAppliedError::ItemNotLoaded(_)
                            | rc::err::ItemStatAppliedError::UnsupportedStat(_) => return None,
                            rc::err::ItemStatAppliedError::ProjecteeNotFound(_)
                            | rc::err::ItemStatAppliedError::ProjecteeCantTakeProjs(_) => results.push(None),
                        };
                    }
                };
            }
            None => {
                match core_item.get_stat_dps(option.reload, core_spool, option.include_charges, option.ignore_state) {
                    Ok(core_stat) => results.push(Some(core_stat.into())),
                    Err(core_err) => {
                        return match core_err {
                            rc::err::ItemStatError::ItemNotLoaded(_) | rc::err::ItemStatError::UnsupportedStat(_) => {
                                None
                            }
                        };
                    }
                };
            }
        }
    }
    Some(results)
}

fn get_volley_stats(core_item: &mut rc::ItemMut, options: Vec<HStatOptionItemVolley>) -> Option<Vec<Option<HStatDmg>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_spool = option.spool.map(|h_spool| h_spool.into());
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_item.get_stat_volley_applied(
                    core_spool,
                    option.include_charges,
                    option.ignore_state,
                    &projectee_item_id,
                ) {
                    Ok(core_stat) => results.push(Some(core_stat.into())),
                    Err(core_err) => {
                        match core_err {
                            rc::err::ItemStatAppliedError::ItemNotLoaded(_)
                            | rc::err::ItemStatAppliedError::UnsupportedStat(_) => return None,
                            rc::err::ItemStatAppliedError::ProjecteeNotFound(_)
                            | rc::err::ItemStatAppliedError::ProjecteeCantTakeProjs(_) => results.push(None),
                        };
                    }
                };
            }
            None => {
                match core_item.get_stat_volley(core_spool, option.include_charges, option.ignore_state) {
                    Ok(core_stat) => results.push(Some(core_stat.into())),
                    Err(core_err) => {
                        return match core_err {
                            rc::err::ItemStatError::ItemNotLoaded(_) | rc::err::ItemStatError::UnsupportedStat(_) => {
                                None
                            }
                        };
                    }
                };
            }
        }
    }
    Some(results)
}

fn get_ehp_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<HStatOptionEhp>,
) -> Option<Vec<HStatTank<Option<HStatLayerEhp>>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_incoming_dps = option.incoming_dps.map(|h_incoming_dps| h_incoming_dps.into());
        match core_item.get_stat_ehp(core_incoming_dps) {
            Ok(core_stat) => results.push(HStatTank::from_opt(core_stat)),
            Err(_) => return None,
        }
    }
    Some(results)
}

fn get_rps_stats(core_item: &mut rc::ItemMut, options: Vec<HStatOptionRps>) -> Option<Vec<HStatTank<HStatLayerRps>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        match core_item.get_stat_rps(option.spool.map(|v| v.into())) {
            Ok(core_stat) => results.push(core_stat.into()),
            Err(_) => return None,
        }
    }
    Some(results)
}

fn get_erps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<HStatOptionErps>,
) -> Option<Vec<HStatTank<Option<HStatLayerErps>>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_incoming_dps = option.incoming_dps.map(|h_incoming_dps| h_incoming_dps.into());
        let core_spool = option.spool.map(|h_spool| h_spool.into());
        match core_item.get_stat_erps(core_incoming_dps, core_spool) {
            Ok(core_stat) => results.push(HStatTank::from_opt(core_stat)),
            Err(_) => return None,
        }
    }
    Some(results)
}

fn get_remote_rps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<HStatOptionItemRemoteRps>,
) -> Option<Vec<HStatTank<rc::AttrVal>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        match core_item.get_stat_remote_rps(option.spool.map(|spool| spool.into()), option.ignore_state) {
            Ok(result) => results.push(result.into()),
            Err(_) => return None,
        }
    }
    Some(results)
}

fn get_remote_cps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<HStatOptionItemRemoteCps>,
) -> Option<Vec<rc::AttrVal>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        match core_item.get_stat_remote_cps(option.ignore_state) {
            Ok(result) => results.push(result),
            Err(_) => return None,
        }
    }
    Some(results)
}

fn get_remote_nps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<HStatOptionItemRemoteNps>,
) -> Option<Vec<Option<rc::AttrVal>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_item.get_stat_remote_nps_applied(
                    option.include_charges,
                    option.ignore_state,
                    &projectee_item_id,
                ) {
                    Ok(result) => results.push(Some(result)),
                    Err(core_err) => {
                        match core_err {
                            rc::err::ItemStatAppliedError::ItemNotLoaded(_)
                            | rc::err::ItemStatAppliedError::UnsupportedStat(_) => return None,
                            rc::err::ItemStatAppliedError::ProjecteeNotFound(_)
                            | rc::err::ItemStatAppliedError::ProjecteeCantTakeProjs(_) => results.push(None),
                        };
                    }
                }
            }
            None => match core_item.get_stat_remote_nps(option.include_charges, option.ignore_state) {
                Ok(result) => results.push(Some(result)),
                Err(_) => return None,
            },
        }
    }
    Some(results)
}
