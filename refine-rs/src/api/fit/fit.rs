use crate::{FitId, SolarSystem};

pub struct Fit<'r, 's> {
    pub(super) sol: &'s mut SolarSystem<'r>,
    pub(super) id: FitId,
}
impl<'r, 's> Fit<'r, 's> {
    pub fn get_fit_id(&self) -> FitId {
        self.id
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'r, 's> Fit<'r, 's> {
    pub(super) fn new(sol: &'s mut SolarSystem<'r>, id: FitId) -> Self {
        Self { sol, id }
    }
}
