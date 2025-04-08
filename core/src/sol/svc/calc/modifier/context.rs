use crate::sol::{FitKey, ItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::calc) enum Context {
    None,
    Item(ItemKey),
    Fit(FitKey),
}
