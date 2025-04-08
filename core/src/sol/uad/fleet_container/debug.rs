use crate::{
    sol::debug::{DebugError, DebugResult},
    util::RSet,
};

use super::Fleets;

impl Fleets {
    pub(in crate::sol) fn debug_consistency_check(&self) -> DebugResult {
        let seen_data: RSet<_> = self.data.iter().map(|(fit_key, _)| fit_key).collect();
        let seen_map: RSet<_> = self.id_to_key.values().copied().collect();
        if seen_data.difference(&seen_map).next().is_some() || seen_map.difference(&seen_data).next().is_some() {
            return Err(DebugError {});
        }
        Ok(())
    }
}
