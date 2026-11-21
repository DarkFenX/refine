use crate::stats::StatCtlAffectors;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionDroneBandwidth {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) decreasers: StatCtlAffectors = StatCtlAffectors::Unmodified,
}
const impl Default for StatOptionDroneBandwidth {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatOptionDroneBandwidth {
    pub const fn new() -> Self {
        Self::default()
    }
    pub const fn with_decreasers(mut self, decreasers: StatCtlAffectors) -> Self {
        self.decreasers = decreasers;
        self
    }
}
