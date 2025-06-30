use crate::{
    dbg::{DebugResult, check_fit_key},
    uad::{Uad, UadService},
};

impl UadService {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        Ok(())
    }
}
