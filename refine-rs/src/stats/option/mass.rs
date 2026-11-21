use crate::stats::StatCtlAffectors;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionMass {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) increasers: StatCtlAffectors = StatCtlAffectors::Unmodified,
}
const impl Default for StatOptionMass {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatOptionMass {
    pub const fn new() -> Self {
        Self::default()
    }
    pub const fn with_increasers(mut self, increasers: StatCtlAffectors) -> Self {
        self.increasers = increasers;
        self
    }
}
