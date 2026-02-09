use crate::num::Count;

#[derive(Copy, Clone)]
pub struct SpoolCycleCountInfo {
    pub current: Count,
    pub max: Count,
    pub overridden: bool,
}
