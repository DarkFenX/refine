pub use fleet::{CreateFleetError, RemoveFleetError};
pub(in crate::cmd) use fleet::{FleetCreateCmdFCtxRIds, FleetRemoveCmdICtx};
pub(in crate::cmd) use sol::SolCreateCmdFCtx;

mod fleet;
mod sol;
