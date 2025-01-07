use crate::{
    defs::{SolFitId, SolFleetId},
    sol::uad::fleet::SolFleet,
};

pub struct SolFleetInfo {
    pub id: SolFleetId,
    pub fits: Vec<SolFitId>,
}
impl SolFleetInfo {
    pub(in crate::sol) fn new(id: SolFleetId, fits: Vec<SolFitId>) -> Self {
        Self { id, fits }
    }
}
impl From<&SolFleet> for SolFleetInfo {
    fn from(fleet: &SolFleet) -> Self {
        Self::new(fleet.id, fleet.iter_fits().map(|v| *v).collect())
    }
}
