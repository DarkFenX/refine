use crate::stats::StatAffectors;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionDroneBandwidth {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) affectors: StatAffectors = StatAffectors::Unmodified,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl StatOptionDroneBandwidth {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_affectors(mut self, affectors: StatAffectors) -> Self {
        self.affectors = affectors;
        self
    }
}
