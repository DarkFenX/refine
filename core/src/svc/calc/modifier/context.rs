use crate::ud::{UFitId, UItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc::calc) enum Context {
    None,
    Item(UItemId),
    Fit(UFitId),
    FitItem(UFitId, UItemId),
}
impl Context {
    pub(in crate::svc::calc) fn get_item_uid(&self) -> Option<UItemId> {
        match self {
            Self::Item(projectee_uid) => Some(*projectee_uid),
            Self::FitItem(_, projectee_uid) => Some(*projectee_uid),
            _ => None,
        }
    }
}
