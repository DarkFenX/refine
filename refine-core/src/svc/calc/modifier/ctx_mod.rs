use crate::{
    svc::calc::{ModContext, RawModId, RawModifier},
    ud::{UFitId, UItemId},
};

/// CtxModifier's identity, stored in registers to avoid storing full modifiers.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(in crate::svc::calc) struct CtxModKey {
    pub(in crate::svc::calc) rmod_id: RawModId,
    ctx: ModContext,
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
            ctx: ModContext::new(),
        }
    }
    pub(in crate::svc::calc) fn with_item(rmod: RawModifier, item_uid: UItemId) -> Self {
        Self {
            rmod,
            ctx: ModContext::with_item(item_uid),
        }
    }
    pub(in crate::svc::calc) fn with_fit(rmod: RawModifier, fit_uid: UFitId) -> Self {
        Self {
            rmod,
            ctx: ModContext::with_fit(fit_uid),
        }
    }
    pub(in crate::svc::calc) fn with_fit_item(rmod: RawModifier, fit_uid: UFitId, item_uid: UItemId) -> Self {
        Self {
            rmod,
            ctx: ModContext::with_fit_item(fit_uid, item_uid),
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
#[cfg(feature = "dev")]
mod debug {
    use super::*;
    use crate::{dbg::DebugResult, svc::calc::registers::RawModStorage, ud::UData};

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
            let Self { rmod, ctx } = self;
            if let Some(fit_uid) = ctx.get_fit_uid() {
                fit_uid.consistency_check(u_data)?;
            }
            // Item modifier is applied to is not necessarily loaded (e.g. a module projected to a
            // non-loaded ship)
            if let Some(item_uid) = ctx.get_item_uid() {
                item_uid.consistency_check(u_data, false)?;
            }
            rmod.consistency_check(u_data)?;
            Ok(())
        }
    }
}
