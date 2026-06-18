use std::hash::Hash;

use crate::{dbg::DebugResult, ud::container::UEntityContainer, util::RSet};

impl<T, ExtId, IntId, Err> UEntityContainer<T, ExtId, IntId, Err>
where
    ExtId: Eq + Hash,
{
    pub(in crate::ud) fn consistency_check(&self) -> DebugResult {
        let seen_data: RSet<_> = self.data.iter().map(|(slab_key, _)| slab_key).collect();
        let seen_map: RSet<_> = self.ext_id_to_index.values().copied().collect();
        if seen_data.difference(&seen_map).next().is_some() || seen_map.difference(&seen_data).next().is_some() {
            return Err(Default::default());
        }
        Ok(())
    }
}
