use crate::sol::FitId;

#[derive(Debug)]
pub struct FitFoundError {
    pub fit_id: FitId,
}
impl FitFoundError {
    pub(crate) fn new(fit_id: FitId) -> Self {
        Self { fit_id }
    }
}
impl std::error::Error for FitFoundError {}
impl std::fmt::Display for FitFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fit {} not found", self.fit_id)
    }
}
