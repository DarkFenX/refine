use super::UadService;
use crate::sol::{
    debug::{DebugResult, check_fit_key},
    uad::Uad,
};

impl UadService {
    pub(in crate::sol::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        Ok(())
    }
}
