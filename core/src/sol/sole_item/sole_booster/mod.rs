pub use sole_add_booster::AddBoosterError;
pub use sole_get_booster::GetBoosterInfoError;
pub use sole_get_fit_boosters::GetFitBoosterInfosError;
pub use sole_remove_booster::RemoveBoosterError;
pub use sole_set_booster_side_effect_state::SetBoosterSideEffectStateError;
pub use sole_set_booster_state::SetBoosterStateError;

mod sole_add_booster;
mod sole_get_booster;
mod sole_get_fit_boosters;
mod sole_remove_booster;
mod sole_set_booster_side_effect_state;
mod sole_set_booster_state;
