use crate::{
    dbg::{DebugError, DebugResult, check_effect_key, check_fit_key, check_item_key},
    ud::{UCharge, UData},
};

impl UCharge {
    pub(in crate::ud::item) fn consistency_check(&self, u_data: &UData) -> DebugResult {
        self.base.consistency_check(u_data)?;
        check_fit_key(u_data, self.get_fit_key())?;
        check_item_key(u_data, self.get_cont_item_key(), false)?;
        match self.get_cont_effect_key() {
            // If effect key is defined, effect key should exist in source and on parent item
            Some(effect_key) => {
                check_item_key(u_data, self.get_cont_item_key(), true)?;
                check_effect_key(u_data, effect_key)?;
                if !u_data
                    .items
                    .get(self.get_cont_item_key())
                    .get_effect_datas()
                    .unwrap()
                    .contains_key(&effect_key)
                {
                    return Err(DebugError {});
                }
            }
            // If effect key is not defined, parent item might be not loaded
            None => check_item_key(u_data, self.get_cont_item_key(), false)?,
        }
        self.get_projs().consistency_check(u_data)?;
        Ok(())
    }
}
