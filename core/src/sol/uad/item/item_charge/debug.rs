use super::UadCharge;
use crate::sol::{
    debug::{DebugResult, check_fit_key, check_item_key},
    uad::Uad,
};

impl UadCharge {
    pub(in crate::sol::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        check_item_key(uad, self.get_cont_item_key(), false)?;
        self.get_projs().consistency_check(uad)?;
        Ok(())
    }
}
