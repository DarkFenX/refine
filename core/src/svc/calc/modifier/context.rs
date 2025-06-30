use crate::def::{FitKey, ItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc::calc) enum Context {
    None,
    Item(ItemKey),
    Fit(FitKey),
}
