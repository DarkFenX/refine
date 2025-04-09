use crate::sol::FitId;

#[derive(thiserror::Error, Debug)]
#[error("fit {fit_id} does not have {item_kind} set")]
pub struct FitHasItemKindError {
    pub fit_id: FitId,
    pub item_kind: &'static str,
}
