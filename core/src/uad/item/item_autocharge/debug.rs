use crate::{
    dbg::{DebugResult, check_a_effect_id, check_fit_key, check_item_key},
    uad::{Uad, UadAutocharge},
};

impl UadAutocharge {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        if let Some(reffs) = self.get_reffs() {
            for a_effect_id in reffs.iter() {
                check_a_effect_id(uad, a_effect_id)?;
            }
        }
        check_fit_key(uad, self.get_fit_key())?;
        // All autocharges are supposed to be loaded
        check_item_key(uad, self.get_cont_key(), true)?;
        self.get_projs().consistency_check(uad)?;
        Ok(())
    }
}
