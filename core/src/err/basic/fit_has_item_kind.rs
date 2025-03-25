use crate::sol::FitId;

#[derive(Debug)]
pub struct FitHasItemKindError {
    pub fit_id: FitId,
    pub item_kind: &'static str,
}
impl FitHasItemKindError {
    pub(crate) fn new(fit_id: FitId, item_kind: &'static str) -> Self {
        Self { fit_id, item_kind }
    }
}
impl std::error::Error for FitHasItemKindError {}
impl std::fmt::Display for FitHasItemKindError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fit {} does not have {} set", self.fit_id, self.item_kind)
    }
}
