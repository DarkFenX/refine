use rc::ItemMutCommon;

use crate::{
    cmd::{
        shared::get_primary_item,
        stats::options::{
            HStatOption, HStatOptionEhp, HStatOptionErps, HStatOptionItemDps, HStatOptionItemRemoteCps,
            HStatOptionItemRemoteRps, HStatOptionItemVolley, HStatOptionRps, HStatResolvedOption,
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
    agility: Option<bool>,
    align_time: Option<bool>,
    speed: Option<bool>,
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
}
impl HGetItemStatsCmd {
    pub(crate) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemStats, HExecError> {
        let mut core_item = get_primary_item(core_sol, item_id)?;
        let mut stats = HItemStats::new();
        if self.agility.unwrap_or(self.default) {
            stats.agility = core_item.get_stat_agility().unwrap_or_default().into();
        }
        if self.align_time.unwrap_or(self.default) {
            stats.align_time = core_item.get_stat_align_time().unwrap_or_default().into();
        }
        if self.speed.unwrap_or(self.default) {
            stats.speed = core_item.get_stat_speed().into();
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
            stats.remote_cps = get_cap_rr_stats(&mut core_item, rcps_opt.options).into();
        }
        Ok(stats)
    }
}

fn get_dps_stats(core_item: &mut rc::ItemMut, options: Vec<HStatOptionItemDps>) -> Option<Vec<HStatDmg>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_spool = option.spool.map(|h_spool| h_spool.into());
        match core_item.get_stat_dps(option.reload, core_spool, option.include_charges, option.ignore_state) {
            Ok(core_stat) => results.push(core_stat.into()),
            Err(_) => return None,
        };
    }
    Some(results)
}

fn get_volley_stats(core_item: &mut rc::ItemMut, options: Vec<HStatOptionItemVolley>) -> Option<Vec<HStatDmg>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        let core_spool = option.spool.map(|h_spool| h_spool.into());
        match core_item.get_stat_volley(core_spool, option.include_charges, option.ignore_state) {
            Ok(core_stat) => results.push(core_stat.into()),
            Err(_) => return None,
        };
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

fn get_cap_rr_stats(core_item: &mut rc::ItemMut, options: Vec<HStatOptionItemRemoteCps>) -> Option<Vec<rc::AttrVal>> {
    let mut results = Vec::with_capacity(options.len());
    for option in options {
        match core_item.get_stat_remote_cps(option.ignore_state) {
            Ok(result) => results.push(result),
            Err(_) => return None,
        }
    }
    Some(results)
}
