use crate::sol::svc::vast::SolResValFail;

pub struct SolValOptions {
    pub cpu: bool,
    pub pg: bool,
}
impl SolValOptions {
    pub fn new(cpu: bool, pg: bool) -> Self {
        Self { cpu, pg }
    }
    pub fn new_enabled() -> Self {
        Self { cpu: true, pg: true }
    }
    pub fn new_disabled() -> Self {
        Self { cpu: false, pg: false }
    }
}

pub struct SolValResult {
    pub cpu: Option<SolResValFail>,
    pub pg: Option<SolResValFail>,
}
impl SolValResult {
    pub(in crate::sol::svc::vast) fn new() -> Self {
        Self { cpu: None, pg: None }
    }
    pub fn all_passed(&self) -> bool {
        self.cpu.is_none() && self.pg.is_none()
    }
}
