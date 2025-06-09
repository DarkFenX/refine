use crate::{cmd::shared::get_primary_fit, info::HFitStats, util::HExecError};

#[derive(serde::Deserialize, Default)]
pub(crate) struct HGetFitStatsCmd {
    default: Option<bool>,
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
}
impl HGetFitStatsCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem, fit_id: &rc::FitId) -> Result<HFitStats, HExecError> {
        let mut core_fit = get_primary_fit(core_sol, fit_id)?;
        let default = self.default.unwrap_or(false);
        let mut stats = HFitStats::new();
        if self.high_slots.unwrap_or(default) {
            stats.high_slots = Some(core_fit.get_stat_high_slots().into());
        }
        if self.mid_slots.unwrap_or(default) {
            stats.mid_slots = Some(core_fit.get_stat_mid_slots().into());
        }
        if self.low_slots.unwrap_or(default) {
            stats.low_slots = Some(core_fit.get_stat_low_slots().into());
        }
        if self.turret_slots.unwrap_or(default) {
            stats.turret_slots = Some(core_fit.get_stat_turret_slots().into());
        }
        if self.launcher_slots.unwrap_or(default) {
            stats.launcher_slots = Some(core_fit.get_stat_launcher_slots().into());
        }
        if self.rig_slots.unwrap_or(default) {
            stats.rig_slots = Some(core_fit.get_stat_rig_slots().into());
        }
        if self.service_slots.unwrap_or(default) {
            stats.service_slots = Some(core_fit.get_stat_service_slots().into());
        }
        if self.subsystem_slots.unwrap_or(default) {
            stats.subsystem_slots = Some(core_fit.get_stat_subsystem_slots().into());
        }
        if self.launched_drones.unwrap_or(default) {
            stats.launched_drones = Some(core_fit.get_stat_launched_drones().into());
        }
        if self.launched_fighters.unwrap_or(default) {
            stats.launched_fighters = Some(core_fit.get_stat_launched_fighters().into());
        }
        if self.launched_light_fighters.unwrap_or(default) {
            stats.launched_light_fighters = Some(core_fit.get_stat_launched_light_fighters().into());
        }
        if self.launched_heavy_fighters.unwrap_or(default) {
            stats.launched_heavy_fighters = Some(core_fit.get_stat_launched_heavy_fighters().into());
        }
        if self.launched_support_fighters.unwrap_or(default) {
            stats.launched_support_fighters = Some(core_fit.get_stat_launched_support_fighters().into());
        }
        if self.launched_st_light_fighters.unwrap_or(default) {
            stats.launched_st_light_fighters = Some(core_fit.get_stat_launched_st_light_fighters().into());
        }
        if self.launched_st_heavy_fighters.unwrap_or(default) {
            stats.launched_st_heavy_fighters = Some(core_fit.get_stat_launched_st_heavy_fighters().into());
        }
        if self.launched_st_support_fighters.unwrap_or(default) {
            stats.launched_st_support_fighters = Some(core_fit.get_stat_launched_st_support_fighters().into());
        }
        if self.cpu.unwrap_or(default) {
            stats.cpu = Some(core_fit.get_stat_cpu().into());
        }
        if self.powergrid.unwrap_or(default) {
            stats.powergrid = Some(core_fit.get_stat_powergrid().into());
        }
        if self.calibration.unwrap_or(default) {
            stats.calibration = Some(core_fit.get_stat_calibration().into());
        }
        if self.drone_bay_volume.unwrap_or(default) {
            stats.drone_bay_volume = Some(core_fit.get_stat_drone_bay_volume().into());
        }
        if self.drone_bandwidth.unwrap_or(default) {
            stats.drone_bandwidth = Some(core_fit.get_stat_drone_bandwidth().into());
        }
        if self.fighter_bay_volume.unwrap_or(default) {
            stats.fighter_bay_volume = Some(core_fit.get_stat_fighter_bay_volume().into());
        }
        Ok(stats)
    }
}
