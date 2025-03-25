use crate::sol::{FitId, ItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::calc) enum Context {
    None,
    Item(ItemId),
    Fit(FitId),
}
