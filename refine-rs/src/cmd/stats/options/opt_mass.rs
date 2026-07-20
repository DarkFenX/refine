use crate::stats::StatAffectors;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct StatOptionMass {
    #[cfg_attr(feature = "serde", serde(default))]
    pub affectors: StatAffectors = StatAffectors::Unmodified,
}
