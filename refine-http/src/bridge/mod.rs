// This module contains entities which bridge synchronous core lib and async HTTP interface
// together.

pub(crate) use sol::HSolarSystem;
pub(crate) use sol_mgr::HSolMgr;

mod sol;
mod sol_mgr;
