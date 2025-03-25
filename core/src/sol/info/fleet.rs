use crate::sol::{FitId, FleetId, uad::fleet::Fleet};

pub struct FleetInfo {
    pub id: FleetId,
    pub fits: Vec<FitId>,
}
impl FleetInfo {
    pub(in crate::sol) fn new(id: FleetId, fits: Vec<FitId>) -> Self {
        Self { id, fits }
    }
}
impl From<&Fleet> for FleetInfo {
    fn from(fleet: &Fleet) -> Self {
        Self::new(fleet.id, fleet.iter_fits().copied().collect())
    }
}
