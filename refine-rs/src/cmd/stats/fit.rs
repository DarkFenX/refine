use crate::stats::{FitStats, StatOption};

#[derive(Default)]
pub struct GetFitStatsCmd {
    pub default: bool = true,
    // Fit resources
    cpu: StatOption = StatOption::Default,
    powergrid: StatOption = StatOption::Default,
    calibration: StatOption = StatOption::Default,
    drone_bay_volume: StatOption = StatOption::Default,
    drone_bandwidth: StatOption = StatOption::Default,
    fighter_bay_volume: StatOption = StatOption::Default,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl GetFitStatsCmd {
    pub(crate) fn execute(self, core_fit: &mut rc::FitMut) -> FitStats {
        let mut stats = FitStats { .. };
        if self.cpu.is_enabled(self.default) {
            stats.cpu = Some(vec![core_fit.get_stat_cpu()])
        }
        if self.powergrid.is_enabled(self.default) {
            stats.powergrid = Some(vec![core_fit.get_stat_powergrid()])
        }
        if self.calibration.is_enabled(self.default) {
            stats.calibration = Some(vec![core_fit.get_stat_calibration()])
        }
        if self.drone_bay_volume.is_enabled(self.default) {
            stats.drone_bay_volume = Some(vec![core_fit.get_stat_drone_bay_volume()])
        }
        if self.drone_bandwidth.is_enabled(self.default) {
            stats.drone_bandwidth = Some(vec![core_fit.get_stat_drone_bandwidth()])
        }
        if self.fighter_bay_volume.is_enabled(self.default) {
            stats.fighter_bay_volume = Some(vec![core_fit.get_stat_fighter_bay_volume()])
        }
        stats
    }
}
