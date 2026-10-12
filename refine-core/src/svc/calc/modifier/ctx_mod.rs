use crate::{
    dbg::DebugResult,
    svc::calc::{ModContext, RawModId, RawModifier, registers::RawModStorage},
    ud::{UData, UFitId, UItemId},
};

/// CtxModifier's identity, stored in registers to avoid storing full modifiers.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc::calc) struct CtxModKey {
    pub(in crate::svc::calc) rmod_id: RawModId,
    pub(in crate::svc::calc) ctx: ModContext,
}

#[derive(Copy, Clone)]
pub(in crate::svc::calc) struct CtxModifier {
    pub(in crate::svc::calc) rmod: RawModifier,
    pub(in crate::svc::calc) ctx: ModContext,
}
impl CtxModifier {
    pub(in crate::svc::calc) fn new(rmod: RawModifier) -> Self {
        Self {
            rmod,
            ctx: ModContext::None,
        }
    }
    pub(in crate::svc::calc) fn with_item(rmod: RawModifier, item_uid: UItemId) -> Self {
        Self {
            rmod,
            ctx: ModContext::Item(item_uid),
        }
    }
    pub(in crate::svc::calc) fn with_fit(rmod: RawModifier, fit_uid: UFitId) -> Self {
        Self {
            rmod,
            ctx: ModContext::Fit(fit_uid),
        }
    }
    pub(in crate::svc::calc) fn with_fit_item(rmod: RawModifier, fit_uid: UFitId, item_uid: UItemId) -> Self {
        Self {
            rmod,
            ctx: ModContext::FitItem(fit_uid, item_uid),
        }
    }
    pub(in crate::svc::calc) fn from_ckey(rmod: RawModifier, ckey: CtxModKey) -> Self {
        Self { rmod, ctx: ckey.ctx }
    }
    pub(in crate::svc::calc) fn get_ckey(&self, rmod_id: RawModId) -> CtxModKey {
        CtxModKey { rmod_id, ctx: self.ctx }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Debugging
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CtxModKey {
    pub(in crate::svc::calc) fn consistency_check(&self, u_data: &UData, rmods: &RawModStorage) -> DebugResult {
        match rmods.try_get_rmod(self.rmod_id) {
            Some(rmod) => CtxModifier::from_ckey(*rmod, *self).consistency_check(u_data),
            None => Err(Default::default()),
        }
    }
}

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
        self.rmod.consistency_check(u_data)?;
        Ok(())
    }
}
