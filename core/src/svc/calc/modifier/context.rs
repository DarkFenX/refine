use crate::ud::{UFitKey, UItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc::calc) enum Context {
    None,
    Item(UItemKey),
    Fit(UFitKey),
    FitItem(UFitKey, UItemKey),
}
impl Context {
    pub(in crate::svc::calc) fn get_item_key(&self) -> Option<UItemKey> {
        match self {
            Self::Item(projectee_key) => Some(*projectee_key),
            Self::FitItem(_, projectee_key) => Some(*projectee_key),
            _ => None,
        }
    }
}
