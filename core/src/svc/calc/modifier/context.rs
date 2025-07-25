use crate::ud::{UFitKey, UItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc::calc) enum Context {
    None,
    Item(UItemKey),
    Fit(UFitKey),
}
