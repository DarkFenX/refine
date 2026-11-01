use std::ops::{Deref, DerefMut};

use crate::sol::{SolarSystem, UndoSavepoint};

/// Transaction on a solar system.
///
/// Derefs to the solar system. Commit to keep changes, roll back or drop to revert them. Only one
/// transaction can be open on a solar system at a time.
///
/// Restrictions:
/// - the solar system must not be replaced or cloned through the guard;
/// - a guard dropped while panicking neither rolls back nor commits, since rolling back half-done
///   changes could panic again; the solar system has to be discarded then.
pub struct SolTx<'a> {
    sol: &'a mut SolarSystem,
    // Taken on commit and rollback
    savepoint: Option<UndoSavepoint>,
}
impl SolTx<'_> {
    pub fn commit(mut self) {
        self.savepoint = None;
        self.sol.internal_commit();
    }
    pub fn rollback(mut self) {
        let savepoint = self.savepoint.take().unwrap();
        self.sol.internal_rollback(savepoint);
    }
}
impl Deref for SolTx<'_> {
    type Target = SolarSystem;

    fn deref(&self) -> &SolarSystem {
        self.sol
    }
}
impl DerefMut for SolTx<'_> {
    fn deref_mut(&mut self) -> &mut SolarSystem {
        self.sol
    }
}
impl Drop for SolTx<'_> {
    fn drop(&mut self) {
        if let Some(savepoint) = self.savepoint.take()
            && !std::thread::panicking()
        {
            self.sol.internal_rollback(savepoint);
        }
    }
}

impl SolarSystem {
    pub fn begin_tx(&mut self) -> Result<SolTx<'_>, TxBeginError> {
        if self.undo.is_recording() {
            return Err(TxBeginError::InTransaction);
        }
        let savepoint = self.internal_savepoint();
        Ok(SolTx {
            sol: self,
            savepoint: Some(savepoint),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TxBeginError {
    #[error("transaction cannot be started within a transaction")]
    InTransaction,
}
