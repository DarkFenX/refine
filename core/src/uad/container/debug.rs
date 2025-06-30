use crate::{
    dbg::{DebugError, DebugResult},
    uad::container::EntityContainer,
    util::RSet,
};

impl<T, E> EntityContainer<T, E> {
    pub(in crate::uad) fn consistency_check(&self) -> DebugResult {
        let seen_data: RSet<_> = self.data.iter().map(|(key, _)| key).collect();
        let seen_map: RSet<_> = self.id_to_key.values().copied().collect();
        if seen_data.difference(&seen_map).next().is_some() || seen_map.difference(&seen_data).next().is_some() {
            return Err(DebugError {});
        }
        Ok(())
    }
}
