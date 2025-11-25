use crate::{
    svc::calc::{Context, RawModifier},
    ud::{UFitKey, UItemKey},
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
    pub(in crate::svc::calc) fn new_with_fit(raw: RawModifier, fit_key: UFitKey) -> Self {
        Self {
            raw,
            ctx: Context::Fit(fit_key),
        }
    }
    pub(in crate::svc::calc) fn new_with_projectee_item(raw: RawModifier, item_key: UItemKey) -> Self {
        Self {
            raw,
            ctx: Context::ProjItem(item_key),
        }
    }
    pub(in crate::svc::calc) fn new_with_projectee_fit_item(
        raw: RawModifier,
        fit_key: UFitKey,
        item_key: UItemKey,
    ) -> Self {
        Self {
            raw,
            ctx: Context::ProjFitItem(fit_key, item_key),
        }
    }
}
