use std::collections::hash_map::Entry;

use parking_lot::RwLock;

use super::inner::{SolInner, SolInnerGuarded};
use crate::{SolId, SrcAlias, util::RMap};

/// Map of solar systems by their IDs.
///
/// Solar systems are closed when removed: their core is taken out for the remover to free, and
/// requests which were waiting for their lock do not work on them. Removal requires holding the
/// solar system's lock.
pub(crate) struct SolMapGuarded {
    inner: RwLock<RMap<SolId, SolInnerGuarded>>,
}
impl SolMapGuarded {
    pub(crate) fn new() -> Self {
        Self {
            inner: RwLock::new(RMap::default()),
        }
    }
    pub(crate) fn get(&self, id: &SolId) -> Option<SolInnerGuarded> {
        self.inner.read().get(id).cloned()
    }
    pub(crate) fn get_all(&self) -> Vec<(SolId, SolInnerGuarded)> {
        self.inner
            .read()
            .iter()
            .map(|(id, inner_sol)| (*id, inner_sol.clone()))
            .collect()
    }
    pub(crate) fn add(&self, src_alias: SrcAlias, core_sol: rc::SolOwned) -> SolInnerGuarded {
        let mut id = SolId::new();
        let mut map = self.inner.write();
        loop {
            match map.entry(id) {
                Entry::Vacant(entry) => {
                    let inner_sol = SolInnerGuarded::new(id, src_alias, core_sol);
                    entry.insert(inner_sol.clone());
                    return inner_sol;
                }
                Entry::Occupied(..) => {
                    id = SolId::new();
                    continue;
                }
            }
        }
    }
    /// Returns the core solar system, or nothing if it was removed already.
    pub(crate) fn remove(&self, inner_sol: &mut SolInner) -> Option<rc::SolOwned> {
        let core_sol = inner_sol.close();
        self.inner.write().remove(&inner_sol.get_id());
        core_sol
    }
    /// Returns core solar systems of the removed ones.
    pub(crate) fn remove_if(&self, mut predicate: impl FnMut(&SolInner) -> bool) -> Vec<rc::SolOwned> {
        // Detection & removal under the same map lock, so that solar systems are not taken to do
        // something and then removed. Core solar systems go to the caller to free, so that they
        // aren't dropped while the map lock is held
        let mut core_sols = Vec::new();
        self.inner.write().retain(|_, inner_sol| match inner_sol.try_lock() {
            Ok(mut sol) => match predicate(&sol) {
                true => {
                    core_sols.extend(sol.close());
                    false
                }
                false => true,
            },
            // If it's locked - it means it's being worked on, we don't touch that
            Err(..) => true,
        });
        core_sols
    }
}
