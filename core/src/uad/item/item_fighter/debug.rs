use itertools::Itertools;

use crate::{
    dbg::{DebugError, DebugResult, check_fit_key},
    uad::{Uad, UadFighter},
};

impl UadFighter {
    pub(in crate::uad::item) fn consistency_check(&self, uad: &Uad) -> DebugResult {
        check_fit_key(uad, self.get_fit_key())?;
        self.get_autocharges().consistency_check(uad)?;
        self.get_projs().consistency_check(uad)?;
        // If fighter has autocharges, make sure projections on them match
        if !self.get_autocharges().is_empty() {
            let fighter_projs = self.get_projs().iter().sorted().collect_vec();
            for &autocharge_key in self.get_autocharges().values() {
                let autocharge_projs = uad
                    .items
                    .get(autocharge_key)
                    .get_autocharge()
                    .unwrap()
                    .get_projs()
                    .iter()
                    .sorted()
                    .collect_vec();
                if fighter_projs != autocharge_projs {
                    return Err(DebugError {});
                }
            }
        }
        Ok(())
    }
}
