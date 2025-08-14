use crate::{def::AttrVal, ud::UProjData};

/// Range between projector and projectee.
pub struct ProjRange {
    /// Center-to-center range.
    pub c2c: AttrVal,
    /// Surface-to-surface range.
    pub s2s: AttrVal,
}
impl From<UProjData> for ProjRange {
    fn from(u_proj_data: UProjData) -> Self {
        Self {
            c2c: u_proj_data.get_range_c2c(),
            s2s: u_proj_data.get_range_s2s(),
        }
    }
}
