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
    None,
    C5,
}
