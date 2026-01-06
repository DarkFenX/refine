use crate::{misc::PValue, ud::UProjData};

/// Range between projector and projectee.
pub struct ProjRange {
    /// Center-to-center range.
    pub c2c: PValue,
    /// Surface-to-surface range.
    pub s2s: PValue,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ProjRange {
    pub(in crate::api) fn from_u_proj_data(u_proj_data: UProjData) -> Self {
        Self {
            c2c: u_proj_data.get_range_c2c(),
            s2s: u_proj_data.get_range_s2s(),
        }
    }
}
