use crate::sol::FitId;

#[derive(Debug)]
pub struct FitDpsProfileFoundError {
    pub fit_id: FitId,
}
impl std::error::Error for FitDpsProfileFoundError {}
impl std::fmt::Display for FitDpsProfileFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DPS profile not found on fit  {}", self.fit_id)
    }
}
