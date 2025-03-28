use crate::sol::FitId;

#[derive(Debug)]
pub struct FitFleetAssignedError {
    pub fit_id: FitId,
}
impl std::error::Error for FitFleetAssignedError {}
impl std::fmt::Display for FitFleetAssignedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fit {} does not belong to any fleet", self.fit_id)
    }
}
