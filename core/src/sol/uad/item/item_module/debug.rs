use crate::sol::{
    debug::{DebugResult, check_fit_key, check_item_key},
    uad::Uad,
};

use super::UadModule;

impl UadModule {
    pub(in crate::sol::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        if let Some(charge_key) = self.get_charge_item_key() {
            check_item_key(uad, charge_key, false)?;
        }
        self.get_projs().consistency_check(uad)?;
        Ok(())
    }
}
