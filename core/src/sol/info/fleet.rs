use crate::sol::{FitId, FleetId, uad::fleet::Fleet};

pub struct FleetInfo {
    pub id: FleetId,
    pub fits: Vec<FitId>,
}
impl From<&Fleet> for FleetInfo {
    fn from(fleet: &Fleet) -> Self {
        Self {
            id: fleet.id,
            fits: fleet.iter_fits().copied().collect(),
        }
    }
}
