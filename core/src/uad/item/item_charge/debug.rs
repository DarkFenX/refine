use crate::{
    dbg::{DebugResult, check_fit_key, check_item_key},
    uad::{Uad, UadCharge},
};

impl UadCharge {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        check_item_key(uad, self.get_cont_key(), false)?;
        self.get_projs().consistency_check(uad)?;
        Ok(())
    }
}
