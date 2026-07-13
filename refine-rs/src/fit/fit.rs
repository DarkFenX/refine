use crate::sol::SolarSystem;

pub struct Fit<'r, 's> {
    pub(super) sol: &'s mut SolarSystem<'r>,
    pub(super) id: rc::FitId,
}
impl<'r, 's> Fit<'r, 's> {
    pub fn get_fit_id(&self) -> rc::FitId {
        self.id
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'r, 's> Fit<'r, 's> {
    pub(super) fn new(sol: &'s mut SolarSystem<'r>, id: rc::FitId) -> Self {
        Self { sol, id }
    }
}
