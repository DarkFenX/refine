pub use abil::{Ability, AbilityIter, AbilityMut, GetAbilityError};
pub use fighter::{Fighter, FighterMut};
pub use sol_get_fighter::GetFighterError;

mod abil;
mod fighter;
mod fighter_iter_autocharges;
mod fighter_remove;
mod fighter_set_coordinates;
mod fighter_set_count_override;
mod fighter_set_state;
mod fighter_set_type_id;
mod fit_add_fighter;
mod fit_iter_fighters;
mod ranged_proj;
mod sol_get_fighter;
mod util_add_remove;
