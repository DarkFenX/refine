// This module contains entities which bridge synchronous core lib and async HTTP interface
// together.

pub(crate) use src_mgr::HSrcMgr;
pub(crate) use ss::{HGuardedSs, HSolarSystem};
pub(crate) use ss_mgr::HSsMgr;

mod src_mgr;
mod ss;
mod ss_mgr;
