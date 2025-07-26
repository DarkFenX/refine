use crate::{
    dbg::{DebugResult, check_effect_key, check_fit_key, check_item_key},
    ud::{UAutocharge, UData},
};

impl UAutocharge {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        if let Some(reffs) = self.get_reffs() {
            for &effect_key in reffs.iter() {
                check_effect_key(u_data, effect_key)?;
            }
        }
        check_fit_key(u_data, self.get_fit_key())?;
        // All autocharges are supposed to be loaded
        check_item_key(u_data, self.get_cont_item_key(), true)?;
        self.get_projs().consistency_check(u_data)?;
        Ok(())
    }
}
