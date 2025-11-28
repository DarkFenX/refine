use rc::ItemMutCommon;

use crate::{
    cmd::{
        shared::get_primary_item,
        stats::options::{
            HStatOption, HStatOptionCapBalance, HStatOptionCapSim, HStatOptionEhp, HStatOptionErps, HStatOptionItemDps,
            HStatOptionItemMining, HStatOptionItemOutCps, HStatOptionItemOutNps, HStatOptionItemOutRps,
            HStatOptionItemVolley, HStatOptionRps, HStatResolvedOption,
        },
    },
    info::{
        HItemStats,
        stats::{
            HStatCapSim, HStatDmg, HStatLayerEhp, HStatLayerErps, HStatLayerErpsRegen, HStatLayerRps,
            HStatLayerRpsRegen, HStatMining, HStatTank, HStatTankRegen,
        },
    },
    util::{HExecError, default_true},
};

#[derive(educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(crate) struct HGetItemStatsCmd {
    #[serde(default = "default_true")]
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
    dscan_range: Option<bool>,
    probing_size: Option<bool>,
    incoming_jam: Option<bool>,
    drone_control_range: Option<bool>,
    dps: Option<HStatOption<HStatOptionItemDps>>,
    volley: Option<HStatOption<HStatOptionItemVolley>>,
    mps: Option<HStatOption<HStatOptionItemMining>>,
    hp: Option<bool>,
    ehp: Option<HStatOption<HStatOptionEhp>>,
    wc_ehp: Option<bool>,
    rps: Option<HStatOption<HStatOptionRps>>,
    erps: Option<HStatOption<HStatOptionErps>>,
    resists: Option<bool>,
    outgoing_rps: Option<HStatOption<HStatOptionItemOutRps>>,
    outgoing_cps: Option<HStatOption<HStatOptionItemOutCps>>,
    outgoing_nps: Option<HStatOption<HStatOptionItemOutNps>>,
    cap_amount: Option<bool>,
    cap_balance: Option<HStatOption<HStatOptionCapBalance>>,
    cap_sim: Option<HStatOption<HStatOptionCapSim>>,
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
        if self.dscan_range.unwrap_or(self.default) {
            stats.dscan_range = core_item.get_stat_dscan_range().into();
        }
        if self.probing_size.unwrap_or(self.default) {
            stats.probing_size = core_item.get_stat_probing_size().unwrap_or_default().into();
        }
        if self.incoming_jam.unwrap_or(self.default) {
            stats.incoming_jam = core_item.get_stat_incoming_jam().into();
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
        let mps_opt = HStatResolvedOption::new(&self.mps, self.default);
        if mps_opt.enabled {
            stats.mps = get_mps_stats(&mut core_item, mps_opt.options).into()
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
        let rrps_opt = HStatResolvedOption::new(&self.outgoing_rps, self.default);
        if rrps_opt.enabled {
            stats.outgoing_rps = get_outgoing_rps_stats(&mut core_item, rrps_opt.options).into();
        }
        let rcps_opt = HStatResolvedOption::new(&self.outgoing_cps, self.default);
        if rcps_opt.enabled {
            stats.outgoing_cps = get_outgoing_cps_stats(&mut core_item, rcps_opt.options).into();
        }
        let rnps_opt = HStatResolvedOption::new(&self.outgoing_nps, self.default);
        if rnps_opt.enabled {
            stats.outgoing_nps = get_outgoing_nps_stats(&mut core_item, rnps_opt.options).into();
        }
        if self.cap_amount.unwrap_or(self.default) {
            stats.cap_amount = core_item.get_stat_cap_amount().into();
        }
        let cblc_opt = HStatResolvedOption::new(&self.cap_balance, self.default);
        if cblc_opt.enabled {
            stats.cap_balance = get_cap_balance_stats(&mut core_item, cblc_opt.options).into();
        }
        let csim_opt = HStatResolvedOption::new(&self.cap_sim, self.default);
        if csim_opt.enabled {
            stats.cap_sim = get_cap_sim_stats(&mut core_item, csim_opt.options).into();
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
        let core_spool = option.spool.map(Into::into);
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
        let core_spool = option.spool.map(Into::into);
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

fn get_mps_stats(core_item: &mut rc::ItemMut, options: Vec<HStatOptionItemMining>) -> Option<Vec<HStatMining>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        match core_item.get_stat_mps(option.reload, option.ignore_state) {
            Ok(core_stat) => results.push(core_stat.into()),
            Err(_) => return None,
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
        let core_incoming_dps = option.incoming_dps.map(Into::into);
        match core_item.get_stat_ehp(core_incoming_dps) {
            Ok(core_stat) => results.push(HStatTank::from_opt(core_stat)),
            Err(_) => return None,
        }
    }
    Some(results)
}

fn get_rps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<HStatOptionRps>,
) -> Option<Vec<HStatTankRegen<HStatLayerRps, HStatLayerRpsRegen>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_shield = rc::UnitInterval::new_clamped(option.shield_perc);
        let core_spool = option.spool.map(Into::into);
        match core_item.get_stat_rps(core_shield, core_spool) {
            Ok(core_stat) => results.push(core_stat.into()),
            Err(_) => return None,
        }
    }
    Some(results)
}

