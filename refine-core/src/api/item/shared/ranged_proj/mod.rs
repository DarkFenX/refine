pub use add_proj::AddProjError;
pub use get_ranged_proj::GetRangedProjError;
pub use iter_ranged_projs::RangedProjIter;
pub(in crate::api) use iter_ranged_projs::iter_ranged_projs;
pub use ranged_proj::{RangedProj, RangedProjMut};

mod add_proj;
mod get_ranged_proj;
mod iter_ranged_projs;
mod ranged_proj;
mod ranged_proj_remove;
