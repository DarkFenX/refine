use crate::sol::{
    FitId, FleetId,
    uad::{Uad, fleet::UadFleet},
};

pub struct FleetInfo {
    pub id: FleetId,
    pub fits: Vec<FitId>,
}
impl FleetInfo {
    pub(in crate::sol) fn from_fleet(uad: &Uad, fleet: &UadFleet) -> Self {
        Self {
            id: fleet.id,
            fits: fleet.iter_fits().map(|fit_key| uad.fits.id_by_key(*fit_key)).collect(),
        }
    }
}
