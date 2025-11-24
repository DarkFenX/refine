use crate::ud::{UFitKey, UItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc::calc) enum Context {
    None,
    Fit(UFitKey),
    ProjItem(UItemKey),
    ProjFitItem(UFitKey, UItemKey),
}
impl Context {
    pub(in crate::svc::calc) fn get_projectee_key(&self) -> Option<UItemKey> {
        match self {
            Self::ProjItem(projectee_key) => Some(*projectee_key),
            Self::ProjFitItem(_, projectee_key) => Some(*projectee_key),
            _ => None,
        }
    }
}
