use crate::{
    dbg::{DebugResult, check_fit_key, check_item_key},
    uad::{Uad, UadAutocharge},
};

impl UadAutocharge {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        // All autocharges are supposed to be loaded
        check_item_key(uad, self.get_cont_item_key(), true)?;
        self.get_projs().consistency_check(uad)?;
        Ok(())
    }
}
