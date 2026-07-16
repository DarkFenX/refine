use crate::stats::StatResource;

pub struct FitStats {
    // Fit resources
    pub cpu: Option<Vec<StatResource>> = None,
    pub powergrid: Option<Vec<StatResource>> = None,
    pub calibration: Option<Vec<StatResource>> = None,
    pub drone_bay_volume: Option<Vec<StatResource>> = None,
    pub drone_bandwidth: Option<Vec<StatResource>> = None,
    pub fighter_bay_volume: Option<Vec<StatResource>> = None,
}
