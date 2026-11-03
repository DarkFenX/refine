use std::ops::{Deref, DerefMut};

use crate::{Core, Sol, sol::Savepoint};

/// Transaction on a solar system.
///
/// Derefs to the solar system's interface. Commit to keep changes, roll back or drop to revert
/// them.
///
/// A guard dropped while panicking neither rolls back nor commits, since rolling back half-done
/// changes could panic again; the solar system has to be discarded then.
pub struct SolTx<'a> {
    sol: &'a mut Core,
    // Taken on commit and rollback
    savepoint: Option<Savepoint>,
}
impl<'a> SolTx<'a> {
    pub(crate) fn new(sol: &'a mut Core, savepoint: Savepoint) -> Self {
        Self {
            sol,
            savepoint: Some(savepoint),
        }
    }
    pub fn commit(mut self) {
        self.savepoint = None;
        self.sol.inner_mut().internal_tx_commit();
    }
    pub fn rollback(mut self) {
        let savepoint = self.savepoint.take().unwrap();
        self.sol.inner_mut().internal_tx_rollback(savepoint);
    }
}
impl Deref for SolTx<'_> {
    type Target = Sol;

    fn deref(&self) -> &Sol {
        self.sol
    }
}
impl DerefMut for SolTx<'_> {
    fn deref_mut(&mut self) -> &mut Sol {
        self.sol
    }
}
impl Drop for SolTx<'_> {
    fn drop(&mut self) {
        if let Some(savepoint) = self.savepoint.take()
            && !std::thread::panicking()
        {
            self.sol.inner_mut().internal_tx_rollback(savepoint);
        }
    }
}
