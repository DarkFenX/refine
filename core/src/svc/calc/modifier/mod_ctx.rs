use crate::{
    svc::calc::{Context, RawModifier},
    ud::{UFitId, UItemId},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc::calc) struct CtxModifier {
    pub(in crate::svc::calc) raw: RawModifier,
    pub(in crate::svc::calc) ctx: Context,
}
impl CtxModifier {
    pub(in crate::svc::calc) fn new(raw: RawModifier) -> Self {
        Self {
            raw,
            ctx: Context::None,
        }
    }
    pub(in crate::svc::calc) fn new_with_item(raw: RawModifier, item_key: UItemId) -> Self {
        Self {
            raw,
            ctx: Context::Item(item_key),
        }
    }
    pub(in crate::svc::calc) fn new_with_fit(raw: RawModifier, fit_key: UFitId) -> Self {
        Self {
            raw,
            ctx: Context::Fit(fit_key),
        }
    }
    pub(in crate::svc::calc) fn new_with_fit_item(raw: RawModifier, fit_key: UFitId, item_key: UItemId) -> Self {
        Self {
            raw,
            ctx: Context::FitItem(fit_key, item_key),
        }
    }
}
