use crate::{def::AttrVal, ud::UProjRange};

/// Range between projector and projectee.
pub struct ProjRangeInfo {
    /// Center-to-center range.
    pub c2c: AttrVal,
    /// Surface-to-surface range.
    pub s2s: AttrVal,
}
impl From<UProjRange> for ProjRangeInfo {
    fn from(u_prange: UProjRange) -> Self {
        Self {
            c2c: u_prange.get_c2c(),
            s2s: u_prange.get_s2s(),
        }
    }
}