fn get_erps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<HStatOptionErps>,
) -> Option<Vec<HStatTankRegen<Option<HStatLayerErps>, Option<HStatLayerErpsRegen>>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_incoming_dps = option.incoming_dps.map(Into::into);
        let core_shield = rc::UnitInterval::new_clamped(option.shield_perc);
        let core_spool = option.spool.map(Into::into);
        match core_item.get_stat_erps(core_incoming_dps, core_shield, core_spool) {
            Ok(core_stat) => results.push(HStatTankRegen::from_opt(core_stat)),
            Err(_) => return None,
        }
    }
    Some(results)
}

fn get_outgoing_rps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<HStatOptionItemOutRps>,
) -> Option<Vec<HStatTank<rc::AttrVal>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        match core_item.get_stat_outgoing_rps(option.spool.map(Into::into), option.ignore_state) {
            Ok(result) => results.push(result.into()),
            Err(_) => return None,
        }
    }
    Some(results)
}

fn get_outgoing_cps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<HStatOptionItemOutCps>,
) -> Option<Vec<rc::AttrVal>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        match core_item.get_stat_outgoing_cps(option.ignore_state) {
            Ok(result) => results.push(result),
            Err(_) => return None,
        }
    }
    Some(results)
}

fn get_outgoing_nps_stats(
    core_item: &mut rc::ItemMut,
    options: Vec<HStatOptionItemOutNps>,
) -> Option<Vec<Option<rc::AttrVal>>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        match option.projectee_item_id {
            Some(projectee_item_id) => {
                match core_item.get_stat_outgoing_nps_applied(
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
            None => match core_item.get_stat_outgoing_nps(option.include_charges, option.ignore_state) {
                Ok(result) => results.push(Some(result)),
                Err(_) => return None,
            },
        }
    }
    Some(results)
}

fn get_cap_balance_stats(core_item: &mut rc::ItemMut, options: Vec<HStatOptionCapBalance>) -> Option<Vec<rc::AttrVal>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_src_kinds = (&option.src_kinds).into();
        match core_item.get_stat_cap_balance(core_src_kinds) {
            Ok(result) => results.push(result),
            Err(_) => return None,
        }
    }
    Some(results)
}

fn get_cap_sim_stats(core_item: &mut rc::ItemMut, options: Vec<HStatOptionCapSim>) -> Option<Vec<HStatCapSim>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let cap_perc = rc::UnitInterval::new_clamped(option.cap_perc);
        let stagger = (&option.stagger).into();
        match core_item.get_stat_cap_sim(cap_perc, stagger) {
            Ok(result) => results.push(result.into()),
            Err(_) => return None,
        }
    }
    Some(results)
}
