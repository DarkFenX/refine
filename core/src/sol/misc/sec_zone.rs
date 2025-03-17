#[derive(Copy, Clone)]
pub enum SolSecZone {
    HiSec(bool),
    LowSec(bool),
    NullSec,
    WSpace,
    Hazard,
}
