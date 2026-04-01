use crate::{
    dbg::DebugResult,
    svc::calc::{ModContext, RawModifier},
    ud::{UData, UFitId, UItemId},
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CtxModifier {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        match self.ctx {
            // Item modifier is applied to is not necessarily loaded (e.g. a module projected to a
            // non-loaded ship)
            ModContext::None => (),
            ModContext::Item(item_uid) => item_uid.consistency_check(u_data, false)?,
            ModContext::Fit(fit_uid) => fit_uid.consistency_check(u_data)?,
            ModContext::FitItem(fit_uid, item_uid) => {
                fit_uid.consistency_check(u_data)?;
                item_uid.consistency_check(u_data, false)?;
            }
        }
        self.raw.consistency_check(u_data)?;
        Ok(())
    }
}
