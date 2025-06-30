#[derive(Copy, Clone)]
pub enum SecZone {
    HiSec(SecZoneCorruption),
    LowSec(SecZoneCorruption),
    NullSec,
    WSpace,
    Hazard,
}

#[derive(Copy, Clone)]
pub enum SecZoneCorruption {
    None,
    C5,
}
