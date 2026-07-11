pub use mgr::SrcMgr;
pub use mgr_create_src::CreateSrcError;
pub use mgr_get_src::GetSrcError;
pub use mgr_remove_src::RemoveSrcError;
pub use src::Src;

pub mod err;
mod mgr;
mod mgr_create_src;
mod mgr_get_src;
mod mgr_remove_src;
mod src;
