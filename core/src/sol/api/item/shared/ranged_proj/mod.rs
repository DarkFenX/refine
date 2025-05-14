pub use add_ranged_proj::AddRangedProjError;
pub use get_ranged_proj::GetRangedProjError;
pub use iter_ranged_projs::RangedProjIter;
pub(in crate::sol::api) use iter_ranged_projs::iter_ranged_projs;
pub use ranged_proj::{RangedProj, RangedProjMut};

mod add_ranged_proj;
mod get_ranged_proj;
mod iter_ranged_projs;
mod ranged_proj;
mod ranged_proj_remove;
mod ranged_proj_set_range;
