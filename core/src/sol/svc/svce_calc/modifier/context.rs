use crate::defs::{SolFitId, SolItemId};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::svce_calc) enum SolContext {
    None,
    Item(SolItemId),
    Fit(SolFitId),
}
