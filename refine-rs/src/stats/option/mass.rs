use crate::stats::StatCtlAffectors;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionMass {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) increasers: StatCtlAffectors = StatCtlAffectors::Unmodified,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatOptionMass {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_increasers(mut self, increasers: StatCtlAffectors) -> Self {
        self.increasers = increasers;
        self
    }
}
