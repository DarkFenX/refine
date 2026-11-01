//! Rollback functionality works via transaction-alike interface.
//!
//! Internally, when transaction is initiated, it stores change log. When requested, the changes are
//! rolled back in LIFO order, to make sure solar system is roughly in the same state as it was
//! before the change. The pre-rollback state is not exact: it is guaranteed to keep solar system's
//! user settings, external IDs, and internal IDs of user entities (like fleets, fits, or items).

pub(crate) use log::UndoLog;
pub(crate) use savepoint::Savepoint;
pub(crate) use step::UndoStep;
pub use tx::SolTx;

mod log;
mod savepoint;
mod step;
mod tx;
