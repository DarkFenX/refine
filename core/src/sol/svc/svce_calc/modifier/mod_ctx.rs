use crate::{
    defs::{SolFitId, SolItemId},
    sol::svc::svce_calc::{SolContext, SolRawModifier},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::sol::svc::svce_calc) struct SolCtxModifier {
    pub(in crate::sol::svc::svce_calc) raw: SolRawModifier,
    pub(in crate::sol::svc::svce_calc) ctx: SolContext,
}
impl SolCtxModifier {
    pub(in crate::sol::svc::svce_calc) fn from_raw(raw: SolRawModifier) -> Self {
        Self {
            raw,
            ctx: SolContext::None,
        }
    }
    pub(in crate::sol::svc::svce_calc) fn from_raw_with_item(raw: SolRawModifier, ctx_item_id: SolItemId) -> Self {
        Self {
            raw,
            ctx: SolContext::Item(ctx_item_id),
        }
    }
    pub(in crate::sol::svc::svce_calc) fn from_raw_with_fit(raw: SolRawModifier, ctx_fit_id: SolFitId) -> Self {
        Self {
            raw,
            ctx: SolContext::Fit(ctx_fit_id),
        }
    }
}
