use crate::stats::StatAffectors;

#[derive(Copy, Clone, Default)]
pub struct StatOptionMass {
    pub affectors: StatAffectors = StatAffectors::Unmodified,
}
