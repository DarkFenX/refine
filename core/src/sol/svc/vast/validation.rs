use crate::sol::svc::vast::SolResValFail;

pub struct SolValOptions {
    pub cpu: bool,
    pub pg: bool,
    pub calibration: bool,
    pub dronebay_volume: bool,
}
impl SolValOptions {
    pub fn new(cpu: bool, pg: bool, calibration: bool, dronebay_volume: bool) -> Self {
        Self {
            cpu,
            pg,
            calibration,
            dronebay_volume,
        }
    }
    pub fn new_enabled() -> Self {
        Self {
            cpu: true,
            pg: true,
            calibration: true,
            dronebay_volume: true,
        }
    }
    pub fn new_disabled() -> Self {
        Self {
            cpu: false,
            pg: false,
            calibration: false,
            dronebay_volume: false,
        }
    }
}

pub struct SolValResult {
    pub cpu: Option<SolResValFail>,
    pub pg: Option<SolResValFail>,
    pub calibration: Option<SolResValFail>,
    pub dronebay_volume: Option<SolResValFail>,
}
impl SolValResult {
    pub(in crate::sol::svc::vast) fn new() -> Self {
        Self {
            cpu: None,
            pg: None,
            calibration: None,
            dronebay_volume: None,
        }
    }
    pub fn all_passed(&self) -> bool {
        self.cpu.is_none() && self.pg.is_none() && self.calibration.is_none() && self.dronebay_volume.is_none()
    }
}
