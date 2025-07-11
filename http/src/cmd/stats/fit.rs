use crate::{
    cmd::{
        shared::get_primary_fit,
        stats::options::{HStatOption, HStatOptionEhp, HStatOptionRr},
    },
    info::HFitStats,
    util::HExecError,
};

#[derive(educe::Educe, serde::Deserialize)]
#[educe(Default)]
pub(crate) struct HGetFitStatsCmd {
    #[educe(Default = true)]
    default: bool,
    high_slots: Option<bool>,
    mid_slots: Option<bool>,
    low_slots: Option<bool>,
    turret_slots: Option<bool>,
    launcher_slots: Option<bool>,
    rig_slots: Option<bool>,
    service_slots: Option<bool>,
    subsystem_slots: Option<bool>,
    launched_drones: Option<bool>,
    launched_fighters: Option<bool>,
    launched_light_fighters: Option<bool>,
    launched_heavy_fighters: Option<bool>,
    launched_support_fighters: Option<bool>,
    launched_st_light_fighters: Option<bool>,
    launched_st_heavy_fighters: Option<bool>,
    launched_st_support_fighters: Option<bool>,
    cpu: Option<bool>,
    powergrid: Option<bool>,
    calibration: Option<bool>,
    drone_bay_volume: Option<bool>,
    drone_bandwidth: Option<bool>,
    fighter_bay_volume: Option<bool>,
    agility: Option<bool>,
    align_time: Option<bool>,
    speed: Option<bool>,
    hp: Option<bool>,
    ehp: Option<HStatOption<HStatOptionEhp>>,
    wc_ehp: Option<bool>,
    resists: Option<bool>,
    rr_shield: Option<HStatOption<HStatOptionRr>>,
    rr_armor: Option<HStatOption<HStatOptionRr>>,
    rr_hull: Option<HStatOption<HStatOptionRr>>,
    rr_capacitor: Option<HStatOption<HStatOptionRr>>,
}
impl HGetFitStatsCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<HFitStats, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let mut stats = HFitStats::new();
        if self.high_slots.unwrap_or(self.default) {
            stats.high_slots = Some(core_fit.get_stat_high_slots().into());
        }
        if self.mid_slots.unwrap_or(self.default) {
            stats.mid_slots = Some(core_fit.get_stat_mid_slots().into());
        }
        if self.low_slots.unwrap_or(self.default) {
            stats.low_slots = Some(core_fit.get_stat_low_slots().into());
        }
        if self.turret_slots.unwrap_or(self.default) {
            stats.turret_slots = Some(core_fit.get_stat_turret_slots().into());
        }
        if self.launcher_slots.unwrap_or(self.default) {
            stats.launcher_slots = Some(core_fit.get_stat_launcher_slots().into());
        }
        if self.rig_slots.unwrap_or(self.default) {
            stats.rig_slots = Some(core_fit.get_stat_rig_slots().into());
        }
        if self.service_slots.unwrap_or(self.default) {
            stats.service_slots = Some(core_fit.get_stat_service_slots().into());
        }
        if self.subsystem_slots.unwrap_or(self.default) {
            stats.subsystem_slots = Some(core_fit.get_stat_subsystem_slots().into());
        }
        if self.launched_drones.unwrap_or(self.default) {
            stats.launched_drones = Some(core_fit.get_stat_launched_drones().into());
        }
        if self.launched_fighters.unwrap_or(self.default) {
            stats.launched_fighters = Some(core_fit.get_stat_launched_fighters().into());
        }
        if self.launched_light_fighters.unwrap_or(self.default) {
            stats.launched_light_fighters = Some(core_fit.get_stat_launched_light_fighters().into());
        }
        if self.launched_heavy_fighters.unwrap_or(self.default) {
            stats.launched_heavy_fighters = Some(core_fit.get_stat_launched_heavy_fighters().into());
        }
        if self.launched_support_fighters.unwrap_or(self.default) {
            stats.launched_support_fighters = Some(core_fit.get_stat_launched_support_fighters().into());
        }
        if self.launched_st_light_fighters.unwrap_or(self.default) {
            stats.launched_st_light_fighters = Some(core_fit.get_stat_launched_st_light_fighters().into());
        }
        if self.launched_st_heavy_fighters.unwrap_or(self.default) {
            stats.launched_st_heavy_fighters = Some(core_fit.get_stat_launched_st_heavy_fighters().into());
        }
        if self.launched_st_support_fighters.unwrap_or(self.default) {
            stats.launched_st_support_fighters = Some(core_fit.get_stat_launched_st_support_fighters().into());
        }
        if self.cpu.unwrap_or(self.default) {
            stats.cpu = Some(core_fit.get_stat_cpu().into());
        }
        if self.powergrid.unwrap_or(self.default) {
            stats.powergrid = Some(core_fit.get_stat_powergrid().into());
        }
        if self.calibration.unwrap_or(self.default) {
            stats.calibration = Some(core_fit.get_stat_calibration().into());
        }
        if self.drone_bay_volume.unwrap_or(self.default) {
            stats.drone_bay_volume = Some(core_fit.get_stat_drone_bay_volume().into());
        }
        if self.drone_bandwidth.unwrap_or(self.default) {
            stats.drone_bandwidth = Some(core_fit.get_stat_drone_bandwidth().into());
        }
        if self.fighter_bay_volume.unwrap_or(self.default) {
            stats.fighter_bay_volume = Some(core_fit.get_stat_fighter_bay_volume().into());
        }
        if self.agility.unwrap_or(self.default) {
            stats.agility = core_fit.get_stat_agility().into();
        }
        if self.align_time.unwrap_or(self.default) {
            stats.align_time = core_fit.get_stat_align_time().into();
        }
        if self.speed.unwrap_or(self.default) {
            stats.speed = core_fit.get_stat_speed().into();
        }
        if self.hp.unwrap_or(self.default) {
            stats.hp = core_fit.get_stat_hp().into();
        }
        let ehp_opt = LocalOpt::new(&self.ehp, self.default);
        if ehp_opt.enabled {
            stats.ehp = Some(
                ehp_opt
                    .options
                    .iter()
                    .map(|inner_opt| {
                        let core_incoming_dps = inner_opt.incoming_dps.map(|h_incoming_dps| h_incoming_dps.into());
                        core_fit
                            .get_stat_ehp(core_incoming_dps.as_ref())
                            .map(|core_ehp| core_ehp.into())
                    })
                    .collect(),
            )
        }
        if self.wc_ehp.unwrap_or(self.default) {
            stats.wc_ehp = core_fit.get_stat_wc_ehp().into();
        }
        if self.resists.unwrap_or(self.default) {
            stats.resists = core_fit.get_stat_resists().into();
        }
        let rr_shield_opt = LocalOpt::new(&self.rr_shield, self.default);
        if rr_shield_opt.enabled {
            stats.rr_shield = Some(
                rr_shield_opt
                    .options
                    .iter()
                    .map(|inner_opt| core_fit.get_stat_rr_shield(inner_opt.spool.map(|spool| spool.into())))
                    .collect(),
            )
        }
        let rr_armor_opt = LocalOpt::new(&self.rr_armor, self.default);
        if rr_armor_opt.enabled {
            stats.rr_armor = Some(
                rr_armor_opt
                    .options
                    .iter()
                    .map(|inner_opt| core_fit.get_stat_rr_armor(inner_opt.spool.map(|spool| spool.into())))
                    .collect(),
            )
        }
        let rr_hull_opt = LocalOpt::new(&self.rr_hull, self.default);
        if rr_hull_opt.enabled {
            stats.rr_hull = Some(
                rr_hull_opt
                    .options
                    .iter()
                    .map(|inner_opt| core_fit.get_stat_rr_hull(inner_opt.spool.map(|spool| spool.into())))
                    .collect(),
            )
        }
        let rr_capacitor_opt = LocalOpt::new(&self.rr_capacitor, self.default);
        if rr_capacitor_opt.enabled {
            stats.rr_capacitor = Some(
                rr_capacitor_opt
                    .options
                    .iter()
                    .map(|inner_opt| core_fit.get_stat_rr_capacitor(inner_opt.spool.map(|spool| spool.into())))
                    .collect(),
            )
        }
        Ok(stats)
    }
}

struct LocalOpt<T> {
    enabled: bool,
    options: Vec<T>,
}
impl<T> LocalOpt<T>
where
    T: Copy + Clone + Default,
{
    fn new(root_opt: &Option<HStatOption<T>>, default: bool) -> Self {
        match root_opt {
            Some(inner_opt) => LocalOpt {
                enabled: inner_opt.is_enabled(),
                options: inner_opt.get_extended_options(),
            },
            None => match default {
                true => LocalOpt {
                    enabled: true,
                    options: vec![T::default()],
                },
                // No need to allocate anything if check is disabled
                false => LocalOpt {
                    enabled: false,
                    options: Vec::new(),
                },
            },
        }
    }
}
