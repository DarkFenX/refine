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
}

pub struct SolValFails {
    pub cpu: Option<SolResValFail>,
    pub pg: Option<SolResValFail>,
}
impl SolValFails {
    pub(in crate::sol::svc::vast) fn new() -> Self {
        Self { cpu: None, pg: None }
    }
}
