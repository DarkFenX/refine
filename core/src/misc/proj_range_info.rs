use crate::{def::AttrVal, uad::UadProjRange};

/// Range between projector and projectee.
pub struct ProjRangeInfo {
    /// Center-to-center range.
    pub c2c: AttrVal,
    /// Surface-to-surface range.
    pub s2s: AttrVal,
}
impl From<UadProjRange> for ProjRangeInfo {
    fn from(uad_prange: UadProjRange) -> Self {
        Self {
            c2c: uad_prange.c2c,
            s2s: uad_prange.s2s,
        }
    }
}
