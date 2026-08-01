pub use item_iter_ranged_projs::RangedProjIter;
pub(in crate::api) use item_iter_ranged_projs::iter_ranged_projs;
pub use ranged_proj::{RangedProj, RangedProjMut};

mod item_get_ranged_proj;
mod item_iter_ranged_projs;
mod ranged_proj;
mod ranged_proj_remove;
