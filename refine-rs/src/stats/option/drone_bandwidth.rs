use crate::stats::StatCtlAffectors;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionDroneBandwidth {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) decreasers: StatCtlAffectors = StatCtlAffectors::Unmodified,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatOptionDroneBandwidth {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_decreasers(mut self, decreasers: StatCtlAffectors) -> Self {
        self.decreasers = decreasers;
        self
    }
}
