pub use item_add_proj::ProjAddError;
pub use item_get_proj::ProjGetError;
use item_iter_projs::iter_projectee_uids;
pub use non_ranged::{Proj, ProjIter};
pub use ranged::{RangedProj, RangedProjIter};

mod item_add_proj;
mod item_get_proj;
mod item_iter_projs;
mod non_ranged;
mod proj_remove;
mod ranged;
