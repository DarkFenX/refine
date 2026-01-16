use crate::{
    dbg::DebugResult,
    svc::calc::{CtxModifier, ModContext},
    ud::UData,
};

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
