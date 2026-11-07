use rc::stats::ItemStatsCommon;

use crate::{
    PValue, Value,
    err::BrResolveError,
    stats::{
        ItemStats, StatBrFallibleError, StatCapSim, StatDmg, StatEhp, StatErps, StatInJam, StatItemStateOptions,
        StatJump, StatMining, StatOptionCapBlc, StatOptionCapSim, StatOptionEhp, StatOptionErps, StatOptionIncomingJam,
        StatOptionItemDmg, StatOptionItemMining, StatOptionItemOutCps, StatOptionItemOutNps, StatOptionItemOutRps,
        StatOptionJump, StatOptionMass, StatOptionRps, StatOutReps, StatResult, StatRps,
        err::{StatItemAppliedError, StatItemError, StatJumpError},
        exec_shared::{collect_stats_err_both_br, collect_stats_err_inner},
        item::ItemStatsOptionsResolved,
    },
};

impl ItemStatsOptionsResolved {
    pub(in crate::stats) fn execute(&self, core_item_stats: &mut rc::stats::ItemStats) -> ItemStats {
        let Self {
            dmg,
            mps,
            outgoing_nps,
            outgoing_rps,
            outgoing_cps,
            resists,
            hp,
            ehp,
            wc_ehp,
            rps,
            erps,
            breach_resist,
            cap_amount,
            cap_balance,
            cap_sim,
            neut_resist,
            locks,
            lock_range,
            scan_res,
            sensors,
            dscan_range,
            probing_size,
            incoming_jam,
            speed,
            agility,
            align_time,
            sig_radius,
            mass,
            warp_speed,
            max_warp_range,
            jump,
            drone_control_range,
            can_warp,
            can_jump_gate,
            can_jump_wormhole,
            can_jump_drive,
            can_dock_station,
            can_dock_citadel,
            can_tether,
        } = self;
        let mut stats = ItemStats { .. };
        // Output stats can switch item states. Options which switch go first, all under one switch;
        // the rest is computed in order below, along with results of switched ones
        let mut dmg_switched = SwitchedResults::new(dmg.iter().flatten().flatten());
        let mut mps_switched = SwitchedResults::new(mps.iter().flatten());
        let mut outgoing_nps_switched = SwitchedResults::new(outgoing_nps.iter().flatten().flatten());
        let mut outgoing_cps_switched = SwitchedResults::new(outgoing_cps.iter().flatten().flatten());
        let mut outgoing_rps_switched = SwitchedResults::new(outgoing_rps.iter().flatten().flatten());
        if dmg_switched.is_switching()
            || mps_switched.is_switching()
            || outgoing_nps_switched.is_switching()
            || outgoing_cps_switched.is_switching()
            || outgoing_rps_switched.is_switching()
        {
            let mut core_switched_stats = core_item_stats.switch_states();
            dmg_switched.fill(dmg.iter().flatten().flatten(), |option| {
                get_dmg_stat(&mut core_switched_stats, option)
            });
            mps_switched.fill(mps.iter().flatten(), |option| {
                get_mps_stat(&mut core_switched_stats, option)
            });
            outgoing_nps_switched.fill(outgoing_nps.iter().flatten().flatten(), |option| {
                get_outgoing_nps_stat(&mut core_switched_stats, option)
            });
            outgoing_cps_switched.fill(outgoing_cps.iter().flatten().flatten(), |option| {
                get_outgoing_cps_stat(&mut core_switched_stats, option)
            });
            outgoing_rps_switched.fill(outgoing_rps.iter().flatten().flatten(), |option| {
                get_outgoing_rps_stat(&mut core_switched_stats, option)
            });
        }
        if let Some(options) = dmg {
            stats.dmg = collect_stats_err_both_br(
                options,
                dmg_switched.or_get(|option| get_dmg_stat(core_item_stats, option)),
            );
        }
        if let Some(options) = mps {
            stats.mps = collect_stats_err_inner(
                options,
                mps_switched.or_get(|option| get_mps_stat(core_item_stats, option)),
            );
        }
        if let Some(options) = outgoing_nps {
            stats.outgoing_nps = collect_stats_err_both_br(
                options,
                outgoing_nps_switched.or_get(|option| get_outgoing_nps_stat(core_item_stats, option)),
            );
        }
        if let Some(options) = outgoing_cps {
            stats.outgoing_cps = collect_stats_err_both_br(
                options,
                outgoing_cps_switched.or_get(|option| get_outgoing_cps_stat(core_item_stats, option)),
            );
        }
        if let Some(options) = outgoing_rps {
            stats.outgoing_rps = collect_stats_err_both_br(
                options,
                outgoing_rps_switched.or_get(|option| get_outgoing_rps_stat(core_item_stats, option)),
            );
        }
        // Tank
        if *resists {
            stats.resists = StatResult::from_result_outer(core_item_stats.get_stat_resists());
        }
        if *hp {
            stats.hp = StatResult::from_result_outer(core_item_stats.get_stat_hp());
        }
        if let Some(options) = ehp {
            stats.ehp = get_ehp_stats(core_item_stats, options);
        }
        if *wc_ehp {
            stats.wc_ehp = StatResult::from_result_outer(core_item_stats.get_stat_wc_ehp());
        }
        if let Some(options) = rps {
            stats.rps = get_rps_stats(core_item_stats, options);
        }
        if let Some(options) = erps {
            stats.erps = get_erps_stats(core_item_stats, options);
        }
        if *breach_resist {
            stats.breach_resist = StatResult::from_result_outer(core_item_stats.get_stat_breach_resist());
        }
        // Cap
        if *cap_amount {
            stats.cap_amount = StatResult::from_result_outer(core_item_stats.get_stat_cap_amount());
        }
        if let Some(options) = cap_balance {
            stats.cap_balance = get_cap_balance_stats(core_item_stats, options);
        }
        if let Some(options) = cap_sim {
            stats.cap_sim = get_cap_sim_stats(core_item_stats, options);
        }
        if *neut_resist {
            stats.neut_resist = StatResult::from_result_outer(core_item_stats.get_stat_neut_resist());
        }
        // Sensors
        if *locks {
            stats.locks = StatResult::from_result_outer(core_item_stats.get_stat_locks());
        }
        if *lock_range {
            stats.lock_range = StatResult::from_result_outer(core_item_stats.get_stat_lock_range());
        }
        if *scan_res {
            stats.scan_res = StatResult::from_result_outer(core_item_stats.get_stat_scan_res());
        }
        if *sensors {
            stats.sensors = StatResult::from_result_outer(core_item_stats.get_stat_sensors());
        }
        if *dscan_range {
            stats.dscan_range = StatResult::from_result_outer(core_item_stats.get_stat_dscan_range());
        }
        if *probing_size {
            stats.probing_size = StatResult::from_result_outer(core_item_stats.get_stat_probing_size());
        }
        if let Some(options) = incoming_jam {
            stats.incoming_jam = get_incoming_jam_stats(core_item_stats, options);
        }
        // Mobility
        if *speed {
            stats.speed = StatResult::from_result_outer(core_item_stats.get_stat_speed());
        }
        if *agility {
            stats.agility = StatResult::from_result_outer(core_item_stats.get_stat_agility());
        }
        if *align_time {
            stats.align_time = StatResult::from_result_outer(core_item_stats.get_stat_align_time());
        }
        if *sig_radius {
            stats.sig_radius = StatResult::from_result_outer(core_item_stats.get_stat_sig_radius());
        }
        if let Some(options) = mass {
            stats.mass = get_mass_stats(core_item_stats, options);
        }
        if *warp_speed {
            stats.warp_speed = StatResult::from_result_outer(core_item_stats.get_stat_warp_speed());
        }
        if *max_warp_range {
            stats.max_warp_range = StatResult::from_result_outer(core_item_stats.get_stat_max_warp_range());
        }
        if let Some(options) = jump {
            stats.jump = get_jump_stats(core_item_stats, options);
        }
        // Misc
        if *drone_control_range {
            stats.drone_control_range = StatResult::from_result_outer(core_item_stats.get_stat_drone_control_range());
        }
        if *can_warp {
            stats.can_warp = StatResult::from_result_outer(core_item_stats.get_stat_can_warp());
        }
        if *can_jump_gate {
            stats.can_jump_gate = StatResult::from_result_outer(core_item_stats.get_stat_can_jump_gate());
        }
        if *can_jump_wormhole {
            stats.can_jump_wormhole = StatResult::from_result_outer(core_item_stats.get_stat_can_jump_wormhole());
        }
        if *can_jump_drive {
            stats.can_jump_drive = StatResult::from_result_outer(core_item_stats.get_stat_can_jump_drive());
        }
        if *can_dock_station {
            stats.can_dock_station = StatResult::from_result_outer(core_item_stats.get_stat_can_dock_station());
        }
        if *can_dock_citadel {
            stats.can_dock_citadel = StatResult::from_result_outer(core_item_stats.get_stat_can_dock_citadel());
        }
        if *can_tether {
            stats.can_tether = StatResult::from_result_outer(core_item_stats.get_stat_can_tether());
        }
        stats
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Output
////////////////////////////////////////////////////////////////////////////////////////////////////
// Output stats are available on both item stats and item stats with switched states
fn get_dmg_stat<S>(core_item_stats: &mut S, option: &StatOptionItemDmg) -> Result<StatDmg, StatItemAppliedError<!>>
where
    S: ItemStatsCommon,
{
    match option.projectee_item_id {
        Some(projectee_item_id) => core_item_stats
            .get_stat_dmg_applied(option.time, option.crits, option.charges, &projectee_item_id)
            .map(StatDmg::from_core_applied),
        None => core_item_stats
            .get_stat_dmg(option.time, option.crits, option.charges)
            .map(StatDmg::from_core)
            .map_err(conv_err_item),
    }
}
fn get_mps_stat<S>(core_item_stats: &mut S, option: &StatOptionItemMining) -> Result<StatMining, StatItemError<!>>
where
    S: ItemStatsCommon,
{
    core_item_stats.get_stat_mps(option.time, option.resource_kind)
}
fn get_outgoing_nps_stat<S>(
    core_item_stats: &mut S,
    option: &StatOptionItemOutNps,
) -> Result<PValue, StatItemAppliedError<!>>
where
    S: ItemStatsCommon,
{
    match option.projectee_item_id {
        Some(projectee_item_id) => {
            core_item_stats.get_stat_outgoing_nps_applied(option.time, option.charges, &projectee_item_id)
        }
        None => core_item_stats
            .get_stat_outgoing_nps(option.time, option.charges)
            .map_err(conv_err_item),
    }
}
fn get_outgoing_rps_stat<S>(
    core_item_stats: &mut S,
    option: &StatOptionItemOutRps,
) -> Result<StatOutReps, StatItemAppliedError<!>>
where
    S: ItemStatsCommon,
{
    match option.projectee_item_id {
        Some(projectee_item_id) => core_item_stats.get_stat_outgoing_rps_applied(option.time, &projectee_item_id),
        None => core_item_stats
            .get_stat_outgoing_rps(option.time)
            .map_err(conv_err_item),
    }
}
fn get_outgoing_cps_stat<S>(
    core_item_stats: &mut S,
    option: &StatOptionItemOutCps,
) -> Result<PValue, StatItemAppliedError<!>>
where
    S: ItemStatsCommon,
{
    match option.projectee_item_id {
        Some(projectee_item_id) => core_item_stats.get_stat_outgoing_cps_applied(option.time, &projectee_item_id),
        None => core_item_stats
            .get_stat_outgoing_cps(option.time)
            .map_err(conv_err_item),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Tank
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_ehp_stats(
    core_item_stats: &mut rc::stats::ItemStats,
    options: &[StatOptionEhp],
) -> StatResult<StatEhp, StatItemError<!>, !> {
    collect_stats_err_inner(options, |option| core_item_stats.get_stat_ehp(option.incoming_dps))
}
fn get_rps_stats(
    core_item_stats: &mut rc::stats::ItemStats,
    options: &[StatOptionRps],
) -> StatResult<StatRps, StatItemError<!>, !> {
    collect_stats_err_inner(options, |option| {
        core_item_stats.get_stat_rps(option.time, option.shield_perc)
    })
}
fn get_erps_stats(
    core_item_stats: &mut rc::stats::ItemStats,
    options: &[StatOptionErps],
) -> StatResult<StatErps, StatItemError<!>, !> {
    collect_stats_err_inner(options, |option| {
        core_item_stats.get_stat_erps(option.incoming_dps, option.time, option.shield_perc)
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cap
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_cap_balance_stats(
    core_item_stats: &mut rc::stats::ItemStats,
    options: &[Result<StatOptionCapBlc, BrResolveError>],
) -> StatResult<Value, StatItemAppliedError<!>, StatBrFallibleError<StatItemAppliedError<!>>> {
    collect_stats_err_both_br(options, |option| {
        core_item_stats.get_stat_cap_balance(option.src_kinds, option.time)
    })
}
fn get_cap_sim_stats(
    core_item_stats: &mut rc::stats::ItemStats,
    options: &[Result<StatOptionCapSim, BrResolveError>],
) -> StatResult<StatCapSim, StatItemAppliedError<!>, StatBrFallibleError<StatItemAppliedError<!>>> {
    collect_stats_err_both_br(options, |option| {
        core_item_stats.get_stat_cap_sim(
            option.cap_perc,
            option.optional_reloads,
            &option.stagger,
            option.nosf_projectee_item_id.as_ref(),
        )
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sensors
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_incoming_jam_stats(
    core_item_stats: &mut rc::stats::ItemStats,
    options: &[StatOptionIncomingJam],
) -> StatResult<StatInJam, StatItemError<!>, !> {
    collect_stats_err_inner(options, |option| core_item_stats.get_stat_incoming_jam(option.time))
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution getters - mobility
////////////////////////////////////////////////////////////////////////////////////////////////////
fn get_mass_stats(
    core_item_stats: &mut rc::stats::ItemStats,
    options: &[StatOptionMass],
) -> StatResult<PValue, StatItemError<!>, !> {
    collect_stats_err_inner(options, |option| core_item_stats.get_stat_mass(option.affectors))
}
fn get_jump_stats(
    core_item_stats: &mut rc::stats::ItemStats,
    options: &[StatOptionJump],
) -> StatResult<StatJump, StatItemError<StatJumpError>, !> {
    collect_stats_err_inner(options, |option| {
        core_item_stats.get_stat_jump(option.range, &option.passenger_fit_ids, option.passenger_fuel_affectors)
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Helpers
////////////////////////////////////////////////////////////////////////////////////////////////////
fn conv_err_item<SS>(err: StatItemError<SS>) -> StatItemAppliedError<SS>
where
    SS: std::error::Error,
{
    match err {
        StatItemError::ItemNotLoaded(err) => StatItemAppliedError::ItemNotLoaded(err),
        StatItemError::UnsupportedStat(err) => StatItemAppliedError::UnsupportedStat(err),
        StatItemError::StatSpecific(err) => StatItemAppliedError::StatSpecific(err),
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Switched options
////////////////////////////////////////////////////////////////////////////////////////////////////
trait SwitchingOption {
    fn is_switching(&self) -> bool;
}
impl SwitchingOption for StatOptionItemDmg {
    fn is_switching(&self) -> bool {
        matches!(self.state, StatItemStateOptions::Switch)
    }
}
impl SwitchingOption for StatOptionItemOutNps {
    fn is_switching(&self) -> bool {
        matches!(self.state, StatItemStateOptions::Switch)
    }
}
impl SwitchingOption for StatOptionItemMining {
    fn is_switching(&self) -> bool {
        matches!(self.state, StatItemStateOptions::Switch)
    }
}
impl SwitchingOption for StatOptionItemOutCps {
    fn is_switching(&self) -> bool {
        matches!(self.state, StatItemStateOptions::Switch)
    }
}
impl SwitchingOption for StatOptionItemOutRps {
    fn is_switching(&self) -> bool {
        matches!(self.state, StatItemStateOptions::Switch)
    }
}

// Results of switching options, in the order options are passed to a stat getter. Results are
// collected only when some option switches; when none does, the getter computes every option
struct SwitchedResults<T, E> {
    results: Vec<Option<Result<T, E>>>,
    switching: bool,
}
impl<T, E> SwitchedResults<T, E> {
    fn new<'a, O, I>(mut options: I) -> Self
    where
        O: SwitchingOption + 'a,
        I: Iterator<Item = &'a O>,
    {
        Self {
            results: Vec::new(),
            switching: options.any(|option| option.is_switching()),
        }
    }
    fn is_switching(&self) -> bool {
        self.switching
    }
    fn fill<'a, O, I, F>(&mut self, options: I, mut getter: F)
    where
        O: SwitchingOption + 'a,
        I: Iterator<Item = &'a O>,
        F: FnMut(&O) -> Result<T, E>,
    {
        if !self.switching {
            return;
        }
        self.results = options
            .map(|option| option.is_switching().then(|| getter(option)))
            .collect();
    }
    fn or_get<O, F>(&mut self, mut getter: F) -> impl FnMut(&O) -> Result<T, E>
    where
        F: FnMut(&O) -> Result<T, E>,
    {
        let mut results = std::mem::take(&mut self.results).into_iter();
        move |option| match results.next().flatten() {
            Some(result) => result,
            None => getter(option),
        }
    }
}
