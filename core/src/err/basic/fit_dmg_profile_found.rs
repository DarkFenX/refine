use crate::defs::SolFitId;

#[derive(Debug)]
pub struct FitDmgProfileFoundError {
    pub fit_id: SolFitId,
}
impl FitDmgProfileFoundError {
    pub(crate) fn new(fit_id: SolFitId) -> Self {
        Self { fit_id }
    }
}
impl std::error::Error for FitDmgProfileFoundError {}
impl std::fmt::Display for FitDmgProfileFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "damage profile not found on fit  {}", self.fit_id)
    }
}
