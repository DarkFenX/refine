pub use add_proj::AddFighterProjError;
pub use change_proj::ChangeFighterProjError;
pub use fighter::{Fighter, FighterMut};
pub use get::GetFighterError;
pub use remove_proj::RemoveFighterProjError;
pub use set_count_override::SetFighterCountOverrideError;

mod add;
mod add_proj;
mod change_proj;
mod fighter;
mod fit_iter;
mod get;
mod iter_autocharges;
mod remove;
mod remove_count_override;
mod remove_proj;
mod set_count_override;
mod set_state;
