use itertools::Itertools;

use crate::{
    dbg::{DebugError, DebugResult, check_fit_key, check_item_key},
    uad::{Uad, UadModule},
};

impl UadModule {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        if let Some(charge_key) = self.get_charge_key() {
            check_item_key(uad, charge_key, false)?;
        }
        self.get_projs().consistency_check(uad)?;
        // If module has a charge, make sure projections on them match
        if let Some(charge_key) = self.get_charge_key() {
            let module_projs = self.get_projs().iter().sorted().collect_vec();
            let charge_projs = uad
                .items
                .get(charge_key)
                .get_charge()
                .unwrap()
                .get_projs()
                .iter()
                .sorted()
                .collect_vec();
            if module_projs != charge_projs {
                return Err(DebugError {});
            }
        }
        Ok(())
    }
}
