use rc::CtlAffectors;

use crate::{
    CmdResps, FitId, FitIdBr,
    shared::BrResolveInfallible,
    stats::{StatAffectors, StatJumpRange},
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct StatOptionJump<F = FitId> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) range: StatJumpRange = StatJumpRange::Max,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) passenger_fit_ids: Vec<F> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) passenger_fuel_affectors: StatAffectors = CtlAffectors::Unmodified,
}
impl<F> Default for StatOptionJump<F> {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<F> StatOptionJump<F> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_range(mut self, range: StatJumpRange) -> Self {
        self.range = range;
        self
    }
    pub fn with_passenger_fit_ids(mut self, passenger_fit_ids: impl Iterator<Item = F>) -> Self {
        self.passenger_fit_ids.extend(passenger_fit_ids);
        self
    }
    pub fn with_passenger_fuel_affectors(mut self, passenger_fuel_affectors: StatAffectors) -> Self {
        self.passenger_fuel_affectors = passenger_fuel_affectors;
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BrResolveInfallible for StatOptionJump<FitIdBr> {
    type Target = StatOptionJump<FitId>;
    fn br_resolve_infallible(self, resps: &CmdResps) -> Self::Target {
        Self::Target {
            range: self.range,
            passenger_fit_ids: resps.resolve_fit_ids_lossy(self.passenger_fit_ids),
            passenger_fuel_affectors: self.passenger_fuel_affectors,
        }
    }
}
