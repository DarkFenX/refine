use crate::uad::{UadFitKey, UadItemKey};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc::calc) enum Context {
    None,
    Item(UadItemKey),
    Fit(UadFitKey),
}
