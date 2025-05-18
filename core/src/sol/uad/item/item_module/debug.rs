use itertools::Itertools;

use super::UadModule;
use crate::sol::{
    debug::{DebugError, DebugResult, check_fit_key, check_item_key},
    uad::Uad,
};

impl UadModule {
    pub(in crate::sol::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        if let Some(charge_key) = self.get_charge_item_key() {
            check_item_key(uad, charge_key, false)?;
        }
        self.get_projs().consistency_check(uad)?;
        // If module has a charge, make sure projections on them match
        if let Some(charge_key) = self.get_charge_item_key() {
            let module_projs = self.get_projs().iter().map(|(k, v)| (*k, *v)).sorted().collect_vec();
            let charge_projs = uad
                .items
                .get(charge_key)
                .get_charge()
                .unwrap()
                .get_projs()
                .iter()
                .map(|(k, v)| (*k, *v))
                .sorted()
                .collect_vec();
            if module_projs != charge_projs {
                return Err(DebugError {});
            }
        }
        Ok(())
    }
}
