use crate::sol::FitId;

#[derive(Debug)]
pub struct FitFoundError {
    pub fit_id: FitId,
}
impl std::error::Error for FitFoundError {}
impl std::fmt::Display for FitFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fit {} not found", self.fit_id)
    }
}
