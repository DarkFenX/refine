use crate::sol::svc::vast::SolResValFail;

pub struct SolValOptions {
    pub cpu: bool,
    pub powergrid: bool,
    pub calibration: bool,
    pub dronebay_volume: bool,
}
impl SolValOptions {
    pub fn new(cpu: bool, powergrid: bool, calibration: bool, dronebay_volume: bool) -> Self {
        Self {
            cpu,
            powergrid,
            calibration,
            dronebay_volume,
        }
    }
    pub fn new_enabled() -> Self {
        Self {
            cpu: true,
            powergrid: true,
            calibration: true,
            dronebay_volume: true,
        }
    }
    pub fn new_disabled() -> Self {
        Self {
            cpu: false,
            powergrid: false,
            calibration: false,
            dronebay_volume: false,
        }
    }
}

pub struct SolValResult {
    pub cpu: Option<SolResValFail>,
    pub powergrid: Option<SolResValFail>,
    pub calibration: Option<SolResValFail>,
    pub dronebay_volume: Option<SolResValFail>,
}
impl SolValResult {
    pub(in crate::sol::svc::vast) fn new() -> Self {
        Self {
            cpu: None,
            powergrid: None,
            calibration: None,
            dronebay_volume: None,
        }
    }
    pub fn all_passed(&self) -> bool {
        self.cpu.is_none() && self.powergrid.is_none() && self.calibration.is_none() && self.dronebay_volume.is_none()
    }
}
