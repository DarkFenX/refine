use rc::CtlAffectors;

use crate::{
    FitId,
    stats::{StatAffectors, StatJumpRange},
};

#[derive(Clone, Default)]
pub struct StatOptionJump {
    pub range: StatJumpRange = StatJumpRange::Max,
    pub passenger_fit_ids: Vec<FitId> = Vec::new(),
    pub passenger_fuel_affectors: StatAffectors = CtlAffectors::Unmodified,
}
