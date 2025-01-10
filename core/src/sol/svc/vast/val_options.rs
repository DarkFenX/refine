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
