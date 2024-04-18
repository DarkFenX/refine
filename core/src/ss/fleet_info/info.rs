use crate::{
    defs::{SsFitId, SsFleetId},
    ss::fleet::SsFleet,
};

pub struct SsFleetInfo {
    pub id: SsFleetId,
    pub fits: Vec<SsFitId>,
}
impl SsFleetInfo {
    pub(in crate::ss) fn new(id: SsFleetId, fits: Vec<SsFitId>) -> Self {
        Self { id, fits }
    }
}
impl From<&SsFleet> for SsFleetInfo {
    fn from(fleet: &SsFleet) -> Self {
        Self::new(fleet.id, fleet.iter_fits().map(|v| *v).collect())
    }
}
