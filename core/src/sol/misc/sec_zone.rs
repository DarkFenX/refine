#[derive(Copy, Clone)]
pub enum SolSecZone {
    HiSec(SolSecZoneCorruption),
    LowSec(SolSecZoneCorruption),
    NullSec,
    WSpace,
    Hazard,
}

#[derive(Copy, Clone)]
pub enum SolSecZoneCorruption {
    Any,
    C5,
}
