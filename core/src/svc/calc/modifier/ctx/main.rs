use crate::{
    svc::calc::{ModContext, RawModifier},
    ud::{UFitId, UItemId},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc::calc) struct CtxModifier {
    pub(in crate::svc::calc) raw: RawModifier,
    pub(in crate::svc::calc) ctx: ModContext,
}
impl CtxModifier {
    pub(in crate::svc::calc) fn new(raw: RawModifier) -> Self {
        Self {
            raw,
            ctx: ModContext::None,
        }
    }
    pub(in crate::svc::calc) fn new_with_item(raw: RawModifier, item_uid: UItemId) -> Self {
        Self {
            raw,
            ctx: ModContext::Item(item_uid),
        }
    }
    pub(in crate::svc::calc) fn new_with_fit(raw: RawModifier, fit_uid: UFitId) -> Self {
        Self {
            raw,
            ctx: ModContext::Fit(fit_uid),
        }
    }
    pub(in crate::svc::calc) fn new_with_fit_item(raw: RawModifier, fit_uid: UFitId, item_uid: UItemId) -> Self {
        Self {
            raw,
            ctx: ModContext::FitItem(fit_uid, item_uid),
        }
    }
}
